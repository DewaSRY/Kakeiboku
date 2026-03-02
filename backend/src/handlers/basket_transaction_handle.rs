use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::dtos::common_dto::{CommonErrorResponse, PaginatedResponse, PaginationQuery};
use crate::dtos::transaction_dto::{ TransactionResponse};
use crate::services::transaction_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    get,
    path = "/user/baskets/{basket_id}/transactions",
    summary = "Get transactions for a specific basket of the authenticated user",
    params(
        ("basket_id" = i64, Path, description = "Basket ID"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("page" = Option<i64>, Query, description = "Page number")
    ),
    responses(
        (status = 200, description = "List of basket transactions", body = PaginatedResponse<TransactionResponse>),
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
) -> Result<Json<PaginatedResponse<TransactionResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    transaction_service::get_basket_transactions(
        &state.pool,
        basket_id,
        user_id,
        pagination.limit,
        pagination.page,
    )
    .await
    .map(Json)
    .map_err(|err| err.to_response())
}
