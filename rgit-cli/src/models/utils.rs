use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

#[cfg(windows)]
use std::os::windows::MetadataExt;

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
