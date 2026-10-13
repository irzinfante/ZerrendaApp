use couch_rs::CouchDocument;
use couch_rs::document::TypedCouchDocument;
use serde::{Deserialize, Serialize};

/// Links a unique identity (e.g. an email) to a user. The identity is the
/// document `_id`, so CouchDB guarantees its uniqueness atomically.
#[derive(Serialize, Deserialize, CouchDocument, Clone)]
pub struct Identity {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    user_id: String,
}

impl Identity {
    pub fn email_id(email: &str) -> String {
        format!("email:{email}")
    }

    pub fn email(email: &str, user_id: String) -> Self {
        Identity {
            _id: Self::email_id(email),
            _rev: String::new(),
            user_id,
        }
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
}
