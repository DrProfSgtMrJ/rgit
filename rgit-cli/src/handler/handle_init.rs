use crate::{
    handler::HandlerErrors,
    models::{Repo, RgitErrors, RgitRepo},
};
use std::path::PathBuf;

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), HandlerErrors> {
    // Create a directory with the name

    RgitRepo::init(name, description).map_err(|e| match e {
        RgitErrors::AlreadyExistsError => {
            HandlerErrors::init_create_error(String::from("Rgit dir already exists"))
        }
        RgitErrors::CreateDirectoryError { message } => HandlerErrors::init_create_error(message),
        RgitErrors::CreateFileError { message } => HandlerErrors::init_create_error(message),
    })?;

    Ok(())
}
