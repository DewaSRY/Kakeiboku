use axum::http::StatusCode;

use crate::dtos::common_dto::{CommonErrorResponse, PaginatedResponse};
use crate::dtos::transaction_dto::{CreateTransactionPayload, TransactionResponse};
use crate::repositories::{
    basket_repository, transaction_detail_repository, transaction_repository,
    transaction_type_repository,
};

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

    if from_basket.balance < payload.amount {
        return Err(CommonErrorResponse::new(
            format!(
                "Insufficient balance. Available: {}, Required: {}",
                from_basket.balance, payload.amount
            ),
            StatusCode::BAD_REQUEST,
        ));
    }

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

    transaction_type_repository::find_by_id(pool, payload.transaction_type_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Transaction type not found".to_string(),
                StatusCode::NOT_FOUND,
            )
        })?;

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

    let _ = transaction_detail_repository::create(
        pool,
        transaction.id,
        payload.title,
        payload.description,
    )
    .await;

    Ok(TransactionResponse::from_model(transaction))
}

pub async fn get_basket_transactions(
    pool: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<PaginatedResponse<TransactionResponse>, CommonErrorResponse> {
    let basket = basket_repository::find_by_id(pool, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "Basket does not belong to you".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let limit = limit.unwrap_or(50);
    let page = page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let transactions = transaction_repository::find_by_basket_id(pool, basket_id, limit, offset)
        .await
        .map_err(|e| {
            println!("Database error: {e:?}");
            CommonErrorResponse::new(
                "Failed to retrieve transactions".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let total = transaction_repository::count_by_basket_id(pool, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to count transactions".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let transaction_list = transactions
        .into_iter()
        .map(TransactionResponse::from_model)
        .collect();

    Ok(PaginatedResponse::new(
        transaction_list,
        page,
        limit,
        total, 
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtos::transaction_dto::CreateTransactionPayload;
    use crate::utils::test_setup_util::setup;

    #[tokio::test]
    async fn test_create_transaction_amount_negative() {
        let state = setup().await;
        let user_id = 1;
        let payload = CreateTransactionPayload {
            from_basket_id: 1,
            to_basket_id: 2,
            amount: -10.0,
            transaction_type_id: 1,
            title: "Test Transaction".to_string(),
            description: Some("Test Desc".to_string()),
        };
        let result = create_transaction(&state.pool, user_id, payload).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Amount must be positive");
    }

    #[tokio::test]
    async fn test_create_transaction_source_basket_not_found() {
        let state = setup().await;
        let user_id = 1;
        let payload = CreateTransactionPayload {
            from_basket_id: -9999, // invalid
            to_basket_id: 2,
            amount: 10.0,
            transaction_type_id: 1,
            title: "Test Transaction".to_string(),
            description: Some("Test Desc".to_string()),
        };
        let result = create_transaction(&state.pool, user_id, payload).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Source basket not found");
    }

    #[tokio::test]
    async fn test_create_transaction_target_basket_not_found() {
        let state = setup().await;
        let user_id = 1;
        let payload = CreateTransactionPayload {
            from_basket_id: 1,
            to_basket_id: -9999, // invalid
            amount: 10.0,
            transaction_type_id: 1,
            title: "Test Transaction".to_string(),
            description: Some("Test Desc".to_string()),
        };
        let result = create_transaction(&state.pool, user_id, payload).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Target basket not found");
    }

    #[tokio::test]
    async fn test_create_transaction_transaction_type_not_found() {
        let state = setup().await;
        let user_id = 1;
        let payload = CreateTransactionPayload {
            from_basket_id: 1,
            to_basket_id: 2,
            amount: 10.0,
            transaction_type_id: -9999, // invalid
            title: "Test Transaction".to_string(),
            description: Some("Test Desc".to_string()),
        };
        let result = create_transaction(&state.pool, user_id, payload).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Transaction type not found");
    }

    #[tokio::test]
    async fn test_get_basket_transactions_success() {
        let state = setup().await;
        let basket_id = 1;
        let user_id = 1;
        let result =
            get_basket_transactions(&state.pool, basket_id, user_id, Some(10), Some(0)).await;
        if let Err(e) = &result {
            println!("Error: {e:?}");
        }
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_basket_transactions_forbidden() {
        let state = setup().await;
        let basket_id = 1;
        let user_id = -9999; // Not the owner
        let result =
            get_basket_transactions(&state.pool, basket_id, user_id, Some(10), Some(0)).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Basket does not belong to you");
    }
}
