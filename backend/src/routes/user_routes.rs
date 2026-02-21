use axum::routing::{get, post, delete};
use axum::Router;
use crate::handlers::user_handlers;

pub fn user_routes() -> Router<crate::state::AppState> {
    Router::new()
        .route("/users", get(user_handlers::list_users))
        .route("/users", post(user_handlers::create_user))
        .route("/users/:id", get(user_handlers::get_user))
        .route("/users/:id", delete(user_handlers::delete_user))
}