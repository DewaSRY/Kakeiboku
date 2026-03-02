use crate::handlers::auth_handler;
use axum::Router;
use axum::routing::{get, post};

pub fn auth_routes() -> Router<crate::state::AppState> {
    Router::new()
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/profile", get(auth_handler::get_profile))
}
