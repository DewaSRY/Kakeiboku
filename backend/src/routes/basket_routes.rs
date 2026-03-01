use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::basket_handler;
use crate::state::AppState;

pub fn basket_routes() -> Router<AppState> {
    Router::new()
        .route("/baskets", post(basket_handler::create_basket))
        .route("/baskets", get(basket_handler::get_all_baskets))
        .route("/baskets/main", get(basket_handler::get_main_basket))
        .route("/baskets/branches", get(basket_handler::get_branch_baskets))
        .route(
            "/baskets/deposit",
            post(basket_handler::deposit_to_main_basket),
        )
        .route(
            "/baskets/transfer",
            post(basket_handler::transfer_to_branch),
        )
        .route(
            "/baskets/{basket_id}",
            get(basket_handler::get_basket_by_id),
        )
        .route("/baskets/{basket_id}", put(basket_handler::update_basket))
        .route(
            "/baskets/{basket_id}",
            delete(basket_handler::delete_basket),
        )
        .route(
            "/basket-categories",
            get(basket_handler::get_all_basket_categories),
        )
        .route(
            "/admin/basket-categories",
            post(basket_handler::create_basket_category),
        )
        .route(
            "/admin/basket-categories/{category_id}",
            delete(basket_handler::delete_basket_category),
        )
}
