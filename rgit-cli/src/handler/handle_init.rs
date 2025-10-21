use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

const RGIT_DIR_NAME: &str = ".rgit";
const HEAD_FILE_NAME: &str = "HEAD";
const CONFIG_FILE_NAME: &str = "config";
const DESCRIPTION_FILE_NAME: &str = "description";

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory wit hthe name
    let rgit_dir_path = create_rgit_dir(&name)
        .map_err(|e| format!("failed to create directory in  {:?}: {}", name, e))?;

    // Creates HEAD file
    create_head_file(&rgit_dir_path)
        .map_err(|e| format!("failed to create HEAD file in  {:?}: {}", name, e))?;

    // Creates config file
    create_config_file(&rgit_dir_path)
        .map_err(|e| format!("failed to create config file in  {:?}: {}", name, e))?;

    Ok(())
}

fn create_rgit_dir(name: &PathBuf) -> Result<PathBuf, std::io::Error> {
    // /name/.rgit
    println!("name: {:?}", name);
    let rgit_dir_path = name.join(RGIT_DIR_NAME);
    fs::create_dir_all(rgit_dir_path.as_path())?;

    Ok(rgit_dir_path)
}

fn create_head_file(rgit_dir_path: &Path) -> Result<(), std::io::Error> {
    let head_file_path = rgit_dir_path.join(HEAD_FILE_NAME);

    let _ = File::create(head_file_path)?;

    Ok(())
}

fn create_config_file(rgit_dir_path: &Path) -> Result<(), std::io::Error> {
    let config_file_path = rgit_dir_path.join(CONFIG_FILE_NAME);

    let _ = File::create(config_file_path)?;

    Ok(())
}
