use axum::{Json, extract::{Query, State}, http::StatusCode};

use crate::dtos::common_dto::CommonErrorResponse;
use crate::dtos::dashboard_dto::{BranchSummaryResponse, DateRangeQuery, UserMoneyStashResponse};
use crate::services::dashboard_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    get,
    path = "/user/dashboard/money-stash",
    summary = "Get user money stash summary",
    description = "Returns the user's main branch balance, total spend, total save, and branch category percentages",
    responses(
        (status = 200, description = "Money stash data retrieved successfully", body = UserMoneyStashResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "dashboard"
)]
pub async fn get_money_stash(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserMoneyStashResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    dashboard_service::get_user_money_stash(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/user/dashboard/branch-summary",
    summary = "Get branch summary with balances",
    description = "Returns all branches with their current balance. Optionally filter by date range.",
    params(DateRangeQuery),
    responses(
        (status = 200, description = "Branch summary retrieved successfully", body = BranchSummaryResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "dashboard"
)]
pub async fn get_branch_summary(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<BranchSummaryResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    dashboard_service::get_branch_summary(&state.pool, user_id, query.start_date, query.end_date)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}
