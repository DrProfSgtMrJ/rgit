use crate::models::utils::{compute_flags, compute_object_type};
use byteorder::{BigEndian, WriteBytesExt};
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

impl IndexEntryObjectType {
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

        let object_type = compute_object_type(path, &metadata);

        if object_type.is_none() {
            return None;
        }

        let object_type = object_type.unwrap();
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

        let mode = object_type.mode();

        let flags = compute_flags(false, stage, path);

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
            stage: stage,
            object_type: object_type,
            mode: mode,
            flags: flags,
            assume_valid: false,
        })
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {

    use super::*;

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
}
