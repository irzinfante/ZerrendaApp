use crate::models::session_model::Session;
use std::sync::Arc;
use couch_rs::{database::Database, Client};
use tokio::sync::Mutex;

pub struct SessionsCollection {
    db: Arc<Mutex<Database>>,
}

impl SessionsCollection {
    pub async fn new(db_client: Arc<Mutex<Client>>) -> Self {
        Self {
            db: Arc::new(Mutex::new(
                db_client.lock().await.db("sessions").await.expect("Couldn't obtain 'sessions' database"),
            )),
        }
    }

    pub async fn create_session(&self, session: &mut Session) -> bool {
        let db = self.db.lock().await;
        let result = db.create(session).await;
        result.is_ok()
    }
}
