use crate::schema::slots;
use ::uuid::Uuid;
use async_graphql::SimpleObject;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::page::Page;

#[derive(
    Associations,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    SimpleObject,
    Queryable,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = slots)]
#[diesel(belongs_to(Page, foreign_key = page_uuid))]
pub struct Slot {
    /// The page to which this slot belongs.
    pub page_uuid: Uuid,
    /// The slot's unique identifier.
    pub uuid: Uuid,
    /// The slot's order.
    pub order: String,
}
