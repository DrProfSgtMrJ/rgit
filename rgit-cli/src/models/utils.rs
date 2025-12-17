use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

#[cfg(unix)]
use crate::models::{IndexEntryObjectType, IndexEntryPermissions, IndexEntryStage};

pub fn create_file(dir_path: &Path, file_name: &str, content: &[u8]) -> Result<(), std::io::Error> {
    let full_path = dir_path.join(file_name);

    let mut file = File::create(full_path)?;
    file.write_all(content)?;

    Ok(())
}

pub fn create_dir(full_path: &Path, recursive: bool) -> Result<(), std::io::Error> {
    if recursive {
        fs::create_dir_all(full_path)?;
    } else {
        fs::create_dir(full_path)?;
    }

    Ok(())
}

pub fn remove_dir(full_path: &Path, recursive: bool) -> Result<(), std::io::Error> {
    if recursive {
        fs::remove_dir_all(full_path)?;
    } else {
        fs::remove_dir(full_path)?;
    }

    Ok(())
}

#[cfg(unix)]
pub fn compute_object_type(path: &Path, meta: &fs::Metadata) -> Option<IndexEntryObjectType> {
    use std::os::unix::fs::MetadataExt;
    if meta.file_type().is_symlink() {
        return Some(IndexEntryObjectType::Symbolic);
    } else if meta.is_file() {
        let is_executable = meta.mode() & 0o100 != 0;
        let perm = if is_executable {
            IndexEntryPermissions::Perm755
        } else {
            IndexEntryPermissions::Perm644
        };
        return Some(IndexEntryObjectType::RegularFile(perm));
    } else if meta.is_dir() {
        let rgit_dir = path.join(".rgit");
        if rgit_dir.exists() {
            return Some(IndexEntryObjectType::Gitlink);
        } else {
            return None;
        }
    }

    None
}

#[cfg(unix)]
pub fn compute_flags(assume_valid: bool, stage: IndexEntryStage, file_path: &Path) -> u16 {
    let mut value: u16 = 0;

    // 1-bit assume-valid (bit 15)
    if assume_valid {
        value |= 1 << 15;
    }

    // 1-bit extended flag (bit 14) - always 0 for version 2

    // 2-bit stage (bits 12-13)
    value |= (stage as u16 & 0b11) << 12;

    // 12-bit name length (bits 0-11)
    let name_len = file_path.as_os_str().len();
    let len_field = name_len.min(0xFFF) as u16;

    value |= len_field;

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{PermissionsExt, symlink};
    use tempfile::tempdir;

    #[test]
    fn test_create_file_with_content() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path();
        let file_name = "test.txt";
        let content = b"fake content";

        let result = create_file(tmp_dir_path, file_name, content);

        assert!(result.is_ok());

        let created_path = tmp_dir_path.join(file_name);
        assert!(created_path.exists());

        let read = fs::read(created_path).unwrap();
        assert_eq!(read, content);
    }

    #[test]
    fn create_dir_non_recursive() {
        let tmp_dir = tempdir().unwrap();
        let new_dir = tmp_dir.path().join("new_dir");

        let result = create_dir(&new_dir, false);
        assert!(result.is_ok());

        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn create_dir_recursive() {
        let tmp_dir = tempdir().unwrap();
        let nested = tmp_dir.path().join("a/b/c");

        let result = create_dir(&nested, true);

        assert!(result.is_ok());

        assert!(nested.exists());
        assert!(nested.is_dir());

        let semi_nested = tmp_dir.path().join("a/b");
        assert!(semi_nested.exists());
        assert!(semi_nested.is_dir());
    }

    #[test]
    fn remove_dir_non_recursive() {
        let tmp_dir = tempdir().unwrap();

        assert!(tmp_dir.path().exists());

        let result = remove_dir(tmp_dir.path(), false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_regular_file_644() {
        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("target");

        fs::write(&target_path, b"hello").unwrap();
        fs::set_permissions(&target_path, fs::Permissions::from_mode(0o644)).unwrap();

        let meta = fs::symlink_metadata(&target_path).unwrap();
        let result = compute_object_type(&target_path, &meta);

        assert!(result.is_some());
        assert!(matches!(
            result.unwrap(),
            IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644)
        ));
    }

    #[test]
    fn test_regular_file_755() {
        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("target");

        fs::write(&target_path, b"hello").unwrap();
        fs::set_permissions(&target_path, fs::Permissions::from_mode(0o755)).unwrap();

        let meta = fs::symlink_metadata(&target_path).unwrap();
        let result = compute_object_type(&target_path, &meta);

        assert!(result.is_some());
        assert!(matches!(
            result.unwrap(),
            IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm755)
        ));
    }

    #[test]
    fn test_directory_with_rgit_is_gitlink() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");
        let rgit_path = repo_path.join(".rgit");

        fs::create_dir(&repo_path).unwrap();
        fs::create_dir(&rgit_path).unwrap();

        let meta = fs::symlink_metadata(&repo_path).unwrap();
        let result = compute_object_type(&repo_path, &meta);

        assert!(result.is_some());
        assert!(matches!(result.unwrap(), IndexEntryObjectType::Gitlink));
    }

    #[test]
    fn test_symbolic_link() {
        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("target");
        let link_path = temp_dir.path().join("link");

        fs::write(&target_path, b"hello").unwrap();
        symlink(&target_path, &link_path).unwrap();

        let meta = fs::symlink_metadata(&link_path).unwrap();
        let result = compute_object_type(&link_path, &meta);

        assert!(result.is_some());
        assert!(matches!(result.unwrap(), IndexEntryObjectType::Symbolic));
    }

    #[test]
    fn test_normal_directory() {
        let temp_dir = tempdir().unwrap();
        let normal_dir = temp_dir.path().join("normal");

        fs::create_dir(&normal_dir).unwrap();

        let meta = fs::symlink_metadata(&normal_dir).unwrap();
        let result = compute_object_type(&normal_dir, &meta);

        assert!(result.is_none());
    }

    #[test]
    fn test_other_perms_files() {
        let temp_dir = tempdir().unwrap();
        let other_file = temp_dir.path().join("other");

        fs::write(&other_file, b"hello").unwrap();
        // group executable only - owner exec bit NOT set
        fs::set_permissions(&other_file, fs::Permissions::from_mode(0o650)).unwrap();

        let meta = fs::symlink_metadata(&other_file).unwrap();
        let result = compute_object_type(&other_file, &meta);

        assert!(result.is_some());
        assert!(matches!(
            result.unwrap(),
            IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644)
        ));
    }

    #[test]
    fn test_compute_flags_normal() {
        let path = Path::new("hello.txt"); // len = 9
        let flags_assume_false = compute_flags(false, IndexEntryStage::Normal, path);
        let mask = 0x0FFF as u16;
        let expected_name_length = 9;
        let expected_stage_bits = 0;
        let mut exepcted_assume_valid_bits = 0;
        let expected_extended_flag = 0; // 0 for version 2

        // lower 12 bits  = name length (9)
        assert_eq!(flags_assume_false & mask, expected_name_length);

        // stage bits
        assert_eq!((flags_assume_false >> 12) & 0b11, expected_stage_bits);

        // assume valid bit
        assert_eq!((flags_assume_false >> 15) & 1, exepcted_assume_valid_bits);

        // bit 14
        assert_eq!((flags_assume_false >> 14) & 1, expected_extended_flag);

        let flags_assume_true = compute_flags(true, IndexEntryStage::Normal, path);
        exepcted_assume_valid_bits = 1;

        // lower 12 bits  = name length (9)
        assert_eq!(flags_assume_true & mask, expected_name_length);

        // stage bits
        assert_eq!((flags_assume_true >> 12) & 0b11, expected_stage_bits);

        // assume valid bit
        assert_eq!((flags_assume_true >> 15) & 1, exepcted_assume_valid_bits);

        // bit 14
        assert_eq!((flags_assume_true >> 14) & 1, expected_extended_flag);
    }

    #[test]
    fn test_stage_bits() {
        let path = Path::new("test");
        let expected_base_stage_bit = 1;
        let expected_ours_stage_bit = 2;
        let expected_theirs_stage_bit = 3;

        let base_stage_flags = compute_flags(false, IndexEntryStage::Base, path);
        let ours_stage_flags = compute_flags(false, IndexEntryStage::Ours, path);
        let theirs_stage_flags = compute_flags(false, IndexEntryStage::Theirs, path);

        assert_eq!((base_stage_flags >> 12) & 0b11, expected_base_stage_bit);
        assert_eq!((ours_stage_flags >> 12) & 0b11, expected_ours_stage_bit);
        assert_eq!((theirs_stage_flags >> 12) & 0b11, expected_theirs_stage_bit);
    }

    #[test]
    fn test_truncated_name_length() {
        // path length > 4095
        let long_path_name = "a".repeat(5000);
        let path = Path::new(&long_path_name);
        let expected_name_len_bits = 0x0FFF;

        let long_name_flags = compute_flags(false, IndexEntryStage::Normal, path);

        assert_eq!(long_name_flags & 0x0FFF, expected_name_len_bits);
    }
}
