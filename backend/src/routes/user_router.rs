use axum::{
    Router,
    routing::{get, post, put},
};

use crate::handlers::{user_basket_handler, user_transaction_handler};

pub fn user_routes() -> Router<crate::state::AppState> {
    let basket_routes = Router::new()
        .route("/", post(user_basket_handler::create_basket))
        .route("/", get(user_basket_handler::get_all_baskets))
        .route("/{basket_id}", get(user_basket_handler::get_by_id))
        .route("/{basket_id}", put(user_basket_handler::update_basket))
        .route(
            "/{basket_id}/transactions",
            get(user_transaction_handler::get_basket_transactions),
        );

    let transaction_routes =
        Router::new().route("/", post(user_transaction_handler::create_transaction));

    Router::new()
        .nest("/user/baskets", basket_routes)
        .nest("/user/transactions", transaction_routes)
}
