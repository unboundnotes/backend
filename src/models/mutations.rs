use async_graphql::{Context, ErrorExtensions, FieldError, InputObject, Object, Result, ResultExt};
use secrecy::Secret;
use thiserror::Error;

use crate::repos::{mongodb_user_repo::MongoDBUserRepo, traits::UserRepo};

use super::user::User;

pub struct MutationRoot;

#[derive(Debug, Error)]
pub enum MutationError {
    #[error("A user with this {0} already exists")]
    UserAlreadyExists(String),
    #[error("unknown data store error: {0}")]
    Default(#[from] anyhow::Error),
}

impl ErrorExtensions for MutationError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            MutationError::Default(_) => e.set("code", 500),
            MutationError::UserAlreadyExists(field) => {
                e.set("code", 400);
                e.set("field", field.as_str());
            }
        })
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    email: String,
    username: String,
    #[graphql(secret)]
    password: String,
}

#[Object]
impl MutationRoot {
    pub async fn create_user(&self, ctx: &Context<'_>, user: CreateUserInput) -> Result<User> {
        let user_repo = ctx.data::<MongoDBUserRepo>().unwrap();
        if let Some(_) = user_repo
            .get_user_by_login(&user.username)
            .await
            .map_err(|e| MutationError::from(e).extend())?
        {
            return Err(MutationError::UserAlreadyExists("username".to_string())).extend();
        }
        if let Some(_) = user_repo
            .get_user_by_login(&user.email)
            .await
            .map_err(|e| MutationError::from(e).extend())?
        {
            return Err(MutationError::UserAlreadyExists("email".to_string())).extend();
        }
        let mut user = User::new(&user.email, &user.username, Secret::new(user.password));
        user_repo.create_user(&mut user).await?;
        Ok(user)
    }
}
