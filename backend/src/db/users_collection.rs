use crate::models::user_model::User;
use couch_rs::{Client, database::Database, types::find::FindQuery};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct UsersCollection {
    db: Arc<Mutex<Database>>,
}

impl UsersCollection {
    pub async fn new(db_client: Arc<Mutex<Client>>) -> Self {
        Self {
            db: Arc::new(Mutex::new(
                db_client.lock().await.db("users").await.expect("Couldn't obtain 'users' database")
            )),
        }
    }

    pub async fn get_user_by_username(&self, username: String) -> Option<User> {
        let db = self.db.lock().await;
        let query = FindQuery::new(json!({"username": username }));
        match db.find::<User>(&query).await {
            Ok(docs) => {
                if docs.total_rows > 0 {
                    docs.rows.first().cloned()
                } else {
                    None
                }
            }
            Err(_) => None
        }
    }

    pub async fn create_user(&self, user: &mut User) -> bool {
        let db = self.db.lock().await;
        let result = db.create(user).await;
        result.is_ok()
    }
}
