use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

const RGIT_DIR_NAME: &str = ".rgit";
const HEAD_FILE_NAME: &str = "HEAD";
const CONFIG_FILE_NAME: &str = "config";
const DESCRIPTION_FILE_NAME: &str = "description";
const HOOKS_DIR_NAME: &str = "/hooks";
const INFO_DIR_NAME: &str = "/info";
const OBJECTS_DIR_NAME: &str = "/objects";
const REFS_DIR_NAME: &str = "/refs";

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

    println!("Initialized repo {:?}", name.clone());
    Ok(())
}

fn create_file(
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

fn create_dir(
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
