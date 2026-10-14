use couch_rs::{CouchDocument, document::TypedCouchDocument};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    email: String,
    password_hash: String,
}

impl User {
    pub fn new(id: String, email: String, password_hash: String) -> Self {
        User {
            _id: id,
            _rev: String::new(),
            email,
            password_hash,
        }
    }

    pub fn get_id(&self) -> String {
        self._id.clone()
    }

    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
}
