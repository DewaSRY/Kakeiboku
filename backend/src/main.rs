mod configs;
mod dtos;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod state;
mod utils;

use anyhow::{Context, Result};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self},
    prelude::*,
};

use routes::{auth_routes, swagger_router};
use state::AppState;

use crate::configs::app_config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_ansi(true))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,tower_http=warn")),
        )
        .init();

    info!("Starting Kakeiboku backend ...");

    match dotenvy::dotenv() {
        Ok(path) => info!("Loaded .env from: {}", path.display()),
        Err(e) => error!("No .env loaded (continuing): {}", e),
    }

    let app_config = app_config::AppConfig::from_env();

    let safe_url = mask_db_url(&app_config.database_url);
    info!("Database URL: {}", safe_url);

    let pool = PgPoolOptions::new()
        .max_connections(app_config.db_max_connections)
        .acquire_timeout(std::time::Duration::from_secs(
            app_config.db_acquire_timeout_secs,
        ))
        .connect(&app_config.database_url)
        .await
        .context("Failed to create pool")?;

    info!("Connected to database successfully");

    let app = Router::new()
        .merge(swagger_router::swagger_routes())
        .merge(auth_routes::auth_routes())
        .with_state(AppState {
            pool,
            jwt_secret: app_config.jwt_secret.clone(),
        });

    let addr: SocketAddr = format!("{}:{}", app_config.host, app_config.port)
        .parse()
        .context("Failed to parse server address")?;

    info!("Starting server on {}", addr);
    info!(
        "swagger-ui available at http://{}:{}/swagger-ui",
        app_config.host, app_config.port
    );

    let listener = TcpListener::bind(addr)
        .await
        .context("Failed to bind to address")?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Failed to start server")?;

    Ok(())
}

async fn shutdown_signal() {
    let cntrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = cntrl_c => {info!("Received Ctrl+C signal")},
        _ = terminate => {info!("Received SIGTERM signal")},
    };
}

fn mask_db_url(url: &str) -> String {
    if let Ok(mut parsed) = url::Url::parse(url) {
        if !parsed.password().is_none() {
            match parsed.set_password(Some("***")) {
                Ok(_) => (),
                Err(_) => error!("Failed to mask password in DB URL: {}", url),
            }
        }
        if !parsed.username().is_empty() {
            parsed.set_username("***").ok();
        }
        parsed.to_string()
    } else {
        "[invalid url format]".to_string()
    }
}
