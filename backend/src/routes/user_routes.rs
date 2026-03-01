// use axum::routing::{get, post, delete};
// use axum::Router;
// use crate::handlers::user_handler;

// pub fn user_routes() -> Router<crate::state::AppState> {
//     Router::new()
//         .route("/users", get(user_handler::list_users))
//         .route("/users", post(user_handler::create_user))
//         .route("/users/{id}", get(user_handler::get_user))
//         .route("/users/{id}", delete(user_handler::delete_user))
// }
