use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::transaction_handler;
use crate::state::AppState;

pub fn transaction_routes() -> Router<AppState> {
    Router::new()
        // User transaction routes
        .route(
            "/transactions",
            post(transaction_handler::create_transaction),
        )
        .route(
            "/transactions",
            get(transaction_handler::get_user_transactions),
        )
        .route(
            "/transactions/{transaction_id}",
            get(transaction_handler::get_transaction_by_id),
        )
        .route(
            "/baskets/{basket_id}/transactions",
            get(transaction_handler::get_basket_transactions),
        )
        // Public transaction type routes
        .route(
            "/transaction-types",
            get(transaction_handler::get_all_transaction_types),
        )
        .route(
            "/transaction-types/flat",
            get(transaction_handler::get_flat_transaction_types),
        )
        // Admin transaction type routes
        .route(
            "/admin/transaction-types",
            post(transaction_handler::create_transaction_type),
        )
        .route(
            "/admin/transaction-types/{type_id}",
            put(transaction_handler::update_transaction_type),
        )
        .route(
            "/admin/transaction-types/{type_id}",
            delete(transaction_handler::delete_transaction_type),
        )
}
