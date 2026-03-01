use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::dtos::basket_dto::{
    BasketCategoryResponse, BasketResponse, CreateBasketCategoryPayload, CreateBasketPayload,
    DepositToMainBasketPayload, TransferToBranchPayload, UpdateBasketPayload,
};
use crate::dtos::common_dto::{CommonErrorResponse, CommonResponse};
use crate::services::basket_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

// ============ Basket Handlers ============

#[utoipa::path(
    post,
    path = "/baskets",
    request_body = CreateBasketPayload,
    responses(
        (status = 201, description = "Basket created successfully", body = BasketResponse),
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
) -> Result<(StatusCode, Json<BasketResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::create_basket(&state.pool, user_id, payload)
        .await
        .map(|basket| (StatusCode::CREATED, Json(basket)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/baskets",
    responses(
        (status = 200, description = "List of user baskets", body = Vec<BasketResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_all_baskets(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<BasketResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_all_user_baskets(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/baskets/main",
    responses(
        (status = 200, description = "Main basket details", body = BasketResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_main_basket(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<BasketResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_or_create_main_basket(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/baskets/branches",
    responses(
        (status = 200, description = "List of branch baskets", body = Vec<BasketResponse>),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_branch_baskets(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<BasketResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_branch_baskets(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/baskets/{basket_id}",
    params(
        ("basket_id" = i64, Path, description = "Basket ID")
    ),
    responses(
        (status = 200, description = "Basket details", body = BasketResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn get_basket_by_id(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(basket_id): Path<i64>,
) -> Result<Json<BasketResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_basket_by_id(&state.pool, basket_id, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    put,
    path = "/baskets/{basket_id}",
    params(
        ("basket_id" = i64, Path, description = "Basket ID")
    ),
    request_body = UpdateBasketPayload,
    responses(
        (status = 200, description = "Basket updated successfully", body = BasketResponse),
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
) -> Result<Json<BasketResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::update_basket(&state.pool, basket_id, user_id, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    delete,
    path = "/baskets/{basket_id}",
    params(
        ("basket_id" = i64, Path, description = "Basket ID")
    ),
    responses(
        (status = 200, description = "Basket deleted successfully", body = CommonResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn delete_basket(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(basket_id): Path<i64>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::delete_basket(&state.pool, basket_id, user_id)
        .await
        .map(|_| Json(CommonResponse::new("Basket deleted successfully".to_string(), StatusCode::OK)))
        .map_err(|err| err.to_response())
}

// ============ Money Operations ============

#[utoipa::path(
    post,
    path = "/baskets/deposit",
    request_body = DepositToMainBasketPayload,
    responses(
        (status = 200, description = "Deposit successful", body = BasketResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn deposit_to_main_basket(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<DepositToMainBasketPayload>,
) -> Result<Json<BasketResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::deposit_to_main_basket(&state.pool, user_id, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    post,
    path = "/baskets/transfer",
    request_body = TransferToBranchPayload,
    responses(
        (status = 200, description = "Transfer successful", body = BasketResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 403, description = "Forbidden", body = CommonErrorResponse),
        (status = 404, description = "Not found", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "baskets"
)]
pub async fn transfer_to_branch(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<TransferToBranchPayload>,
) -> Result<Json<BasketResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::transfer_to_branch(&state.pool, user_id, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

// ============ Basket Category Handlers (Admin) ============

#[utoipa::path(
    post,
    path = "/admin/basket-categories",
    request_body = CreateBasketCategoryPayload,
    responses(
        (status = 201, description = "Basket category created successfully", body = BasketCategoryResponse),
        (status = 400, description = "Bad request", body = CommonErrorResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn create_basket_category(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Json(payload): Json<CreateBasketCategoryPayload>,
) -> Result<(StatusCode, Json<BasketCategoryResponse>), (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::create_basket_category(&state.pool, payload)
        .await
        .map(|category| (StatusCode::CREATED, Json(category)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/basket-categories",
    responses(
        (status = 200, description = "List of basket categories", body = Vec<BasketCategoryResponse>),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    tag = "baskets"
)]
pub async fn get_all_basket_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<BasketCategoryResponse>>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::get_all_basket_categories(&state.pool)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    delete,
    path = "/admin/basket-categories/{category_id}",
    params(
        ("category_id" = i64, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category deleted successfully", body = CommonResponse),
        (status = 401, description = "Unauthorized", body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn delete_basket_category(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Path(category_id): Path<i64>,
) -> Result<Json<CommonResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    basket_service::delete_basket_category(&state.pool, category_id)
        .await
        .map(|_| Json(CommonResponse::new("Category deleted successfully".to_string(), StatusCode::OK)))
        .map_err(|err| err.to_response())
}
