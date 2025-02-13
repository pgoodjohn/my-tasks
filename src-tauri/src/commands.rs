use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    command: String,
    message: String,
    display_message: String,
}

impl ErrorResponse {
    pub fn new(command: String, message: String, display_message: String) -> Self {
        ErrorResponse {
            command,
            message,
            display_message,
        }
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_message)
    }
}

#[derive(Debug)]
pub enum CommandError {
    InvalidInput,
    InternalError,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandError::InvalidInput => write!(f, "Invalid input"),
            CommandError::InternalError => write!(f, "Internal error"),
        }
    }
}

// Implement Serialize for CommandError to convert it to JSON
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = self.to_string();
        serializer.serialize_str(&message)
    }
}
