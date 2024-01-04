use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{Page, User, Workspace};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_user_by_uuid(&self, uuid: &Uuid) -> Result<Option<User>>;
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn get_user_by_login(&self, login: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait WorkspaceRepo: Send + Sync {
    async fn get_all_workspaces(&self) -> Result<Vec<Workspace>>;
    async fn get_workspace_by_uuid(&self, uuid: &Uuid) -> Result<Option<Workspace>>;
    async fn create_workspace(&self, workspace: &Workspace) -> Result<()>;
    async fn update_workspace(&self, workspace: &Workspace) -> Result<()>;
    async fn delete_workspace(&self, uuid: &Uuid) -> Result<()>;
    async fn get_pages(&self, uuid: &Uuid) -> Result<Vec<Page>>;
}

#[async_trait]
pub trait PageRepo: Send + Sync {
    async fn get_page_by_uuid(&self, uuid: &Uuid) -> Result<Option<Page>>;
    async fn create_page(&self, page: &Page) -> Result<()>;
    async fn update_page(&self, page: &Page) -> Result<()>;
    async fn delete_page(&self, uuid: &Uuid) -> Result<()>;
}

#[async_trait]
pub trait ImagesRepo: Send + Sync {
    async fn upload_image(&self, path: &str, image: &[u8]) -> Result<String>;
    async fn delete_image(&self, path: &str) -> Result<()>;
}
