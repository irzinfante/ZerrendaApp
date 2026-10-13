use axum::http::StatusCode;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    r#type: ErrorType,
    message: String,
}

// The variant names are the `error.type` values of the API contract.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Serialize)]
pub enum ErrorType {
    ValidationError,
    AuthError,
    InternalError,
    DbError,
}

impl ErrorType {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ErrorType::ValidationError => StatusCode::BAD_REQUEST,
            ErrorType::AuthError => StatusCode::UNAUTHORIZED,
            ErrorType::InternalError | ErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl AppError {
    pub fn new(r#type: ErrorType, message: String) -> Self {
        Self { r#type, message }
    }

    pub fn status_code(&self) -> StatusCode {
        self.r#type.status_code()
    }

    /// Version of the error that is safe to send to the client: internal
    /// failures are collapsed into a generic `InternalError`.
    pub fn into_public(self) -> Self {
        match self.r#type {
            ErrorType::InternalError | ErrorType::DbError => {
                Self::new(ErrorType::InternalError, "Unexpected error.".to_string())
            }
            _ => self,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", &self.r#type, &self.message)
    }
}

impl std::error::Error for AppError {}
