use std::env;

pub struct AppConfig{
    pub port: u16,
    pub database_url: String,
    pub host: String,
    pub jwt_secret: String,
}


impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid u16");
        
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in environment variables");
        
        let host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".into());

        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set in environment variables");

        Self {
            port,
            database_url,
            host,
            jwt_secret
        }
    }
}