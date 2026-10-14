use crate::errors::{AppError, ErrorType};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use bcrypt::{DEFAULT_COST, hash, verify};
use hex::decode;
use p256::ecdsa::{
    Signature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier},
};
use uuid::Uuid;

// Session token: base64url(ECDSA P-256 signature (64 bytes) ‖ session id (16-byte UUID)).
const SIGNATURE_LEN: usize = 64;
const SESSION_ID_LEN: usize = 16;

fn unexpected_error<E>(_: E) -> AppError {
    AppError::new(ErrorType::InternalError, "Unexpected error.".to_string())
}

// bcrypt is CPU-intensive on purpose, so it runs on Tokio's blocking thread pool
// instead of the async workers that serve requests.
pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(unexpected_error)?
        .map_err(unexpected_error)
}

pub async fn verify_password(hash: String, password: String) -> Result<bool, AppError> {
    tokio::task::spawn_blocking(move || verify(password, &hash))
        .await
        .map_err(unexpected_error)?
        .map_err(unexpected_error)
}

pub fn signing_key_from_hex(private_key_hex: &str) -> Option<SigningKey> {
    SigningKey::from_slice(&decode(private_key_hex).ok()?).ok()
}

pub fn create_session_token(
    session_id: &str,
    signing_key: &SigningKey,
) -> Result<String, AppError> {
    let session_id = Uuid::parse_str(session_id).map_err(unexpected_error)?;
    let signature: Signature = signing_key.sign(session_id.as_bytes());
    let mut token = signature.to_bytes().to_vec();
    token.extend_from_slice(session_id.as_bytes());
    Ok(BASE64_URL_SAFE_NO_PAD.encode(token))
}

pub fn verify_session_token(token: &str, verifying_key: &VerifyingKey) -> Option<String> {
    let token = BASE64_URL_SAFE_NO_PAD.decode(token).ok()?;
    if token.len() != SIGNATURE_LEN + SESSION_ID_LEN {
        return None;
    }
    let (signature, session_id) = token.split_at(SIGNATURE_LEN);
    let signature = Signature::from_slice(signature).ok()?;
    verifying_key.verify(session_id, &signature).ok()?;
    Some(Uuid::from_slice(session_id).ok()?.to_string())
}
