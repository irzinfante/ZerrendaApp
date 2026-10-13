use crate::{
    errors::{AppError, ErrorType},
    helpers::util::new_id,
};
use chrono::{DateTime, Days, Utc};
use couch_rs::CouchDocument;
use couch_rs::document::TypedCouchDocument;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument, Clone)]
pub struct Session {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    user_id: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: String) -> Result<Self, AppError> {
        let now = Utc::now();
        let expires_at = now.checked_add_days(Days::new(1)).ok_or_else(|| {
            AppError::new(
                ErrorType::InternalError,
                "Can't set expiration time.".to_string(),
            )
        })?;
        Ok(Session {
            _id: new_id(),
            _rev: String::new(),
            user_id,
            created_at: now,
            expires_at,
        })
    }

    pub fn get_id(&self) -> String {
        self._id.clone()
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}
