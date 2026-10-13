use crate::{
    errors::{AppError, ErrorType},
    models::session_model::Session,
};
use couch_rs::{Client, database::Database};

pub struct SessionsCollection {
    db: Database,
}

impl SessionsCollection {
    pub async fn new(db_client: &Client) -> Self {
        Self {
            db: db_client
                .db("sessions")
                .await
                .expect("Couldn't obtain 'sessions' database"),
        }
    }

    pub async fn create_session(&self, session: &mut Session) -> Result<(), AppError> {
        self.db
            .create(session)
            .await
            .map(|_| ())
            .map_err(|_| AppError::new(ErrorType::DbError, "Couldn't create session.".to_string()))
    }

    pub async fn get_session(&self, id: &str) -> Result<Option<Session>, AppError> {
        match self.db.get::<Session>(id).await {
            Ok(session) => Ok(Some(session)),
            Err(error) if error.is_not_found() => Ok(None),
            Err(_) => Err(AppError::new(
                ErrorType::DbError,
                "Couldn't get session.".to_string(),
            )),
        }
    }
}
