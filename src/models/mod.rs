use async_graphql::{Context, Object};
use secrecy::Secret;
use user::User;
use uuid::Uuid;

use crate::repos::{mongodb_user_repo::MongoDBUserRepo, traits::UserRepo};

pub mod mutations;
pub mod user;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn get_user(&self, ctx: &Context<'_>, uuid: Uuid) -> Option<User> {
        let user_repo = ctx.data::<MongoDBUserRepo>().unwrap();
        user_repo.get_user_by_uuid(&uuid).await.unwrap()
    }
}
