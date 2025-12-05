use crate::models::{Index, RgitIndex, create_dir, create_file, error::RgitError, remove_dir};
use std::path::PathBuf;

pub const RGIT_DIR_NAME: &str = ".rgit";
pub const HEAD_FILE_NAME: &str = "HEAD";
pub const CONFIG_FILE_NAME: &str = "config";
pub const DESCRIPTION_FILE_NAME: &str = "description";
pub const HOOKS_DIR_NAME: &str = "hooks";
pub const INFO_DIR_NAME: &str = "info";
pub const OBJECTS_DIR_NAME: &str = "objects";
pub const REFS_DIR_NAME: &str = "refs";

pub trait Repo: Sized {
    fn init(name: PathBuf, descritpion: Option<String>) -> Result<Self, RgitError>;
    fn remove(&self) -> Result<(), RgitError>;
}

#[derive(Debug)]
pub struct RgitRepo {
    name: String,
    description: String,
    main_dir_path: PathBuf,
    index: RgitIndex,
    initialized: bool,
    head_file_path: PathBuf,
    config_file_path: PathBuf,
    descritpion_file_path: PathBuf,
    hooks_dir_path: PathBuf,
    info_dir_path: PathBuf,
    objects_dir_path: PathBuf,
    refs_dir_path: PathBuf,
}

impl RgitRepo {
    pub fn index_dir_path(&self) -> &PathBuf {
        &self.index.dir_path
    }
}

impl Repo for RgitRepo {
    fn init(name: PathBuf, description: Option<String>) -> Result<Self, RgitError> {
        let name_str = String::from(name.to_str().unwrap_or_default());
        let rgit_dir_path = name.join(RGIT_DIR_NAME);
        if rgit_dir_path.exists() {
            return Err(RgitError::AlreadyExists);
        }

        create_dir(rgit_dir_path.as_path(), true).map_err(|e| RgitError::CreateDirectory {
            message: format!("failed to create Rgit dir: Error {}", e),
        })?;

        create_file(&rgit_dir_path, HEAD_FILE_NAME, &[]).map_err(|e| RgitError::CreateFile {
            message: format!("failed to create HEAD file: Error {}", e),
        })?;

        create_file(&rgit_dir_path, CONFIG_FILE_NAME, &[]).map_err(|e| RgitError::CreateFile {
            message: format!("failed to create config file: Error {}", e),
        })?;

        let description_bytes = description
            .as_ref()
            .map_or(&[] as &[u8], |desc| desc.as_bytes());

        create_file(&rgit_dir_path, DESCRIPTION_FILE_NAME, description_bytes).map_err(|e| {
            RgitError::CreateFile {
                message: format!("failed to create description file: Error {}", e),
            }
        })?;

        let hooks_dir_path = rgit_dir_path.join(HOOKS_DIR_NAME);
        create_dir(hooks_dir_path.as_path(), false).map_err(|e| RgitError::CreateDirectory {
            message: format!("failed to create HOOKS directory: Error {}", e),
        })?;

        let info_dir_path = rgit_dir_path.join(INFO_DIR_NAME);
        create_dir(info_dir_path.as_path(), false).map_err(|e| RgitError::CreateDirectory {
            message: format!("failed to create info directory: Error {}", e),
        })?;

        let objects_dir_path = rgit_dir_path.join(OBJECTS_DIR_NAME);
        create_dir(objects_dir_path.as_path(), false).map_err(|e| RgitError::CreateDirectory {
            message: format!("failed to create objects directory: Error {}", e),
        })?;

        let refs_dir_path = rgit_dir_path.join(REFS_DIR_NAME);
        create_dir(refs_dir_path.as_path(), false).map_err(|e| RgitError::CreateDirectory {
            message: format!("failed to create REFS directory: Error {}", e),
        })?;

        let index = RgitIndex::init(&rgit_dir_path, 2)?;

        println!("Initialized empty repo {:?}", name.clone());

        let head_file_path = rgit_dir_path.join(HEAD_FILE_NAME);
        let config_file_path = rgit_dir_path.join(CONFIG_FILE_NAME);
        let description_file_path = rgit_dir_path.join(DESCRIPTION_FILE_NAME);

        Ok(RgitRepo {
            name: name_str,
            description: description.clone().unwrap_or_default(),
            main_dir_path: rgit_dir_path,
            index,
            initialized: true,
            head_file_path: head_file_path,
            config_file_path: config_file_path,
            descritpion_file_path: description_file_path,
            hooks_dir_path: hooks_dir_path,
            info_dir_path: info_dir_path,
            objects_dir_path: objects_dir_path,
            refs_dir_path: refs_dir_path,
        })
    }
    fn remove(&self) -> Result<(), RgitError> {
        match remove_dir(self.main_dir_path.as_path(), true) {
            Ok(_) => Ok(()),
            Err(e) => Err(RgitError::DeleteDirectory {
                message: format!("Failed to remove directory: {:?}", e),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init() {
        // create temp dir
        const REPO_NAME: &str = "myrepo";
        let temp_dir = tempdir().expect("failed to created temp dir");

        let repo_result = RgitRepo::init(REPO_NAME.into(), Some("myrepo".into()));

        assert!(repo_result.is_ok());
        let repo = repo_result.unwrap();

        assert!(&repo.main_dir_path.exists());

        assert!(repo.refs_dir_path.exists());
        assert!(repo.config_file_path.exists());
        assert!(repo.head_file_path.exists());
        assert!(repo.descritpion_file_path.exists());
        assert!(repo.hooks_dir_path.exists());
        assert!(repo.info_dir_path.exists());
        assert!(repo.objects_dir_path.exists());
        assert!(repo.index_dir_path().exists());

        let remove_result = repo.remove();
        assert!(remove_result.is_ok());

        assert!(!&repo.main_dir_path.exists());

        assert!(!repo.refs_dir_path.exists());
        assert!(!repo.config_file_path.exists());
        assert!(!repo.head_file_path.exists());
        assert!(!repo.descritpion_file_path.exists());
        assert!(!repo.hooks_dir_path.exists());
        assert!(!repo.info_dir_path.exists());
        assert!(!repo.objects_dir_path.exists());
        assert!(!repo.index_dir_path().exists());
    }
}
