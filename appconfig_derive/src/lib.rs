use std::error::Error as StdError;
use thiserror::Error;

pub use appconfig_derive_impl::*;

pub trait AppConfig {}

pub trait DataSource {
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn StdError>>;
    fn set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn StdError>>;
}

#[derive(Error, Debug)]
pub enum AppConfigError {
    #[error("ParseError: {0}")]
    ParsingError(Box<dyn StdError>),
    #[error("Datastore error: {0}")]
    DatastoreError(Box<dyn StdError>),
}
