use argon2::{self, Config as Argon2Config};
use async_graphql::SimpleObject;
use diesel::prelude::*;
use rand_core::{OsRng, RngCore};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(
    Debug, Serialize, Deserialize, Clone, SimpleObject, Queryable, Insertable, AsChangeset,
)]
#[diesel(table_name = users)]
pub struct User {
    /// The user's unique identifier.
    pub uuid: Uuid,

    /// The user's email address.
    pub email: String,

    /// The user's username.
    pub username: String,

    #[graphql(skip)]
    password: String,
}

impl User {
    pub fn new(email: &str, username: &str, password: &Secret<String>) -> Self {
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

    pub fn check_password(&self, password: &Secret<String>) -> bool {
        argon2::verify_encoded(&self.password, password.expose_secret().as_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[test]
    fn test_check_password() {
        let user = User::new(
            "test@example.com",
            "test",
            &Secret::new("password".to_string()),
        );
        assert!(user.check_password(&Secret::new("password".to_string())));
        assert!(!user.check_password(&Secret::new("wrong password".to_string())));
    }
}
