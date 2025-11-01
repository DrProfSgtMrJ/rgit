use byteorder::{BigEndian, WriteBytesExt};
use std::path::PathBuf;

use crate::models::{RgitErrors, create_file};

pub const INDEX_HEADER_SIGNATURE: &str = "DIRC";
pub const INDEX_FILE_NAME: &str = "index";

pub trait Index: Sized {
    fn init(root_dir: &PathBuf, version: u32) -> Result<Self, RgitErrors>;
    fn add_entry(&mut self, entry: &IndexEntry);
}

#[derive(Debug)]
pub struct IndexEntry {}

#[derive(Debug)]
pub struct RgitIndex {
    dir_path: PathBuf,
    version: u32,
    entries: Vec<IndexEntry>,
}

impl Index for RgitIndex {
    fn init(root_dir: &PathBuf, version: u32) -> Result<Self, RgitErrors> {
        let full_dir_path = root_dir.join(INDEX_FILE_NAME);
        let mut index_content = Vec::new();
        index_content.extend_from_slice(INDEX_HEADER_SIGNATURE.as_bytes());
        index_content.write_u32::<BigEndian>(version);
        index_content.write_u32::<BigEndian>(0); // Number of entries which is 0 atm
        create_file(root_dir.as_path(), INDEX_FILE_NAME, &index_content);

        Ok(RgitIndex {
            dir_path: full_dir_path,
            version,
            entries: Vec::new(),
        })
    }

    fn add_entry(&mut self, entry: &IndexEntry) {
        todo!()
    }
}
