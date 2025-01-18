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
            message: message,
            display_message: display_message,
        }
    }
}
