use std::{path::Path, sync::Arc};

use async_graphql::{Context, InputObject, Object, Result, Upload};
use std::io::Read;
use uuid::Uuid;

use crate::{
    models::Page,
    repos::traits::{ImagesRepo, PageRepo},
    utils::types::WithError,
};

#[derive(Default)]
pub struct PageMutation;

#[derive(InputObject)]
pub struct CreatePageInput {
    pub name: String,
    pub workspace_uuid: Uuid,
    pub image: Option<Upload>,
}

#[Object]
impl PageMutation {
    pub async fn create_page(
        &self,
        ctx: &Context<'_>,
        page: CreatePageInput,
    ) -> Result<WithError<Page>> {
        let page_repo = ctx.data_unchecked::<Arc<dyn PageRepo>>();
        let images_repo = ctx.data_unchecked::<Arc<dyn ImagesRepo>>();

        let image = match page.image {
            Some(image) => {
                let mut image = image.value(ctx)?;
                let image_extension = Path::new(&image.filename)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let image_name = format!(
                    "images/{}/{}.{}",
                    page.workspace_uuid,
                    Uuid::new_v4(),
                    image_extension
                );
                let mut buf = &mut Vec::new();
                image.content.read_to_end(&mut buf)?;
                let image = images_repo.upload_image(&image_name, buf).await?;
                Some(image)
            }
            None => None,
        };

        let page = Page::new(page.workspace_uuid, page.name, image);
        page_repo.create_page(&page).await?;
        Ok(WithError {
            errors: vec![],
            value: Some(page),
        })
    }
}
