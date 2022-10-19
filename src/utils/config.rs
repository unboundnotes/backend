use appconfig_derive::*;

fn generate_jwt() -> String {
    "token".to_string()
}

#[derive(AppConfig)]
pub struct BaseConfig {
    #[appconfig(default = "0.0.0.0:8000")]
    pub bind_addr: String,
    pub postgres_url: String,
}

#[derive(AppConfig)]
pub struct Config {
    #[appconfig(skip)]
    pub base: BaseConfig,
    #[appconfig(default_fn = generate_jwt)]
    pub jwt_secret: String,
}
