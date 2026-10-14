use crate::errors::{AppError, ErrorType};
use axum::{
    Json, async_trait,
    extract::{FromRequest, Request, rejection::JsonRejection},
};

pub struct ApiJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ApiJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(ApiJson(value)),
            Err(_) => Err(AppError::new(
                ErrorType::ValidationError,
                "Invalid request body.".to_string(),
            )),
        }
    }
}
