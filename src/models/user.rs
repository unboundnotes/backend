use argon2::{self, Config as Argon2Config};
use async_graphql::Object;
use rand_core::{OsRng, RngCore};
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    uuid: Uuid,
    email: String,
    username: String,
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
    async fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// The user's email address.
    async fn email(&self) -> &str {
        &self.email
    }

    /// The user's username.
    async fn username(&self) -> &str {
        &self.username
    }
}
