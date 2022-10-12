use appconfig_derive::AppConfig;

#[derive(Clone, AppConfig)]
pub struct Config {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub jwt_secret: String,
    pub bind_addr: String,
}

// impl Config {
//     pub fn new() -> Self {
//         dotenv::dotenv().ok();
//         let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
//         let mongo_db = env::var("MONGO_DB").expect("MONGO_DB must be set");
//         let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//         Self {
//             mongo_uri,
//             mongo_db,
//             jwt_secret,
//             bind_addr: "0.0.0.0:8000".to_string(),
//         }
//     }
// }
