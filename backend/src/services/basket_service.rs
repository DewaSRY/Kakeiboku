use axum::http::StatusCode;
use sqlx::PgPool;

use crate::dtos::basket_dto::{
    BasketDetailResponse, BasketResponse, CreateBasketPayload, UpdateBasketPayload,
};
use crate::dtos::common_dto::{CommonErrorResponse, CommonResponse, PaginatedResponse};
use crate::repositories::{basket_category_repository, basket_repository};

pub async fn create_basket<'e, E>(
    executor: E,
    user_id: i64,
    payload: CreateBasketPayload,
) -> Result<CommonResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    basket_repository::create(
        executor,
        user_id,
        user_id,
        payload.name,
        payload.description,
        payload.basket_category_id,
        "branch".to_string(),
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create basket".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(CommonResponse::new(
        "Basket created successfully".to_string(),
        StatusCode::CREATED,
    ))
}

pub async fn get_paginate_baskets(
    executor: &PgPool,
    user_id: i64,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<PaginatedResponse<BasketResponse>, CommonErrorResponse> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let offset = (page - 1) * limit;

    let baskets =
        basket_repository::find_all_by_user_id_with_balance(executor, user_id, limit, offset)
            .await
            .map_err(|_| {
                CommonErrorResponse::new(
                    "Failed to retrieve baskets".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;

    let total = basket_repository::count_user_baskets(executor, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to count baskets".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let basket_list = baskets
        .into_iter()
        .map(BasketResponse::from_model)
        .collect();
    Ok(PaginatedResponse::new(basket_list, page, limit, total))
}

pub async fn update_basket(
    pool: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
    payload: UpdateBasketPayload,
) -> Result<CommonResponse, CommonErrorResponse> {
    // Check ownership first
    let existing = basket_repository::find_by_id(pool, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if existing.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "You don't have permission to update this basket".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if let Some(ref status) = payload.status {
        if !["active", "frozen", "archived"].contains(&status.as_str()) {
            return Err(CommonErrorResponse::new(
                "Invalid status. Must be 'active', 'frozen', or 'archived'".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }
    }

    basket_repository::update(
        pool,
        basket_id,
        payload.name,
        payload.description,
        payload.status,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to update basket".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(CommonResponse::new(
        "Basket updated successfully".to_string(),
        StatusCode::OK,
    ))
}

pub async fn get_by_id(
    executor: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
) -> Result<BasketDetailResponse, CommonErrorResponse> {
    let basket = basket_repository::find_by_id_with_balance(executor, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "You don't have permission to view this basket".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let basket_category =
        basket_category_repository::find_by_id(executor, basket.basket_category_id)
            .await
            .map_err(|_| {
                CommonErrorResponse::new(
                    "Basket category not found".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;

    Ok(BasketDetailResponse::from_model(basket, basket_category))
}
