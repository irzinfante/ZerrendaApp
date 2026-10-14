pub mod identity_model;
pub mod session_model;
pub mod user_model;

use crate::errors::AppError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
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
            error: Some(error.into_public()),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = self
            .error
            .as_ref()
            .map_or(StatusCode::OK, AppError::status_code);
        (status, Json(self)).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ApiResponse::<()>::error(self).into_response()
    }
}
