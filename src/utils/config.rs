use appconfig_derive::*;

#[derive(AppConfig)]
pub struct BaseConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub bind_addr: String,
}
