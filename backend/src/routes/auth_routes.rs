use axum::routing::{ post};
use axum::Router;
use crate::handlers::auth_handler;

pub fn auth_routes() -> Router<crate::state::AppState> {
    Router::new()
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/login", post(auth_handler::login))

}