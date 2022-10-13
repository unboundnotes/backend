use std::error::Error as StdError;
use thiserror::Error;

pub use appconfig_derive_impl::*;

pub trait AppConfig {}

pub trait DataSource {
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn StdError>>;
    fn set(&mut self, key: &str, value: String) -> Result<(), Box<dyn StdError>>;
}

pub struct NopDataSource;

impl DataSource for NopDataSource {
    fn get(&self, _key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    fn set(&mut self, _key: &str, _value: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum AppConfigError {
    #[error("ParseError: {0}")]
    ParsingError(Box<dyn StdError>),
    #[error("Datastore error: {0}")]
    DatastoreError(#[from] Box<dyn StdError>),
    #[error("Field not set error: {0}")]
    FieldNotSetError(String),
}
