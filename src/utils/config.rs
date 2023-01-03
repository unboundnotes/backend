use appconfig_derive::*;

/// Used to generate a jwt secret when the app is first loaded
fn generate_jwt_secret() -> String {
    // TODO: Implement random generation
    "token".to_string()
}

#[derive(AppConfig)]
pub struct BaseConfig {
    #[appconfig(default = "0.0.0.0:8000")]
    pub bind_addr: String,
    pub database_url: String,
}

#[derive(AppConfig)]
pub struct Config {
    #[appconfig(skip)]
    pub base: BaseConfig,
    #[appconfig(default_fn = generate_jwt_secret)]
    pub jwt_secret: String,
    pub s3_bucket: String,
    pub s3_endpoint: String,
}
