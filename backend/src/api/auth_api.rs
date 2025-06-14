use crate::{
    helpers::auth_helpers::{create_session_token, hash_password},
    models::ApiResponse,
    services::auth_service::AuthService,
};
use axum::{Json, extract::State};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    session_token: String,
}

pub async fn register(
    State(auth_service): State<Arc<AuthService>>,
    Json(req): Json<RegisterRequest>,
) -> Json<ApiResponse<RegisterResponse>> {
    let password_hash = match hash_password(req.password) {
        Ok(hash) => hash,
        Err(error) => return Json(ApiResponse::error(error)),
    };

    match auth_service.register(req.username, password_hash).await {
        Ok(session) => Json(ApiResponse::success(RegisterResponse {
            session_token: BASE64_STANDARD.encode(create_session_token(session)),
        })),
        Err(error) => Json(ApiResponse::error(error)),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    session_token: String,
}

pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(req): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    match auth_service.login(req.username, req.password).await {
        Ok(session) => Json(ApiResponse::success(LoginResponse {
            session_token: BASE64_STANDARD.encode(create_session_token(session)),
        })),
        Err(error) => Json(ApiResponse::error(error)),
    }
}
