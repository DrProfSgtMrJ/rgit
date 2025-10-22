use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_file(
    dir_path: &Path,
    file_name: &str,
    content: Option<String>,
) -> Result<(), std::io::Error> {
    let full_path = dir_path.join(file_name);

    let mut file = File::create(full_path)?;
    if let Some(con) = content {
        file.write_all(con.as_bytes())?;
    }

    Ok(())
}

pub fn create_dir(
    root_dir_path: &Path,
    dir_name: &str,
    recursive: bool,
) -> Result<PathBuf, std::io::Error> {
    let full_path = root_dir_path.join(dir_name);

    if recursive {
        fs::create_dir_all(full_path.as_path())?;
    } else {
        fs::create_dir(full_path.as_path())?;
    }

    Ok(full_path)
}
