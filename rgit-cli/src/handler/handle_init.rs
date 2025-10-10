use std::{
    fs::{self, File},
    path::PathBuf,
};

const HEAD_FILE_NAME: &str = "HEAD";
pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory with the name
    fs::create_dir_all(name.as_path())
        .map_err(|e| format!("failed to create directory {:?}: {}", name, e))?;

    // Creates HEAD file
    let mut head_file = PathBuf::from(name.clone());
    head_file.push(HEAD_FILE_NAME);

    let _ =
        File::create(head_file).map_err(|e| format!("failed to create Head file in {:?}", name))?;
    println!("Initialized repo {:?}", name.clone());
    return Ok(());
}
