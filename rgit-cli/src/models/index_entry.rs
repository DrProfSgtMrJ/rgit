use std::path::PathBuf;
use std::{fs, path::Path};

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
pub fn computer_object_type(path: &Path, meta: &fs::Metadata) -> IndexEntryObjectType {
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

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IndexEntry {
    ctimes: u32,  // Last time a file's metadata changed
    ctimens: u32, // nano secs
    mtimes: u32,  // Last time a file's data changed
    mtimens: u32, // nano secs
    dev: u32,     // Device ID (stat(2))
    ino: u32,     // File Serial number (stat(2))
    object_type: IndexEntryObjectType,
    stage: IndexEntryStage,
    uid: u32,       // User Id of the file (stat(2))
    gid: u32,       // Group Id of the file (stat(2))
    file_size: u32, // On-disk size (stat(2))
    assume_valid: bool,
    file_path: PathBuf,
}

impl IndexEntry {
    pub fn mode(&self) -> u32 {
        match &self.object_type {
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

    pub fn flags(&self) -> u16 {
        let mut value: u16 = 0;

        // 1-bit assume-valid (bit 15)
        if self.assume_valid {
            value |= 1 << 15;
        }

        // 1-bit extended flag (bit 14) - always 0 for version 2

        // 2-bit stage (bits 12-13)
        value |= (self.stage as u16 & 0b11) << 12;

        // 12-bit name length (bits 0-11)
        let name_len = self.file_path.as_os_str().len();
        let len_field = name_len.min(0xFFF) as u16;

        value |= len_field;

        value
    }

    pub fn object_name(&self) -> [u8; 20] {
        todo!()
    }
}

#[cfg(unix)]
impl From<&Path> for IndexEntry {
    fn from(path: &Path) -> Self {
        use std::{fs, os::unix::fs::MetadataExt};

        let metadata = fs::symlink_metadata(path).expect("Failed to get path metadata");

        let object_type = computer_object_type(path, &metadata);
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
            assume_valid: false,
        }
    }
}
