#[derive(Debug)]
pub enum RgitError {
    AlreadyExists,
    CreateDirectory { message: String },
    DeleteDirectory { message: String },
    CreateFile { message: String },
    Write { message: String },
    InvalidPath { message: String },
}
