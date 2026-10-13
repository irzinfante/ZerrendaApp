use crate::helpers::util::new_id;
use couch_rs::CouchDocument;
use couch_rs::document::TypedCouchDocument;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        User {
            _id: new_id(),
            _rev: String::new(),
            username,
            password_hash,
        }
    }

    pub fn get_id(&self) -> String {
        self._id.clone()
    }

    pub fn get_rev(&self) -> String {
        self._rev.clone()
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
}
