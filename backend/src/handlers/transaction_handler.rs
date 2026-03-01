use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::dtos::common_dto::{CommonErrorResponse, CommonResponse};
use crate::dtos::transaction_dto::{
    CreateTransactionPayload, CreateTransactionTypePayload, TransactionResponse,
    TransactionTypeResponse, TransactionWithDetails, UpdateTransactionTypePayload,
};
use crate::services::transaction_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============ Transaction Handlers ============

#[utoipa::path(
    post,
    path = "/transactions",
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
    path = "/transactions/{transaction_id}",
    params(
        ("transaction_id" = i64, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction details", body = TransactionWithDetails),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn get_transaction_by_id(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(transaction_id): Path<i64>,
) -> Result<Json<TransactionWithDetails>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_transaction_by_id(&state.pool, transaction_id, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/transactions",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("offset" = Option<i64>, Query, description = "Number of results to skip")
    ),
    responses(
        (status = 200, description = "List of user transactions", body = Vec<TransactionResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
pub async fn get_user_transactions(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<TransactionResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_user_transactions(&state.pool, user_id, pagination.limit, pagination.offset)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/baskets/{basket_id}/transactions",
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

// ============ Transaction Type Handlers (Admin) ============

#[utoipa::path(
    post,
    path = "/admin/transaction-types",
    request_body = CreateTransactionTypePayload,
    responses(
        (status = 201, description = "Transaction type created successfully", body = TransactionTypeResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn create_transaction_type(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Json(payload): Json<CreateTransactionTypePayload>,
) -> Result<(StatusCode, Json<TransactionTypeResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::create_transaction_type(&state.pool, payload)
        .await
        .map(|t| (StatusCode::CREATED, Json(t)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/transaction-types",
    responses(
        (status = 200, description = "List of transaction types (hierarchical)", body = Vec<TransactionTypeResponse>),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    tag = "transactions"
)]
pub async fn get_all_transaction_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<TransactionTypeResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_all_transaction_types(&state.pool)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/transaction-types/flat",
    responses(
        (status = 200, description = "Flat list of all transaction types", body = Vec<TransactionTypeResponse>),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    tag = "transactions"
)]
pub async fn get_flat_transaction_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<TransactionTypeResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_flat_transaction_types(&state.pool)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    put,
    path = "/admin/transaction-types/{type_id}",
    params(
        ("type_id" = i64, Path, description = "Transaction Type ID")
    ),
    request_body = UpdateTransactionTypePayload,
    responses(
        (status = 200, description = "Transaction type updated successfully", body = TransactionTypeResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn update_transaction_type(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Path(type_id): Path<i64>,
    Json(payload): Json<UpdateTransactionTypePayload>,
) -> Result<Json<TransactionTypeResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::update_transaction_type(&state.pool, type_id, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    delete,
    path = "/admin/transaction-types/{type_id}",
    params(
        ("type_id" = i64, Path, description = "Transaction Type ID")
    ),
    responses(
        (status = 200, description = "Transaction type deleted successfully", body = CommonResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn delete_transaction_type(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Path(type_id): Path<i64>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::delete_transaction_type(&state.pool, type_id)
        .await
        .map(|_| {
            Json(CommonResponse::new(
                "Transaction type deleted successfully".to_string(),
                StatusCode::OK,
            ))
        })
        .map_err(|err| err.to_response())
}
