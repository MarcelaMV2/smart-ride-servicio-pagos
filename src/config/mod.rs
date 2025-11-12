use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub mongo_col: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        let mongo_uri = env::var("MONGO_URI").expect("Falta MONGO_URI");
        let mongo_db = env::var("MONGO_DB").expect("Falta MONGO_DB");
        let mongo_col = env::var("MONGO_COL").unwrap_or_else(|_| "pagos".into());
        let port = env::var("SERVER_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        Self { mongo_uri, mongo_db, mongo_col, port }
    }
}
