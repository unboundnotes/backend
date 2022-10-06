use async_graphql::Object;
use secrecy::Secret;
use user::User;
use uuid::Uuid;

use crate::repos::{mongodb_user_repo::MongoDBUserRepo, traits::UserRepo};

pub mod user;

pub struct QueryRoot {
    pub user_repo: MongoDBUserRepo,
}

#[Object]
impl QueryRoot {
    pub async fn get_user(&self, uuid: Uuid) -> Option<User> {
        self.user_repo.get_user(&uuid).await.unwrap()
    }
}

pub struct MutationRoot {
    pub user_repo: MongoDBUserRepo,
}

#[Object]
impl MutationRoot {
    pub async fn create_user(
        &self,
        email: String,
        username: String,
        password: Secret<String>,
    ) -> User {
        let mut user = User::new(&email, &username, password);
        self.user_repo.create_user(&mut user).await.unwrap();
        user
    }
}
