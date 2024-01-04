use anyhow::Error;
use async_trait::async_trait;
use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::models::{Page, Workspace};

use super::traits::WorkspaceRepo;

pub struct PostgresqlWorkspaceRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresqlWorkspaceRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkspaceRepo for PostgresqlWorkspaceRepo {
    async fn get_all_workspaces(&self) -> Result<Vec<Workspace>, Error> {
        use crate::schema::workspaces::dsl::*;

        let mut conn = self.pool.get()?;
        let result = workspaces.load::<Workspace>(&mut conn)?;
        Ok(result)
    }

    async fn get_workspace_by_uuid(&self, uuid_val: &Uuid) -> Result<Option<Workspace>, Error> {
        use crate::schema::workspaces::dsl::*;

        let mut conn = self.pool.get()?;
        let result = workspaces
            .filter(uuid.eq(uuid_val))
            .first::<Workspace>(&mut conn)
            .optional()?;
        Ok(result)
    }

    async fn create_workspace(&self, workspace: &Workspace) -> Result<(), Error> {
        use crate::schema::workspaces::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::insert_into(workspaces)
            .values(workspace)
            .execute(&mut conn)?;
        Ok(())
    }

    async fn update_workspace(&self, workspace: &Workspace) -> Result<(), Error> {
        use crate::schema::workspaces::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::update(workspaces)
            .filter(uuid.eq(&workspace.uuid))
            .set(workspace)
            .execute(&mut conn)?;
        Ok(())
    }

    async fn delete_workspace(&self, uuid_val: &Uuid) -> Result<(), Error> {
        use crate::schema::workspaces::dsl::*;

        let mut conn = self.pool.get()?;
        diesel::delete(workspaces.filter(uuid.eq(uuid_val))).execute(&mut conn)?;
        Ok(())
    }

    async fn get_pages(&self, uuid_val: &Uuid) -> Result<Vec<Page>, Error> {
        use crate::schema::pages::dsl::*;

        let mut conn = self.pool.get()?;
        pages
            .filter(workspace_uuid.eq(uuid_val))
            .load::<Page>(&mut conn)
            .map_err(|e| e.into())
    }
}
