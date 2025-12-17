#[cfg(unix)]
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexEntryStage {
    Normal = 0,
    Base = 1,
    Ours = 2,
    Theirs = 3,
}

impl Default for IndexEntryStage {
    fn default() -> Self {
        IndexEntryStage::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexEntryFlags {
    pub stage: IndexEntryStage,
    pub assume_valid: bool,
    pub name_len: u16, // 0...=0xFFFF (4095)
}

#[cfg(unix)]
impl IndexEntryFlags {
    pub fn from_path(path: &Path, assume_valid: bool, stage: IndexEntryStage) -> Self {
        let name_len = path.as_os_str().len();

        IndexEntryFlags {
            stage: stage,
            assume_valid: assume_valid,
            name_len: name_len.min(0xFFF) as u16,
        }
    }
}

impl Into<u16> for IndexEntryFlags {
    fn into(self) -> u16 {
        let mut value: u16 = 0;

        // 1-bit assume-valid (bit 15)
        if self.assume_valid {
            value |= 1 << 15;
        }

        // 1-bit extended flag (bit 14) - always 0 for version 2

        // 2-bit stage (bits 12-13)
        value |= (self.stage as u16 & 0b11) << 12;

        // 12-bit name length (bits 0-11)
        value |= self.name_len;

        value
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IndexEntryPermissions {
    Perm644,
    Perm755,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum IndexEntryObjectType {
    RegularFile(IndexEntryPermissions),
    Symbolic,
    Gitlink,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IndexEntry {
    ctimes: u32,  // Last time a file's metadata changed
    ctimens: u32, // nano secs
    mtimes: u32,  // Last time a file's data changed
    mtimens: u32, // nano secs
    dev: u32,     // Device ID (stat(2))
    ino: u32,     // File Serial number (stat(2))
    object_type: IndexEntryObjectType,
    flags: IndexEntryFlags,
    uid: u32,       // User Id of the file (stat(2))
    gid: u32,       // Group Id of the file (stat(2))
    file_size: u32, // On-disk size (stat(2))
    //object_name: [u8; 20],
    file_path: PathBuf,
}

#[cfg(unix)]
impl IndexEntryObjectType {
    pub fn from_path(path: &Path, meta: &fs::Metadata) -> Option<Self> {
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

    pub fn mode(&self) -> u32 {
        match self {
            IndexEntryObjectType::RegularFile(perms) => {
                let obj_bits: u32 = 0b1000 << 12;
                let unused_bits: u32 = 0b000 << 9;
                let perm_bits = match perms {
                    IndexEntryPermissions::Perm644 => 0o644,
                    IndexEntryPermissions::Perm755 => 0o755,
                };
                obj_bits | unused_bits | perm_bits
            }
            IndexEntryObjectType::Symbolic => 0b1010 << 12,
            IndexEntryObjectType::Gitlink => 0b1110 << 12,
        }
    }
}

#[cfg(unix)]
impl IndexEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        use std::{fs, os::unix::fs::MetadataExt};

        let metadata = fs::symlink_metadata(path).expect("Failed to get path metadata");

        let object_type = IndexEntryObjectType::from_path(path, &metadata);

        if object_type.is_none() {
            return None;
        }

        let object_type = object_type.unwrap();
        let ctime = metadata.ctime() as u32;
        let ctime_nano = metadata.ctime_nsec() as u32;
        let mtime = metadata.mtime() as u32;
        let mtime_nano = metadata.mtime_nsec() as u32;
        let dev = metadata.dev() as u32;
        let ino = metadata.ino() as u32;
        let uid = metadata.uid();
        let gid = metadata.gid();
        let file_size = metadata.size() as u32;

        let flags = IndexEntryFlags::from_path(path, false, IndexEntryStage::default());

        Some(IndexEntry {
            ctimes: ctime,
            ctimens: ctime_nano,
            mtimes: mtime,
            mtimens: mtime_nano,
            dev: dev,
            ino: ino,
            uid: uid,
            gid: gid,
            file_size: file_size,
            file_path: path.to_path_buf(),
            object_type: object_type,
            flags: flags,
        })
    }

    pub fn mode(&self) -> u32 {
        self.object_type.mode()
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {

    use super::*;
    use std::os::unix::fs::{PermissionsExt, symlink};
    use tempfile::tempdir;

    #[test]
    fn test_object_type_from_path_regular_file_644() {
        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("target");

        fs::write(&target_path, b"hello").unwrap();
        fs::set_permissions(&target_path, fs::Permissions::from_mode(0o644)).unwrap();

        let meta = fs::symlink_metadata(&target_path).unwrap();
        let result = IndexEntryObjectType::from_path(&target_path, &meta);

        assert!(result.is_some());
        assert!(matches!(
            result.unwrap(),
            IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644)
        ));
    }

    #[test]
    fn test_object_type_from_path_regular_file_755() {
        let temp_dir = tempdir().unwrap();
        let target_path = temp_dir.path().join("target");

        fs::write(&target_path, b"hello").unwrap();
        fs::set_permissions(&target_path, fs::Permissions::from_mode(0o755)).unwrap();

        let meta = fs::symlink_metadata(&target_path).unwrap();
        let result = IndexEntryObjectType::from_path(&target_path, &meta);

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
        let result = IndexEntryObjectType::from_path(&repo_path, &meta);

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
        let result = IndexEntryObjectType::from_path(&link_path, &meta);

        assert!(result.is_some());
        assert!(matches!(result.unwrap(), IndexEntryObjectType::Symbolic));
    }

    #[test]
    fn test_normal_directory() {
        let temp_dir = tempdir().unwrap();
        let normal_dir = temp_dir.path().join("normal");

        fs::create_dir(&normal_dir).unwrap();

        let meta = fs::symlink_metadata(&normal_dir).unwrap();
        let result = IndexEntryObjectType::from_path(&normal_dir, &meta);

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
        let result = IndexEntryObjectType::from_path(&other_file, &meta);

        assert!(result.is_some());
        assert!(matches!(
            result.unwrap(),
            IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644)
        ));
    }

    #[test]
    fn test_regular_file_mode() {
        let expected_perm644: u32 = (0b1000 << 12) | 0o644;
        let expected_perm755: u32 = (0b1000 << 12) | 0o755;
        let obj_perm644 = IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644);
        let obj_perm755 = IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm755);

        assert_eq!(obj_perm644.mode(), expected_perm644);
        assert_eq!(obj_perm755.mode(), expected_perm755);
    }

    #[test]
    fn test_symbolic_link_mode() {
        let obj_sym = IndexEntryObjectType::Symbolic;
        let expected_sym: u32 = 0b1010 << 12;

        assert_eq!(obj_sym.mode(), expected_sym);
    }

    #[test]
    fn test_gitlink_mode() {
        let obj_gitlink = IndexEntryObjectType::Gitlink;
        let expected_gitlink: u32 = 0b1110 << 12;

        assert_eq!(obj_gitlink.mode(), expected_gitlink);
    }

    #[test]
    fn test_entry_flags_normal_into_u16() {
        let path = Path::new("hello.txt");
        let flags_normal = IndexEntryFlags::from_path(path, false, IndexEntryStage::Normal);

        let mask = 0x0FFF as u16;
        let expected_name_length = 9;
        let expected_stage_bits = 0;
        let mut exepcted_assume_valid_bits = 0;
        let expected_extended_flag = 0; // 0 for version 2

        let mut flags_u16: u16 = flags_normal.into();

        // lower 12 bits  = name length (9)
        assert_eq!(flags_u16 & mask, expected_name_length);

        // stage bits
        assert_eq!((flags_u16 >> 12) & 0b11, expected_stage_bits);

        // assume valid bit
        assert_eq!((flags_u16 >> 15) & 1, exepcted_assume_valid_bits);

        // bit 14
        assert_eq!((flags_u16 >> 14) & 1, expected_extended_flag);

        let flags_assume_true = IndexEntryFlags::from_path(path, true, IndexEntryStage::Normal);
        exepcted_assume_valid_bits = 1;
        flags_u16 = flags_assume_true.into();

        // lower 12 bits  = name length (9)
        assert_eq!(flags_u16 & mask, expected_name_length);

        // stage bits
        assert_eq!((flags_u16 >> 12) & 0b11, expected_stage_bits);

        // assume valid bit
        assert_eq!((flags_u16 >> 15) & 1, exepcted_assume_valid_bits);

        // bit 14
        assert_eq!((flags_u16 >> 14) & 1, expected_extended_flag);
    }

    #[test]
    fn test_stage_bits() {
        let path = Path::new("test");
        let expected_base_stage_bit = 1;
        let expected_ours_stage_bit = 2;
        let expected_theirs_stage_bit = 3;

        let base_stage_flags = IndexEntryFlags::from_path(path, false, IndexEntryStage::Base);
        let ours_stage_flags = IndexEntryFlags::from_path(path, false, IndexEntryStage::Ours);
        let theirs_stage_flags = IndexEntryFlags::from_path(path, false, IndexEntryStage::Theirs);

        let base_stage_flags_u16: u16 = base_stage_flags.into();
        let ours_stage_flags_u16: u16 = ours_stage_flags.into();
        let theirs_stage_flags_u16: u16 = theirs_stage_flags.into();

        assert_eq!((base_stage_flags_u16 >> 12) & 0b11, expected_base_stage_bit);
        assert_eq!((ours_stage_flags_u16 >> 12) & 0b11, expected_ours_stage_bit);
        assert_eq!(
            (theirs_stage_flags_u16 >> 12) & 0b11,
            expected_theirs_stage_bit
        );
    }

    #[test]
    fn test_truncated_name_length() {
        // path length > 4095
        let long_path_name = "a".repeat(5000);
        let path = Path::new(&long_path_name);
        let expected_name_len_bits = 0x0FFF;

        let long_name_flags = IndexEntryFlags::from_path(path, false, IndexEntryStage::Normal);
        let long_name_flags_u16: u16 = long_name_flags.into();
        assert_eq!(long_name_flags_u16 & 0x0FFF, expected_name_len_bits);
    }
}
