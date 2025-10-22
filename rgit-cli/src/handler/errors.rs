#[derive(Debug)]
pub enum HandlerErrors {
    InitCreateError { message: String },
}

impl HandlerErrors {
    pub fn init_create_error(error_message: String) -> HandlerErrors {
        HandlerErrors::InitCreateError {
            message: error_message,
        }
    }

    pub fn message(self) -> String {
        match self {
            Self::InitCreateError { message } => message,
        }
    }
}
