mod state;
mod models;
mod dtos;
mod repositories;
mod services;
mod handlers;
mod routes;

use axum::Router;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

use state::AppState;
use routes::user_routes;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let app = Router::new()
        .merge(user_routes::user_routes())
        .with_state(AppState { pool });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}