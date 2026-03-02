use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::dtos::common_dto::{CommonErrorResponse, PaginatedResponse, PaginationQuery};
use crate::dtos::{
    basket_dto::{BasketDetailResponse, BasketResponse, CreateBasketPayload, UpdateBasketPayload},
    common_dto::CommonResponse,
};
use crate::services::basket_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    post,
    path = "/user/baskets",
    summary = "Create a new basket for the user",
    request_body = CreateBasketPayload,
    responses(
        (status = 201, description = "Basket created successfully", body = CommonResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn create_basket(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateBasketPayload>,
) -> Result<(StatusCode, Json<CommonResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::create_basket(&state.pool, user_id, payload)
        .await
        .map(|response| response.to_response())
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/user/baskets",
    summary = "Get all baskets for the authenticated user",
    responses(
        (status = 200, description = "List of user baskets", body = CommonResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
       params(
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("page" = Option<i64>, Query, description = "Page number")
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_all_baskets(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<BasketResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_paginate_baskets(&state.pool, user_id, pagination.limit, pagination.page)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/user/baskets/{basket_id}",
    summary = "Get a basket by ID for the authenticated user",
    params(
        ("basket_id" = i64, Path, description = "Basket ID")
    ),
    responses(
        (status = 200, description = "Basket retrieved successfully", body = BasketDetailResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(basket_id): Path<i64>,
) -> Result<Json<BasketDetailResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_by_id(&state.pool, basket_id, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    put,
    path = "/user/baskets/{basket_id}",
    summary = "Update a basket for the authenticated user",
    params(
        ("basket_id" = i64, Path, description = "Basket ID")
    ),
    request_body = UpdateBasketPayload,
    responses(
        (status = 200, description = "Basket updated successfully", body = CommonResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn update_basket(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(basket_id): Path<i64>,
    Json(payload): Json<UpdateBasketPayload>,
) -> Result<(StatusCode, Json<CommonResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::update_basket(&state.pool, basket_id, user_id, payload)
        .await
        .map(|response| response.to_response())
        .map_err(|err| err.to_response())
}
