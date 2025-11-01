use crate::handler::{HandlerErrors, constants::*, utils::*};
use std::path::PathBuf;

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), HandlerErrors> {
    // Create a directory with the name

    let rgit_dir_path = name.join(RGIT_DIR_NAME);
    if rgit_dir_path.exists() {
        return Err(HandlerErrors::init_create_error(String::from(
            "rgit already initialized",
        )));
    }

    create_dir(&rgit_dir_path, true).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create directory in  {:?}: {}",
            name, e
        ))
    })?;

    // Creates HEAD file
    create_file(&rgit_dir_path, HEAD_FILE_NAME, None).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create HEAD file in  {:?}: {}",
            name, e
        ))
    })?;

    // Creates config file
    create_file(&rgit_dir_path, CONFIG_FILE_NAME, None).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create config file in  {:?}: {}",
            name, e
        ))
    })?;

    create_file(&rgit_dir_path, DESCRIPTION_FILE_NAME, description).map_err(|e| {
        HandlerErrors::init_create_error(format!("failed to create descriptiuon file {}", e))
    })?;

    let hooks_dir_path = &rgit_dir_path.join(HOOKS_DIR_NAME);
    create_dir(hooks_dir_path, false).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create HOOKS directory in  {:?}: {}",
            rgit_dir_path, e
        ))
    })?;

    let info_dir_path = &rgit_dir_path.join(INFO_DIR_NAME);
    create_dir(info_dir_path, false).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create INFO directory in  {:?}: {}",
            rgit_dir_path, e
        ))
    })?;

    let objects_dir_path = &rgit_dir_path.join(OBJECTS_DIR_NAME);
    create_dir(objects_dir_path, false).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create OBJECTS directory in  {:?}: {}",
            rgit_dir_path, e
        ))
    })?;

    let refs_dir_path = &rgit_dir_path.join(REFS_DIR_NAME);
    let _ = create_dir(refs_dir_path, false).map_err(|e| {
        HandlerErrors::init_create_error(format!(
            "failed to create REFS directory in  {:?}: {}",
            rgit_dir_path, e
        ))
    })?;

    println!("Initialized empty repo {:?}", name.clone());
    Ok(())
}
