use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_user_by_uuid(&self, uuid: &Uuid) -> Result<Option<User>>;
    async fn create_user(&self, user: &mut User) -> Result<()>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn get_user_by_login(&self, login: &str) -> Result<Option<User>>;
}
