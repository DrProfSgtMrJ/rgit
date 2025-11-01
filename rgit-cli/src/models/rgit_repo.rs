use crate::models::{create_dir, create_file, error::Error};
use std::path::{Path, PathBuf};

pub const RGIT_DIR_NAME: &str = ".rgit";
pub const HEAD_FILE_NAME: &str = "HEAD";
pub const CONFIG_FILE_NAME: &str = "config";
pub const DESCRIPTION_FILE_NAME: &str = "description";
pub const HOOKS_DIR_NAME: &str = "hooks";
pub const INFO_DIR_NAME: &str = "info";
pub const OBJECTS_DIR_NAME: &str = "objects";
pub const REFS_DIR_NAME: &str = "refs";

pub trait Repo: Sized {
    fn init(name: PathBuf, descritpion: Option<String>) -> Result<Self, Error>;
}

#[derive(Debug)]
pub struct RgitRepo {
    name: String,
    description: String,
    main_dir_path: PathBuf,
    initialized: bool,
}

impl RgitRepo {
    pub fn head_file(self) -> PathBuf {
        self.main_dir_path.join(HEAD_FILE_NAME)
    }

    pub fn config_file(self) -> PathBuf {
        self.main_dir_path.join(CONFIG_FILE_NAME)
    }

    pub fn description_file(self) -> PathBuf {
        self.main_dir_path.join(DESCRIPTION_FILE_NAME)
    }

    pub fn hooks_dir(self) -> PathBuf {
        self.main_dir_path.join(HOOKS_DIR_NAME)
    }

    pub fn objects_dir(self) -> PathBuf {
        self.main_dir_path.join(OBJECTS_DIR_NAME)
    }

    pub fn refs_dir_path(self) -> PathBuf {
        self.main_dir_path.join(REFS_DIR_NAME)
    }
}

impl Repo for RgitRepo {
    fn init(name: PathBuf, description: Option<String>) -> Result<Self, Error> {
        let name_str = String::from(name.to_str().unwrap_or_default());
        let rgit_dir_path = name.join(RGIT_DIR_NAME);
        if rgit_dir_path.exists() {
            return Err(Error::AlreadyExistsError);
        }

        create_dir(rgit_dir_path.as_path(), true).map_err(|e| Error::CreateDirectoryError {
            message: format!("failed to create Rgit dir: Error {}", e),
        })?;

        create_file(&rgit_dir_path, HEAD_FILE_NAME, None).map_err(|e| Error::CreateFileError {
            message: format!("failed to create HEAD file: Error {}", e),
        })?;

        create_file(&rgit_dir_path, CONFIG_FILE_NAME, None).map_err(|e| {
            Error::CreateFileError {
                message: format!("failed to create config file: Error {}", e),
            }
        })?;

        create_file(&rgit_dir_path, DESCRIPTION_FILE_NAME, description.clone()).map_err(|e| {
            Error::CreateFileError {
                message: format!("failed to create description file: Error {}", e),
            }
        })?;

        let hooks_dir_path = rgit_dir_path.join(HOOKS_DIR_NAME);
        create_dir(hooks_dir_path.as_path(), false).map_err(|e| Error::CreateDirectoryError {
            message: format!("failed to create HOOKS directory: Error {}", e),
        })?;

        let info_dir_path = rgit_dir_path.join(INFO_DIR_NAME);
        create_dir(info_dir_path.as_path(), false).map_err(|e| Error::CreateDirectoryError {
            message: format!("failed to create info directory: Error {}", e),
        })?;

        let objects_dir_path = rgit_dir_path.join(OBJECTS_DIR_NAME);
        create_dir(objects_dir_path.as_path(), false).map_err(|e| Error::CreateDirectoryError {
            message: format!("failed to create objects directory: Error {}", e),
        })?;

        let refs_dir_path = rgit_dir_path.join(REFS_DIR_NAME);
        create_dir(refs_dir_path.as_path(), false).map_err(|e| Error::CreateDirectoryError {
            message: format!("failed to create REFS directory: Error {}", e),
        })?;

        println!("Initialized empty repo {:?}", name.clone());

        Ok(RgitRepo {
            name: name_str,
            description: description.clone().unwrap_or_default(),
            main_dir_path: rgit_dir_path,
            initialized: true,
        })
    }
}
