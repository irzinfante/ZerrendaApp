use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    r#type: ErrorType,
    message: String,
}

#[derive(Debug, Serialize)]
pub enum ErrorType {
    AuthError,
}

impl AppError {
    pub fn new(r#type: ErrorType, message: String) -> Self {
        Self { r#type, message }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", &self.r#type, &self.message)
    }
}

impl std::error::Error for AppError {}
