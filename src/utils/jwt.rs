use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::models::user::User;

pub fn generate_jwt(secret: &str, user: &User) -> Result<String> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.uuid.to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let token = claims.sign_with_key(&key)?;
    Ok(token)
}

#[allow(dead_code)]
pub fn verify_token(secret: &str, token: &str) -> Result<Uuid> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    let uuid = Uuid::parse_str(&claims["sub"])?;
    Ok(uuid)
}
