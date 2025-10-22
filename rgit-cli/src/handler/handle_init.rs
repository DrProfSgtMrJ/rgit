use super::constants::*;
use super::utils::*;
use std::path::PathBuf;

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), String> {
    // Create a directory with the name
    let rgit_dir_path = create_dir(&name, RGIT_DIR_NAME, true)
        .map_err(|e| format!("failed to create directory in  {:?}: {}", name, e))?;

    // Creates HEAD file
    create_file(&rgit_dir_path, HEAD_FILE_NAME, None)
        .map_err(|e| format!("failed to create HEAD file in  {:?}: {}", name, e))?;

    // Creates config file
    create_file(&rgit_dir_path, CONFIG_FILE_NAME, None)
        .map_err(|e| format!("failed to create config file in  {:?}: {}", name, e))?;

    create_file(&rgit_dir_path, DESCRIPTION_FILE_NAME, description)
        .map_err(|e| format!("failed to create descriptiuon file {}", e))?;

    let _ = create_dir(&rgit_dir_path, HOOKS_DIR_NAME, false).map_err(|e| {
        format!(
            "failed to create HOOKS directory in  {:?}: {}",
            rgit_dir_path, e
        )
    })?;

    let _ = create_dir(&rgit_dir_path, INFO_DIR_NAME, false).map_err(|e| {
        format!(
            "failed to create INFO directory in  {:?}: {}",
            rgit_dir_path, e
        )
    })?;

    let _ = create_dir(&rgit_dir_path, OBJECTS_DIR_NAME, false).map_err(|e| {
        format!(
            "failed to create OBJECTS directory in  {:?}: {}",
            rgit_dir_path, e
        )
    })?;

    let _ = create_dir(&rgit_dir_path, REFS_DIR_NAME, false).map_err(|e| {
        format!(
            "failed to create REFS directory in  {:?}: {}",
            rgit_dir_path, e
        )
    })?;

    println!("Initialized repo {:?}", name.clone());
    Ok(())
}
