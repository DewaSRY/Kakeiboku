use axum::{
    Json,
    extract::{ State},
    http::StatusCode,
};

use crate::dtos::common_dto::{CommonErrorResponse};
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

