use std::{fs, path::PathBuf};

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory with the name
    println!("Works: {:?}", name);

    return Ok(());
}
