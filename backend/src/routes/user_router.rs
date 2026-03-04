use axum::{
    Router,
    routing::{get, post, put},
};

use crate::handlers::{
    basket_transaction_handle, common_handler, dashboard_handler, user_basket_handler,
    user_transaction_handler,
};

pub fn user_routes() -> Router<crate::state::AppState> {
    let basket_routes = Router::new()
        .route("/", post(user_basket_handler::create_basket))
        .route("/", get(user_basket_handler::get_all_baskets))
        .route("/{basket_id}", get(user_basket_handler::get_by_id))
        .route("/{basket_id}", put(user_basket_handler::update_basket))
        .route(
            "/{basket_id}/transactions",
            get(basket_transaction_handle::get_basket_transactions),
        );

    let transaction_routes =
        Router::new().route("/", post(user_transaction_handler::create_transaction));

    let common_routes = Router::new()
        .route(
            "/basket_category",
            get(common_handler::get_basket_categories),
        )
        .route(
            "/transaction_type",
            get(common_handler::get_transaction_types),
        );

    let dashboard_routes = Router::new()
        .route("/money-stash", get(dashboard_handler::get_money_stash))
        .route("/branch-summary", get(dashboard_handler::get_branch_summary));

    Router::new()
        .nest("/user/baskets", basket_routes)
        .nest("/user/transactions", transaction_routes)
        .nest("/user/common", common_routes)
        .nest("/user/dashboard", dashboard_routes)
}
