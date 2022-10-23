use crate::schema::atoms;
use async_graphql::{Enum, SimpleObject};
use diesel::{
    backend::{self, Backend},
    deserialize::{self, FromSql},
    prelude::*,
    serialize::{self, Output, ToSql},
    sql_types::Text,
    AsExpression, FromSqlRow,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, IntoStaticStr};
use uuid::Uuid;

use super::slot::Slot;

#[derive(
    Enum,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    EnumString,
    IntoStaticStr,
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = Text)]
pub enum AtomType {
    Text,
}

impl<DB> ToSql<Text, DB> for AtomType
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        let s: &'b str = self.into();
        s.to_sql(out)
    }
}

impl<DB> FromSql<Text, DB> for AtomType
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: backend::RawValue<DB>) -> deserialize::Result<Self> {
        let data = String::from_sql(bytes)?;
        AtomType::from_str(&data)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

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
#[diesel(table_name = atoms)]
#[diesel(belongs_to(Slot, foreign_key = slot_uuid))]
pub struct Atom {
    /// The slot to which this atom belongs.
    pub slot_uuid: Uuid,
    /// The atom's index.
    pub idx: i32,
    /// The atom's type.
    pub typ: AtomType,
    /// The atom's data.
    pub data: Option<String>,
}
