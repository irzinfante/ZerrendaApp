use crate::{
    errors::{AppError, ErrorType},
    services::auth_service::AuthService,
};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn enforce_auth(
    State(auth_service): State<Arc<AuthService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::new(ErrorType::AuthError, "Unauthorized.".to_string()))?;

    let session = auth_service.authenticate(token).await?;

    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}
