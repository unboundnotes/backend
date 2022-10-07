use async_graphql::MergedObject;

use self::user::{UserMutation, UserQuery};

pub mod user;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationsRoot(UserMutation);
