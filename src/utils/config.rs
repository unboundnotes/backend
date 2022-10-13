use appconfig_derive::*;

#[derive(AppConfig)]
pub struct BaseConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub bind_addr: String,
}

pub struct NopDataSource;

impl DataSource for NopDataSource {
    fn get(&self, _key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    fn set(&mut self, _key: &str, _value: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
