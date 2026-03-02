use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::dtos::common_dto::{CommonErrorResponse, PaginationQuery};
use crate::dtos::transaction_dto::{CreateTransactionPayload, TransactionResponse};
use crate::services::transaction_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    post,
    path = "/user/transactions",
    summary = "Create a new transaction for the authenticated user",
    request_body = CreateTransactionPayload,
    responses(
        (status = 201, description = "Transaction created successfully", body = TransactionResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn create_transaction(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateTransactionPayload>,
) -> Result<(StatusCode, Json<TransactionResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::create_transaction(&state.pool, user_id, payload)
        .await
        .map(|transaction| (StatusCode::CREATED, Json(transaction)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/user/baskets/{basket_id}/transactions",
    summary = "Get transactions for a specific basket of the authenticated user",
    params(
        ("basket_id" = i64, Path, description = "Basket ID"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("offset" = Option<i64>, Query, description = "Number of results to skip")
    ),
    responses(
        (status = 200, description = "List of basket transactions", body = Vec<TransactionResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn get_basket_transactions(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(basket_id): Path<i64>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<TransactionResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_basket_transactions(
        &state.pool,
        basket_id,
        user_id,
        pagination.limit,
        pagination.offset,
    )
    .await
    .map(Json)
    .map_err(|err| err.to_response())
}
