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
