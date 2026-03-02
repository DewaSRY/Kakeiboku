use axum::http::StatusCode;

use crate::dtos::basket_dto::{BasketResponse, CreateBasketPayload, UpdateBasketPayload};
use crate::dtos::common_dto::CommonErrorResponse;
use crate::repositories::basket_repository;

pub async fn create_basket<'e, E>(
    executor: E,
    user_id: i64,
    payload: CreateBasketPayload,
) -> Result<BasketResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    // For main basket, check if user already has one
    if payload.basket_type == "main" {
        return Err(CommonErrorResponse::new(
            "Cannot manually create main basket. Main basket is created automatically.".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    let basket = basket_repository::create(
        executor,
        user_id,
        payload.name,
        payload.description,
        payload.basket_category_id,
        payload.basket_type,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create basket".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(BasketResponse {
        id: basket.id,
        user_id: basket.user_id,
        name: basket.name,
        description: basket.description,
        basket_category_id: basket.basket_category_id,
        basket_type: basket.basket_type,
        status: basket.status,
        balance: 0.0,
        created_at: basket.created_at,
        updated_at: basket.updated_at,
    })
}

pub async fn get_basket_by_id<'e, E>(
    executor: E,
    basket_id: i64,
    user_id: i64,
) -> Result<BasketResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let basket = basket_repository::find_by_id_with_balance(executor, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "You don't have permission to access this basket".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    Ok(BasketResponse {
        id: basket.id,
        user_id: basket.user_id,
        name: basket.name,
        description: basket.description,
        basket_category_id: basket.basket_category_id,
        basket_type: basket.basket_type,
        status: basket.status,
        balance: basket.balance,
        created_at: basket.created_at,
        updated_at: basket.updated_at,
    })
}

pub async fn get_all_user_baskets<'e, E>(
    executor: E,
    user_id: i64,
) -> Result<Vec<BasketResponse>, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let baskets = basket_repository::find_all_by_user_id_with_balance(executor, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve baskets".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(baskets
        .into_iter()
        .map(|b| BasketResponse {
            id: b.id,
            user_id: b.user_id,
            name: b.name,
            description: b.description,
            basket_category_id: b.basket_category_id,
            basket_type: b.basket_type,
            status: b.status,
            balance: b.balance,
            created_at: b.created_at,
            updated_at: b.updated_at,
        })
        .collect())
}

pub async fn update_basket(
    pool: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
    payload: UpdateBasketPayload,
) -> Result<BasketResponse, CommonErrorResponse> {
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

    // Validate status if provided
    if let Some(ref status) = payload.status {
        if !["active", "frozen", "archived"].contains(&status.as_str()) {
            return Err(CommonErrorResponse::new(
                "Invalid status. Must be 'active', 'frozen', or 'archived'".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }
    }

    let basket = basket_repository::update(
        pool,
        basket_id,
        payload.name,
        payload.description,
        payload.basket_category_id,
        payload.status,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to update basket".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(BasketResponse {
        id: basket.id,
        user_id: basket.user_id,
        name: basket.name,
        description: basket.description,
        basket_category_id: basket.basket_category_id,
        basket_type: basket.basket_type,
        status: basket.status,
        balance: 0.0, // Balance needs to be calculated separately
        created_at: basket.created_at,
        updated_at: basket.updated_at,
    })
}
