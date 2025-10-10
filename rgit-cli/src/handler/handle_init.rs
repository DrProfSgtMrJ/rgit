use std::{fs, path::PathBuf};

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory with the name
    println!("Works: {:?}", name);

    fs::create_dir_all(name.as_path())
        .map_err(|e| format!("failed to create directory {:?}: {}", name, e))?;

    println!("Initialized repo {:?}", name);
    return Ok(());
}
