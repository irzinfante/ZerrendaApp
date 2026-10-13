use std::str;

use crate::{
    config::Config,
    errors::{AppError, ErrorType},
    models::session_model::Session,
};
use axum::{
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use hex::decode;
use p256::ecdsa::{
    Signature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier},
};

pub fn hash_password(password: String) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::new(ErrorType::AuthError, "Unexpected error.".to_string()))
}

pub fn verify_password(hash: &str, password: String) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|_| AppError::new(ErrorType::AuthError, "Unexpected error.".to_string()))
}

pub fn create_session_token(session: Session) -> String {
    let auth_private_key = Config::from_env().auth_private_key;
    let signer = SigningKey::from_slice(&decode(auth_private_key).unwrap()).unwrap();
    let json_string = serde_json::to_string(&session).unwrap();
    let signature: Signature = signer.sign(json_string.as_bytes());
    signature.to_string() + "|" + &json_string
}

async fn validate_session_token(auth_header: String) -> Option<Session> {
    let auth_header_decoded = BASE64_STANDARD.decode(&auth_header).ok()?;
    let auth_header_parts = str::from_utf8(&auth_header_decoded)
        .ok()?
        .split("|")
        .collect::<Vec<&str>>();
    let signature = Signature::from_slice(&hex::decode(auth_header_parts[0]).ok()?).ok()?;

    let auth_private_key = Config::from_env().auth_private_key;
    let verifyer =
        VerifyingKey::from(SigningKey::from_slice(&decode(auth_private_key).ok()?).ok()?);
    verifyer
        .verify(auth_header_parts[1].as_bytes(), &signature)
        .ok()?;

    let session = serde_json::from_str(auth_header_parts[1]).ok()?;
    Some(session)
}

pub async fn enforce_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(session) = validate_session_token(auth_header.to_owned()).await {
        let now = Utc::now();
        if now < session.get_created_at() {
            Err(StatusCode::UNAUTHORIZED)
        } else if session.get_expires_at() < now {
            Err(StatusCode::UNAUTHORIZED)
        } else {
            req.extensions_mut().insert(session);
            Ok(next.run(req).await)
        }
        
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
