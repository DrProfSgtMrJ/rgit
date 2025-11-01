use std::{
    fs::{self, File},
    io::Write,
    path::Path,
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

pub fn create_dir(full_path: &Path, recursive: bool) -> Result<(), std::io::Error> {
    if recursive {
        fs::create_dir_all(full_path)?;
    } else {
        fs::create_dir(full_path)?;
    }

    Ok(())
}
