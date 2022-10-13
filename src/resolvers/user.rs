use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, FieldError, InputObject, Object, Result, ResultExt};
use secrecy::Secret;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    models::user::User,
    repos::{mongodb_user_repo::MongoDBUserRepo, traits::UserRepo},
    utils::{config::BaseConfig, jwt::generate_jwt},
};

#[derive(Debug, Error)]
pub enum UserMutationError {
    #[error("A user with this {0} already exists")]
    UserAlreadyExists(String),
    #[error("unknown data store error: {0}")]
    Default(#[from] anyhow::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
}

impl ErrorExtensions for UserMutationError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            UserMutationError::Default(_) => e.set("code", 500),
            UserMutationError::UserAlreadyExists(field) => {
                e.set("code", 400);
                e.set("field", field.as_str());
            }
            UserMutationError::UserNotFound => e.set("code", 404),
            UserMutationError::InvalidPassword => e.set("code", 400),
        })
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn get_user(&self, ctx: &Context<'_>, uuid: Uuid) -> Option<User> {
        let user_repo = ctx.data::<Arc<dyn UserRepo>>().unwrap();
        user_repo.get_user_by_uuid(&uuid).await.unwrap()
    }

    pub async fn current_user(&self, ctx: &Context<'_>) -> Option<User> {
        ctx.data::<Option<User>>().unwrap().clone()
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    email: String,
    username: String,
    #[graphql(secret)]
    password: String,
}

#[derive(InputObject)]
pub struct LoginUserInput {
    email: String,
    #[graphql(secret)]
    password: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, user: CreateUserInput) -> Result<User> {
        let user_repo = ctx.data::<MongoDBUserRepo>().unwrap();
        if user_repo
            .get_user_by_login(&user.username)
            .await
            .map_err(|e| UserMutationError::from(e).extend())?
            .is_some()
        {
            return Err(UserMutationError::UserAlreadyExists("username".to_string())).extend();
        }
        if user_repo
            .get_user_by_login(&user.email)
            .await
            .map_err(|e| UserMutationError::from(e).extend())?
            .is_some()
        {
            return Err(UserMutationError::UserAlreadyExists("email".to_string())).extend();
        }
        let mut user = User::new(&user.email, &user.username, &Secret::new(user.password));
        user_repo.create_user(&mut user).await?;
        Ok(user)
    }

    pub async fn login_user(&self, ctx: &Context<'_>, login: LoginUserInput) -> Result<String> {
        let user_repo = ctx.data::<MongoDBUserRepo>().unwrap();
        let config = ctx.data::<BaseConfig>().unwrap();
        let user = user_repo
            .get_user_by_login(&login.email)
            .await
            .map_err(|e| UserMutationError::from(e).extend())?
            .ok_or(UserMutationError::UserNotFound.extend())?;
        if !user.check_password(&Secret::new(login.password)) {
            return Err(UserMutationError::InvalidPassword.extend());
        }

        let token = generate_jwt(&config.mongo_db, &user)
            .map_err(|e| UserMutationError::from(e).extend())?;
        Ok(token)
    }
}
