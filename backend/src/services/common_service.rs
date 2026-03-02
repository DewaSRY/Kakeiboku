use axum::http::StatusCode;

use crate::dtos::common_dto::{CommonErrorResponse, IdNameResponse, PaginatedResponse};
use crate::repositories::{basket_category_repository, transaction_type_repository};

pub async fn get_basket_categories(
    pool: &sqlx::PgPool,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<PaginatedResponse<IdNameResponse>, CommonErrorResponse> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let offset = (page - 1) * limit;

    let categories = basket_category_repository::find_paginated(pool, limit, offset)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve basket categories".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let total = basket_category_repository::count(pool).await.map_err(|_| {
        CommonErrorResponse::new(
            "Failed to count basket categories".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let data = categories
        .into_iter()
        .map(|c| IdNameResponse {
            id: c.id,
            name: c.name,
        })
        .collect();

    Ok(PaginatedResponse::new(data, page, limit, total))
}

pub async fn get_transaction_types(
    pool: &sqlx::PgPool,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<PaginatedResponse<IdNameResponse>, CommonErrorResponse> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let offset = (page - 1) * limit;

    let types = transaction_type_repository::find_paginated(pool, limit, offset)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve transaction types".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let total = transaction_type_repository::count(pool)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to count transaction types".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let data = types
        .into_iter()
        .map(|t| IdNameResponse {
            id: t.id,
            name: t.name,
        })
        .collect();

    Ok(PaginatedResponse::new(data, page, limit, total))
}
