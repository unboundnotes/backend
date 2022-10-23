use crate::schema::workspaces;
use ::uuid::Uuid;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, SimpleObject, Queryable, Insertable, AsChangeset,
)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    /// The workspace's unique identifier.
    pub uuid: Uuid,

    /// The workspace's name.
    pub name: String,

    /// The workspace's image or icon.
    pub image: String,
}
impl Workspace {
    pub(crate) fn new(name: &str, image: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            image: image.to_string(),
        }
    }
}
