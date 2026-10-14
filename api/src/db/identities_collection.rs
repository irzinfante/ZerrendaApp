use crate::{
    errors::{AppError, ErrorType},
    models::identity_model::Identity,
};
use couch_rs::{Client, database::Database, http::StatusCode};

pub struct IdentitiesCollection {
    db: Database,
}

impl IdentitiesCollection {
    pub async fn new(db_client: &Client) -> Self {
        Self {
            db: db_client
                .db("identities")
                .await
                .expect("Couldn't obtain 'identities' database"),
        }
    }

    /// Returns `Ok(false)` if the identity already exists.
    pub async fn create_identity(&self, identity: &mut Identity) -> Result<bool, AppError> {
        match self.db.create(identity).await {
            Ok(_) => Ok(true),
            Err(error) if error.status() == Some(StatusCode::CONFLICT) => Ok(false),
            Err(_) => Err(AppError::new(
                ErrorType::DbError,
                "Couldn't create identity.".to_string(),
            )),
        }
    }

    pub async fn get_identity(&self, id: &str) -> Result<Option<Identity>, AppError> {
        match self.db.get::<Identity>(id).await {
            Ok(identity) => Ok(Some(identity)),
            Err(error) if error.is_not_found() => Ok(None),
            Err(_) => Err(AppError::new(
                ErrorType::DbError,
                "Couldn't get identity.".to_string(),
            )),
        }
    }

    pub async fn delete_identity(&self, identity: &Identity) -> Result<(), AppError> {
        self.db
            .remove(identity)
            .await
            .map(|_| ())
            .map_err(|_| AppError::new(ErrorType::DbError, "Couldn't delete identity.".to_string()))
    }
}
