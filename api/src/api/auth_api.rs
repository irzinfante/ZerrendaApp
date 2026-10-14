use super::extractors::ApiJson;
use crate::{models::ApiResponse, services::auth_service::AuthService};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    session_token: String,
}

pub async fn register(
    State(auth_service): State<Arc<AuthService>>,
    ApiJson(req): ApiJson<RegisterRequest>,
) -> ApiResponse<RegisterResponse> {
    match auth_service.register(req.email, req.password).await {
        Ok(session_token) => ApiResponse::success(RegisterResponse { session_token }),
        Err(error) => ApiResponse::error(error),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    session_token: String,
}

pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    ApiJson(req): ApiJson<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    match auth_service.login(req.email, req.password).await {
        Ok(session_token) => ApiResponse::success(LoginResponse { session_token }),
        Err(error) => ApiResponse::error(error),
    }
}
