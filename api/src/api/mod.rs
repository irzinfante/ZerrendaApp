mod auth_api;
mod dummy_api;
mod extractors;
pub mod middleware;

use crate::services::auth_service::AuthService;
use axum::{Router, routing::post};
use std::sync::Arc;

pub fn auth_api(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/auth/register", post(auth_api::register))
        .route("/auth/login", post(auth_api::login))
        .with_state(auth_service)
}

pub fn dummy_api() -> Router {
    Router::new().route("/dummy", post(dummy_api::dummy))
}
