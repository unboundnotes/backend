use async_graphql::Object;
use secrecy::Secret;
use user::User;
use uuid::Uuid;

pub mod user;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn get_user(&self, uuid: Uuid) -> User {
        println!("uuid: {:?}", uuid);
        User::new("test@example.com", "test", "test".to_string().into())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user(
        &self,
        email: String,
        username: String,
        password: Secret<String>,
    ) -> User {
        User::new(&email, &username, password)
    }
}
