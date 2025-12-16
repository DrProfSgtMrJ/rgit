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

#[cfg(unix)]
pub fn compute_object_type(path: &Path, meta: &fs::Metadata) -> IndexEntryObjectType {
    use std::os::unix::fs::MetadataExt;
    if meta.file_type().is_symlink() {
        return IndexEntryObjectType::Symbolic;
    } else if meta.is_file() {
        let perms = meta.mode() & 0o777; // lower 9 bits
        let git_perm = match perms {
            0o644 => IndexEntryPermissions::Perm644,
            0o755 => IndexEntryPermissions::Perm755,
            _ => IndexEntryPermissions::Perm644,
        };
        return IndexEntryObjectType::RegularFile(git_perm);
    } else if meta.is_dir() {
        let rgit_dir = path.join(".rgit");
        if rgit_dir.exists() {
            return IndexEntryObjectType::Gitlink;
        } else {
            return IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644);
        }
    }

    IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644)
}

#[cfg(unix)]
pub fn compute_mode(object_type: &IndexEntryObjectType) -> u32 {
    match object_type {
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

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IndexEntry {
    ctimes: u32,  // Last time a file's metadata changed
    ctimens: u32, // nano secs
    mtimes: u32,  // Last time a file's data changed
    mtimens: u32, // nano secs
    dev: u32,     // Device ID (stat(2))
    ino: u32,     // File Serial number (stat(2))
    object_type: IndexEntryObjectType,
    mode: u32,
    flags: u16,
    stage: IndexEntryStage,
    uid: u32,       // User Id of the file (stat(2))
    gid: u32,       // Group Id of the file (stat(2))
    file_size: u32, // On-disk size (stat(2))
    assume_valid: bool,
    //object_name: [u8; 20],
    file_path: PathBuf,
}

#[cfg(unix)]
impl From<&Path> for IndexEntry {
    fn from(path: &Path) -> Self {
        use std::{fs, os::unix::fs::MetadataExt};

        let metadata = fs::symlink_metadata(path).expect("Failed to get path metadata");

        let object_type = compute_object_type(path, &metadata);
        let stage = IndexEntryStage::default();
        let ctime = metadata.ctime() as u32;
        let ctime_nano = metadata.ctime_nsec() as u32;
        let mtime = metadata.mtime() as u32;
        let mtime_nano = metadata.mtime_nsec() as u32;
        let dev = metadata.dev() as u32;
        let ino = metadata.ino() as u32;
        let uid = metadata.uid();
        let gid = metadata.gid();
        let file_size = metadata.size() as u32;

        let mode = compute_mode(&object_type);

        let flags = compute_flags(false, stage, path);

        IndexEntry {
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
            stage: stage,
            object_type: object_type,
            mode: mode,
            flags: flags,
            assume_valid: false,
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_regular_file_mode() {
        let expected_perm644: u32 = (0b1000 << 12) | 0o644;
        let expected_perm755: u32 = (0b1000 << 12) | 0o755;
        let obj_perm644 = IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm644);
        let obj_perm755 = IndexEntryObjectType::RegularFile(IndexEntryPermissions::Perm755);

        let mode_644 = compute_mode(&obj_perm644);
        let mode_755 = compute_mode(&obj_perm755);

        assert_eq!(mode_644, expected_perm644);
        assert_eq!(mode_755, expected_perm755);
    }

    #[test]
    fn test_symbolic_link_mode() {
        let obj_sym = IndexEntryObjectType::Symbolic;
        let expected_sym: u32 = 0b1010 << 12;

        let mode_sym = compute_mode(&obj_sym);

        assert_eq!(mode_sym, expected_sym);
    }

    #[test]
    fn test_gitlink_mode() {
        let obj_sym = IndexEntryObjectType::Gitlink;
        let expected_gitlink: u32 = 0b1110 << 12;

        let mode_gitlink = compute_mode(&obj_sym);

        assert_eq!(mode_gitlink, expected_gitlink);
    }
}
