use crate::{
    handler::HandlerErrors,
    models::{Error, Repo, RgitRepo},
};
use std::path::PathBuf;

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), HandlerErrors> {
    // Create a directory with the name

    RgitRepo::init(name, description).map_err(|e| match e {
        Error::AlreadyExistsError => {
            HandlerErrors::init_create_error(String::from("Rgit dir already exists"))
        }
        Error::CreateDirectoryError { message } => HandlerErrors::init_create_error(message),
        Error::CreateFileError { message } => HandlerErrors::init_create_error(message),
    })?;

    Ok(())
}
