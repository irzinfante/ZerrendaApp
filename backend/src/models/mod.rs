pub mod session_model;
pub mod user_model;

use crate::errors::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    error: Option<AppError>,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(t: T) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(t),
        }
    }

    pub fn error(error: AppError) -> Self {
        Self {
            success: false,
            error: Some(error),
            data: None,
        }
    }
}
