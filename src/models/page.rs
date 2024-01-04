use ::uuid::Uuid;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::pages;

use super::workspace::Workspace;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    SimpleObject,
    Queryable,
    Insertable,
    AsChangeset,
    Associations,
)]
#[diesel(table_name = pages)]
#[diesel(belongs_to(Workspace, foreign_key = workspace_uuid))]
pub struct Page {
    /// The workspace to which this page belongs.
    pub workspace_uuid: Uuid,

    /// The page's unique identifier.
    pub uuid: Uuid,

    /// The page's title.
    pub title: String,

    /// The page's image or icon.
    pub image: Option<String>,
}

impl Page {
    pub fn new(workspace_uuid: Uuid, title: String, image: Option<String>) -> Self {
        Self {
            workspace_uuid,
            uuid: Uuid::new_v4(),
            title,
            image,
        }
    }
}
