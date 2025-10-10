use std::{
    fs::{self, File},
    path::PathBuf,
};

const RGIT_DIR_NAME: &str = ".rgit";
const HEAD_FILE_NAME: &str = "HEAD";
const CONFIG_FILE_NAME: &str = "config";

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory with the name
    let rgit_dir = name.join(RGIT_DIR_NAME);
    fs::create_dir_all(rgit_dir.as_path())
        .map_err(|e| format!("failed to create directory in {:?}: {}", name, e))?;

    // Creates HEAD file
    let head_file = rgit_dir.join(HEAD_FILE_NAME);

    let _ = File::create(head_file)
        .map_err(|e| format!("failed to create HEAD file in {:?}: {}", name, e))?;

    // Creates config file
    let config_file = rgit_dir.join(CONFIG_FILE_NAME);
    let _ = File::create(config_file)
        .map_err(|e| format!("failed to create config file in {:?}: {}", name, e))?;

    println!("Initialized repo {:?}", name.clone());
    return Ok(());
}
