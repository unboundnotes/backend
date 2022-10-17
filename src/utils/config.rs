use appconfig_derive::*;

fn generate_jwt() -> String {
    "token".to_string()
}

#[derive(AppConfig)]
pub struct BaseConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub bind_addr: String,
    pub postgres_host: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
}

#[derive(AppConfig)]
pub struct Config {
    #[appconfig(skip)]
    pub base: BaseConfig,
    #[appconfig(default_fn = generate_jwt)]
    pub jwt_secret: String,
}
