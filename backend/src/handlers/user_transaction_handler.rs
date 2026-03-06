use axum::{Json, extract::State, http::StatusCode};

use crate::dtos::common_dto::CommonErrorResponse;
use crate::dtos::transaction_dto::{
    AllocatePayload, CreateTransactionPayload, DepositPayload, SpendPayload, TransactionResponse,
};
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
    post,
    path = "/user/transactions/deposit",
    summary = "Deposit money into the user's main basket",
    description = "Creates a deposit transaction. Money comes from an external source and goes into the user's main basket.",
    request_body = DepositPayload,
    responses(
        (status = 201, description = "Deposit created successfully", body = TransactionResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 404, description = "Main basket not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn deposit(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<DepositPayload>,
) -> Result<(StatusCode, Json<TransactionResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::deposit_transaction(&state.pool, user_id, payload)
        .await
        .map(|transaction| (StatusCode::CREATED, Json(transaction)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    post,
    path = "/user/transactions/allocate",
    summary = "Allocate money from main basket to a branch basket",
    description = "Creates an allocation transaction. Money moves from the user's main basket to a specified branch basket.",
    request_body = AllocatePayload,
    responses(
        (status = 201, description = "Allocation created successfully", body = TransactionResponse),
        (status = 400, description = "Bad request (insufficient balance, inactive basket, etc.)", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Target basket does not belong to you", body = CommonErrorResponse),
        (status = 404, description = "Basket or transaction type not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn allocate(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<AllocatePayload>,
) -> Result<(StatusCode, Json<TransactionResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::allocate_transaction(&state.pool, user_id, payload)
        .await
        .map(|transaction| (StatusCode::CREATED, Json(transaction)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    post,
    path = "/user/transactions/spend",
    summary = "Spend money from a branch basket",
    description = "Creates a spend transaction. Money leaves the specified branch basket (spent externally).",
    request_body = SpendPayload,
    responses(
        (status = 201, description = "Spend transaction created successfully", body = TransactionResponse),
        (status = 400, description = "Bad request (insufficient balance, inactive basket, etc.)", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Source basket does not belong to you", body = CommonErrorResponse),
        (status = 404, description = "Basket or transaction type not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn spend(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<SpendPayload>,
) -> Result<(StatusCode, Json<TransactionResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::spend_transaction(&state.pool, user_id, payload)
        .await
        .map(|transaction| (StatusCode::CREATED, Json(transaction)))
        .map_err(|err| err.to_response())
}
