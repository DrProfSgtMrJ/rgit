#[derive(Debug)]
pub enum RgitErrors {
    AlreadyExistsError,
    CreateDirectoryError { message: String },
    CreateFileError { message: String },
}
