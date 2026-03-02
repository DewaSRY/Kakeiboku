use axum::http::StatusCode;

use crate::dtos::common_dto::CommonErrorResponse;
use crate::dtos::transaction_dto::{
    CreateTransactionPayload,
    TransactionResponse,
};
use crate::repositories::{
    basket_repository, transaction_detail_repository, transaction_repository,
    transaction_type_repository,
};

// ============ Transaction Operations ============

pub async fn create_transaction(
    pool: &sqlx::PgPool,
    user_id: i64,
    payload: CreateTransactionPayload,
) -> Result<TransactionResponse, CommonErrorResponse> {
    if payload.amount <= 0.0 {
        return Err(CommonErrorResponse::new(
            "Amount must be positive".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Verify from_basket ownership and status
    let from_basket = basket_repository::find_by_id_with_balance(pool, payload.from_basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Source basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if from_basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "Source basket does not belong to you".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if from_basket.status != "active" {
        return Err(CommonErrorResponse::new(
            "Source basket is not active".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Check sufficient balance
    if from_basket.balance < payload.amount {
        return Err(CommonErrorResponse::new(
            format!(
                "Insufficient balance. Available: {}, Required: {}",
                from_basket.balance, payload.amount
            ),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Verify to_basket ownership and status
    let to_basket = basket_repository::find_by_id_with_balance(pool, payload.to_basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Target basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if to_basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "Target basket does not belong to you".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if to_basket.status != "active" {
        return Err(CommonErrorResponse::new(
            "Target basket is not active".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Verify transaction type exists
    transaction_type_repository::find_by_id(pool, payload.transaction_type_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Transaction type not found".to_string(),
                StatusCode::NOT_FOUND,
            )
        })?;

    // Create the transaction
    let transaction = transaction_repository::create(
        pool,
        user_id,
        Some(payload.from_basket_id),
        payload.to_basket_id,
        payload.amount,
        payload.transaction_type_id,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create transaction".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // Create transaction detail
    let _ = transaction_detail_repository::create(
        pool,
        transaction.id,
        payload.title,
        payload.description,
    )
    .await;

    Ok(TransactionResponse {
        id: transaction.id,
        created_by_id: transaction.created_by_id,
        from_basket_id: transaction.from_basket_id,
        to_basket_id: transaction.to_basket_id,
        amount: transaction.amount,
        transaction_type_id: transaction.transaction_type_id,
        created_at: transaction.created_at,
    })
}

pub async fn get_basket_transactions(
    pool: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<TransactionResponse>, CommonErrorResponse> {
    // Verify basket ownership
    let basket = basket_repository::find_by_id(pool, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "You don't have access to this basket".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let transactions = transaction_repository::find_by_basket_id(pool, basket_id, limit, offset)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve transactions".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(transactions
        .into_iter()
        .map(|t| TransactionResponse {
            id: t.id,
            created_by_id: t.created_by_id,
            from_basket_id: t.from_basket_id,
            to_basket_id: t.to_basket_id,
            amount: t.amount,
            transaction_type_id: t.transaction_type_id,
            created_at: t.created_at,
        })
        .collect())
}
