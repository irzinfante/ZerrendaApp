use couch_rs::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::{sessions_collection::SessionsCollection, users_collection::UsersCollection},
    errors::{AppError, ErrorType},
    helpers::auth_helpers,
    models::{session_model::Session, user_model::User},
};

pub struct AuthService {
    users_collection: Arc<UsersCollection>,
    sessions_collection: Arc<SessionsCollection>,
}

impl AuthService {
    pub async fn new(db_client: Arc<Mutex<Client>>) -> Self {
        let users_collection = Arc::new(UsersCollection::new(db_client.clone()).await);
        let sessions_collection = Arc::new(SessionsCollection::new(db_client.clone()).await);
        Self {
            users_collection,
            sessions_collection,
        }
    }

    pub async fn register(
        &self,
        username: String,
        password_hash: String,
    ) -> Result<Session, AppError> {
        match self
            .users_collection
            .get_user_by_username(username.clone())
            .await
        {
            Some(_) => Err(AppError::new(
                ErrorType::AuthError,
                "Username is already registered.".to_string(),
            )),
            None => {
                let new_user = User::new(username.clone(), password_hash);
                if self.users_collection.create_user(&mut new_user.clone()).await {
                    let new_session = Session::new(new_user.get_id());
                    if self
                        .sessions_collection
                        .create_session(&mut new_session.clone())
                        .await
                    {
                        Ok(new_session)
                    } else {
                        Err(AppError::new(
                            ErrorType::AuthError,
                            "Couldn't create session.".to_string(),
                        ))
                    }
                } else {
                    Err(AppError::new(
                        ErrorType::AuthError,
                        "Couldn't create user.".to_string(),
                    ))
                }
            }
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<Session, AppError> {
        match self
            .users_collection
            .get_user_by_username(username.clone())
            .await
        {
            Some(user) => {
                match auth_helpers::verify_password(&user.get_password_hash(), password) {
                    Ok(is_valid) => {
                        if is_valid {
                            let new_session = Session::new(user.get_id());
                            if self
                                .sessions_collection
                                .create_session(&mut new_session.clone())
                                .await
                            {
                                Ok(new_session)
                            } else {
                                Err(AppError::new(
                                    ErrorType::AuthError,
                                    "Couldn't create session.".to_string(),
                                ))
                            }
                        } else {
                            Err(AppError::new(
                                ErrorType::AuthError,
                                "Invalid credentials.".to_string(),
                            ))
                        }
                    }
                    Err(error) => Err(error),
                }
            }
            None => Err(AppError::new(
                ErrorType::AuthError,
                "Invalid credentials.".to_string(),
            )),
        }
    }
}
