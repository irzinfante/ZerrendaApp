use crate::{
    db::{
        identities_collection::IdentitiesCollection, sessions_collection::SessionsCollection,
        users_collection::UsersCollection,
    },
    errors::{AppError, ErrorType},
    helpers::{auth_helpers, util::new_id, validation},
    models::{identity_model::Identity, session_model::Session, user_model::User},
};
use chrono::Utc;
use couch_rs::Client;
use p256::ecdsa::SigningKey;

pub struct AuthService {
    identities_collection: IdentitiesCollection,
    users_collection: UsersCollection,
    sessions_collection: SessionsCollection,
    signing_key: SigningKey,
    // Verified against when the user doesn't exist, so that login takes the
    // same time whether the email is registered or not.
    dummy_password_hash: String,
}

impl AuthService {
    pub async fn new(db_client: &Client, signing_key: SigningKey) -> Self {
        let identities_collection = IdentitiesCollection::new(db_client).await;
        let users_collection = UsersCollection::new(db_client).await;
        let sessions_collection = SessionsCollection::new(db_client).await;
        let dummy_password_hash = auth_helpers::hash_password(new_id())
            .await
            .expect("Couldn't create dummy password hash");
        Self {
            identities_collection,
            users_collection,
            sessions_collection,
            signing_key,
            dummy_password_hash,
        }
    }

    pub async fn register(&self, email: String, password: String) -> Result<String, AppError> {
        let email = validation::normalize_email(&email).ok_or_else(|| {
            AppError::new(ErrorType::ValidationError, "Invalid email.".to_string())
        })?;
        if !validation::is_valid_password(&password) {
            return Err(AppError::new(
                ErrorType::ValidationError,
                "Invalid password.".to_string(),
            ));
        }

        // Hash before checking the email so that both outcomes take the same time.
        let password_hash = auth_helpers::hash_password(password).await?;

        // Creating the identity is the atomic uniqueness check: CouchDB rejects a
        // second document with the same `_id`.
        let user_id = new_id();
        let mut identity = Identity::email(&email, user_id.clone());
        if !self
            .identities_collection
            .create_identity(&mut identity)
            .await?
        {
            return Err(AppError::new(
                ErrorType::ValidationError,
                "Registration failed.".to_string(),
            ));
        }

        let mut new_user = User::new(user_id, email, password_hash);
        if let Err(error) = self.users_collection.create_user(&mut new_user).await {
            // Release the email so it isn't blocked by a user that doesn't exist.
            let _ = self.identities_collection.delete_identity(&identity).await;
            return Err(error);
        }

        self.create_session(new_user.get_id()).await
    }

    pub async fn login(&self, email: String, password: String) -> Result<String, AppError> {
        let invalid_credentials =
            || AppError::new(ErrorType::AuthError, "Invalid credentials.".to_string());

        // Rejecting malformed input early depends only on the input, so it doesn't
        // reveal which emails are registered.
        let email = validation::normalize_email(&email).ok_or_else(invalid_credentials)?;
        if password.len() > validation::PASSWORD_MAX_BYTES {
            return Err(invalid_credentials());
        }

        let user = match self
            .identities_collection
            .get_identity(&Identity::email_id(&email))
            .await?
        {
            Some(identity) => {
                self.users_collection
                    .get_user(&identity.get_user_id())
                    .await?
            }
            None => None,
        };

        // Always run bcrypt, even if the user doesn't exist, so that the response
        // time doesn't reveal which emails are registered.
        let password_hash = user
            .as_ref()
            .map_or(self.dummy_password_hash.clone(), User::get_password_hash);
        let is_valid = auth_helpers::verify_password(password_hash, password).await?;

        match user {
            Some(user) if is_valid => self.create_session(user.get_id()).await,
            _ => Err(invalid_credentials()),
        }
    }

    pub async fn authenticate(&self, token: &str) -> Result<Session, AppError> {
        let unauthorized = || AppError::new(ErrorType::AuthError, "Unauthorized.".to_string());

        let session_id =
            auth_helpers::verify_session_token(token, self.signing_key.verifying_key())
                .ok_or_else(unauthorized)?;
        let session = self
            .sessions_collection
            .get_session(&session_id)
            .await?
            .ok_or_else(unauthorized)?;

        let now = Utc::now();
        if now < session.get_created_at() || session.get_expires_at() < now {
            return Err(unauthorized());
        }

        Ok(session)
    }

    async fn create_session(&self, user_id: String) -> Result<String, AppError> {
        let mut session = Session::new(user_id)?;
        self.sessions_collection
            .create_session(&mut session)
            .await?;
        auth_helpers::create_session_token(&session.get_id(), &self.signing_key)
    }
}
