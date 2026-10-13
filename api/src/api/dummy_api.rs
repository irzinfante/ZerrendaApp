use super::extractors::ApiJson;
use crate::models::{ApiResponse, session_model::Session};
use axum::Extension;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DummyRequest {
    dummy_string: String,
}

#[derive(Serialize)]
pub struct DummyResponse {
    dummy_message: String,
}

pub async fn dummy(
    Extension(session): Extension<Session>,
    ApiJson(req): ApiJson<DummyRequest>,
) -> ApiResponse<DummyResponse> {
    ApiResponse::success(DummyResponse {
        dummy_message: format!(
            "Your dummy string is: {}. Session is: {}. And user is {}",
            req.dummy_string,
            session.get_id(),
            session.get_user_id()
        ),
    })
}
