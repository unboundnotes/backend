use async_graphql::MergedObject;

use self::{
    page::PageMutation,
    user::{UserMutation, UserQuery},
    workspace::{WorkspaceMutation, WorkspaceQuery},
};

pub mod page;
pub mod user;
pub mod workspace;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, WorkspaceQuery);

#[derive(MergedObject, Default)]
pub struct MutationsRoot(UserMutation, WorkspaceMutation, PageMutation);
