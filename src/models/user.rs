use argon2::{self, Config as Argon2Config};
use async_graphql::Object;
use mongodb::bson::oid::ObjectId;
use rand_core::{OsRng, RngCore};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    password: String,
}

impl User {
    pub fn new(email: &str, username: &str, password: Secret<String>) -> Self {
        let uuid = Uuid::new_v4();
        let password = password.expose_secret().as_bytes();
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        let cfg = Argon2Config::default();
        let hashed_password = argon2::hash_encoded(password, &salt, &cfg).unwrap();
        Self {
            id: None,
            uuid,
            email: email.to_string(),
            username: username.to_string(),
            password: hashed_password,
        }
    }

    pub fn check_password(&self, password: &str) -> bool {
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }
}

#[Object]
impl User {
    /// The user's unique identifier.
    pub async fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// The user's email address.
    pub async fn email(&self) -> &str {
        &self.email
    }

    /// The user's username.
    pub async fn username(&self) -> &str {
        &self.username
    }
}
