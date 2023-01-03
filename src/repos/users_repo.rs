use ::uuid::Uuid as TUuid;
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::traits::UserRepo;
use crate::models::user::User;

pub struct PostgresqlUsersRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresqlUsersRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PostgresqlUsersRepo {
    async fn get_user_by_uuid(&self, uuid_val: &TUuid) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get()?;
        let result = users
            .filter(uuid.eq(uuid_val))
            .first::<User>(&mut conn)
            .optional()?;
        Ok(result)
    }

    async fn create_user(&self, user: &User) -> Result<()> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::insert_into(users).values(user).execute(&mut conn)?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::update(users)
            .filter(uuid.eq(&user.uuid))
            .set(user)
            .execute(&mut conn)?;
        Ok(())
    }
    async fn get_user_by_login(&self, login: &str) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get()?;
        let result = users
            .filter(email.eq(login).or(username.eq(login)))
            .first::<User>(&mut conn)
            .optional()?;
        Ok(result)
    }
}
