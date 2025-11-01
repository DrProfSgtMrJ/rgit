pub enum Error {
    AlreadyExistsError,
    CreateDirectoryError { message: String },
    CreateFileError { message: String },
}
