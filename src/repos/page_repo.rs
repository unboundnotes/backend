use anyhow::Error;
use async_trait::async_trait;
use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::models::Page;

use super::traits;

pub struct PageRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PageRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl traits::PageRepo for PageRepo {
    async fn get_page_by_uuid(&self, uuid: &Uuid) -> Result<Option<Page>, Error> {
        use crate::schema::pages;

        let mut conn = self.pool.get()?;
        let page = pages::dsl::pages
            .filter(pages::columns::uuid.eq(uuid))
            .first::<Page>(&mut conn)
            .optional()?;

        Ok(page)
    }

    async fn create_page(&self, page: &Page) -> Result<(), Error> {
        use crate::schema::pages::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::insert_into(pages).values(page).execute(&mut conn)?;

        Ok(())
    }

    async fn update_page(&self, page: &Page) -> Result<(), Error> {
        use crate::schema::pages::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::update(pages.filter(uuid.eq(page.uuid)))
            .set(page)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn delete_page(&self, uuid: &Uuid) -> Result<(), Error> {
        use crate::schema::pages::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::delete(pages.filter(uuid.eq(uuid))).execute(&mut conn)?;

        Ok(())
    }
}
