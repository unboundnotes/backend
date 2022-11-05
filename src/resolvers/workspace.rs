use std::{io::Read, path::Path, sync::Arc};
use thiserror::Error;

use async_graphql::{
    ComplexObject, Context, ErrorExtensions, FieldError, InputObject, Object, Result, Upload,
};
use uuid::Uuid;

use crate::{
    models::{Page, Workspace},
    repos::traits::{ImagesRepo, WorkspaceRepo},
    utils::{
        img::generate_image,
        types::{InputError, WithError},
    },
};

#[derive(Debug, Error)]
pub enum WorkspaceMutationError {
    #[error("generic error: {0}")]
    DefaultError(#[from] anyhow::Error),
    #[error("Invalid image extension: {0}")]
    InvalidImageExtension(String),
}

impl ErrorExtensions for WorkspaceMutationError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            Self::DefaultError(err) => {
                e.set("code", 500);
                e.set("message", err.to_string());
            }
            _ => e.set("code", 400),
        })
    }
}

#[derive(Default)]
pub struct WorkspaceQuery;

#[Object]
impl WorkspaceQuery {
    pub async fn get_all_workspaces(&self, ctx: &Context<'_>) -> Vec<Workspace> {
        let repo = ctx.data_unchecked::<Arc<dyn WorkspaceRepo>>();
        repo.get_all_workspaces().await.unwrap()
    }

    pub async fn get_workspace(&self, ctx: &Context<'_>, uuid: Uuid) -> Option<Workspace> {
        let workspace_repo = ctx.data_unchecked::<Arc<dyn WorkspaceRepo>>();
        workspace_repo.get_workspace_by_uuid(&uuid).await.unwrap()
    }
}

#[derive(InputObject)]
pub struct CreateWorkspaceInput {
    name: String,
    image: Option<Upload>,
}

#[derive(Default)]
pub struct WorkspaceMutation;

#[Object]
impl WorkspaceMutation {
    pub async fn create_workspace(
        &self,
        ctx: &Context<'_>,
        workspace: CreateWorkspaceInput,
    ) -> Result<WithError<Workspace>> {
        let workspace_repo = ctx.data_unchecked::<Arc<dyn WorkspaceRepo>>();
        let s3_images_repo = ctx.data_unchecked::<Arc<dyn ImagesRepo>>();
        let workspace_uuid = Uuid::new_v4();

        if workspace.name.is_empty() {
            return Ok(WithError {
                errors: vec![InputError {
                    field: "name".to_string(),
                    message: "Name is required".to_string(),
                }],
                value: None,
            });
        }

        let workspace_image: String = match workspace.image {
            Some(image) => {
                let mut image = image.value(ctx)?;
                let image_extension = Path::new(&image.filename)
                    .extension()
                    .ok_or(WorkspaceMutationError::InvalidImageExtension(
                        "".to_string(),
                    ))?
                    .to_str()
                    .unwrap();

                // TODO: check if image_extension is valid

                // Upload image to S3
                let image_name =
                    format!("images/workspaces/{}.{}", workspace_uuid, image_extension);
                let mut buf = &mut Vec::new();
                image.content.read_to_end(&mut buf)?;

                s3_images_repo.upload_image(&image_name, &buf).await?
            }
            None => {
                let image_name = format!("images/workspaces/{}.png", workspace_uuid);
                let data = generate_image(&workspace.name).await;
                s3_images_repo.upload_image(&image_name, &data).await?
            }
        };
        let workspace = Workspace::new(&workspace.name, &workspace_image);
        workspace_repo.create_workspace(&workspace).await?;
        Ok(workspace.into())
    }

    pub async fn delete_workspace(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<bool> {
        let workspace_repo = ctx.data_unchecked::<Arc<dyn WorkspaceRepo>>();
        let s3_images_repo = ctx.data_unchecked::<Arc<dyn ImagesRepo>>();
        let workspace = workspace_repo.get_workspace_by_uuid(&uuid).await?;
        if let Some(workspace) = workspace {
            s3_images_repo.delete_image(&workspace.image).await?;
            workspace_repo.delete_workspace(&workspace.uuid).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[ComplexObject]
impl Workspace {
    pub async fn pages(&self, ctx: &Context<'_>) -> Result<Vec<Page>> {
        let workspace_repo = ctx.data_unchecked::<Arc<dyn WorkspaceRepo>>();
        workspace_repo
            .get_pages(&self.uuid)
            .await
            .map_err(|err| err.into())
    }
}
