use crate::{configs::app_config, state::AppState};
use sqlx::postgres::PgPoolOptions;

#[warn(unused)]
pub async fn setup() -> AppState {
    dotenvy::dotenv().expect("Failed to load .env file");
    let app_config = app_config::AppConfig::from_env();

    let pool = PgPoolOptions::new()
        .connect(&app_config.database_url)
        .await
        .expect("Failed to connect to test DB");

    AppState {
        pool,
        jwt_secret: app_config.jwt_secret,
    }
}
