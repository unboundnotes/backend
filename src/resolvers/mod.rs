use async_graphql::MergedObject;

use self::{
    user::{UserMutation, UserQuery},
    workspace::{WorkspaceMutation, WorkspaceQuery},
};

pub mod user;
pub mod workspace;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, WorkspaceQuery);

#[derive(MergedObject, Default)]
pub struct MutationsRoot(UserMutation, WorkspaceMutation);
