use crate::{
    errors::{AppError, ErrorType},
    models::user_model::User,
};
use couch_rs::{Client, database::Database};

pub struct UsersCollection {
    db: Database,
}

impl UsersCollection {
    pub async fn new(db_client: &Client) -> Self {
        Self {
            db: db_client
                .db("users")
                .await
                .expect("Couldn't obtain 'users' database"),
        }
    }

    pub async fn get_user(&self, id: &str) -> Result<Option<User>, AppError> {
        match self.db.get::<User>(id).await {
            Ok(user) => Ok(Some(user)),
            Err(error) if error.is_not_found() => Ok(None),
            Err(_) => Err(AppError::new(
                ErrorType::DbError,
                "Couldn't get user.".to_string(),
            )),
        }
    }

    pub async fn create_user(&self, user: &mut User) -> Result<(), AppError> {
        self.db
            .create(user)
            .await
            .map(|_| ())
            .map_err(|_| AppError::new(ErrorType::DbError, "Couldn't create user.".to_string()))
    }
}
