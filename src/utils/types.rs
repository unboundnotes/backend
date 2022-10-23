use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
pub struct InputError {
    pub field: String,
    pub message: String,
}

#[derive(SimpleObject)]
pub struct WithError<T>
where
    T: Send + Sync + OutputType,
{
    pub errors: Vec<InputError>,
    pub value: Option<T>,
}

impl<T> From<T> for WithError<T>
where
    T: Send + Sync + OutputType,
{
    fn from(value: T) -> Self {
        Self {
            errors: Vec::new(),
            value: Some(value),
        }
    }
}
