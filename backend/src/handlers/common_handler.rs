use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::dtos::common_dto::{
    CommonErrorResponse, IdNameResponse, PaginatedResponse, PaginationQuery,
};
use crate::services::common_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    get,
    path = "/user/common/basket_category",
    summary = "Get paginated list of basket categories",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("page" = Option<i64>, Query, description = "Page number")
    ),
    responses(
        (status = 200, description = "List of basket categories", body = PaginatedResponse<IdNameResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "common"
)]
pub async fn get_basket_categories(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<IdNameResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    common_service::get_basket_categories(&state.pool, pagination.limit, pagination.page)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/user/common/transaction_type",
    summary = "Get paginated list of transaction types",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("page" = Option<i64>, Query, description = "Page number")
    ),
    responses(
        (status = 200, description = "List of transaction types", body = PaginatedResponse<IdNameResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "common"
)]
pub async fn get_transaction_types(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<IdNameResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    common_service::get_transaction_types(&state.pool, pagination.limit, pagination.page)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}
