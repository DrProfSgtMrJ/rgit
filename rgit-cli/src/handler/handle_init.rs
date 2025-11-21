use crate::{
    handler::HandlerErrors,
    models::{Repo, RgitError, RgitRepo},
};
use std::path::PathBuf;

pub fn handle_init(name: PathBuf, description: Option<String>) -> Result<(), HandlerErrors> {
    // Create a directory with the name

    RgitRepo::init(name, description).map_err(|e| match e {
        RgitError::AlreadyExists => {
            HandlerErrors::init_create_error(String::from("Rgit dir already exists"))
        }
        RgitError::CreateDirectory { message } => HandlerErrors::init_create_error(message),
        RgitError::CreateFile { message } => HandlerErrors::init_create_error(message),
        RgitError::Write { message } => HandlerErrors::init_create_error(message),
        RgitError::InvalidPath { message } => HandlerErrors::init_create_error(message),
    })?;

    Ok(())
}
