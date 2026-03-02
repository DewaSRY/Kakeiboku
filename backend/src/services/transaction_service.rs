use axum::http::StatusCode;

use crate::dtos::common_dto::CommonErrorResponse;
use crate::dtos::transaction_dto::{
    CreateTransactionPayload, CreateTransactionTypePayload, TransactionBasketInfo,
    TransactionDetailResponse, TransactionResponse, TransactionTypeChildrenResponse,
    TransactionTypeInfo, TransactionTypeResponse, TransactionWithDetails,
    UpdateTransactionTypePayload,
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

pub async fn get_transaction_by_id(
    pool: &sqlx::PgPool,
    transaction_id: i64,
    user_id: i64,
) -> Result<TransactionWithDetails, CommonErrorResponse> {
    let transaction = transaction_repository::find_by_id(pool, transaction_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Transaction not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    // Verify user has access to this transaction
    let from_basket = match transaction.from_basket_id {
        Some(id) => basket_repository::find_by_id(pool, id).await.ok(),
        None => None,
    };
    let to_basket = basket_repository::find_by_id(pool, transaction.to_basket_id)
        .await
        .ok();

    // User has access if they own either the from or to basket
    let has_access = match (&from_basket, &to_basket) {
        (Some(from), Some(to)) => from.user_id == user_id || to.user_id == user_id,
        (Some(from), None) => from.user_id == user_id,
        (None, Some(to)) => to.user_id == user_id,
        _ => false,
    };

    if !has_access {
        return Err(CommonErrorResponse::new(
            "You don't have access to this transaction".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    // Build response
    let from_basket_info = from_basket
        .map(|b| TransactionBasketInfo {
            id: b.id,
            name: b.name,
            basket_type: b.basket_type,
        })
        .unwrap_or(TransactionBasketInfo {
            id: 0,
            name: "External".to_string(),
            basket_type: "external".to_string(),
        });

    let to_basket_info = to_basket
        .map(|b| TransactionBasketInfo {
            id: b.id,
            name: b.name,
            basket_type: b.basket_type,
        })
        .unwrap_or(TransactionBasketInfo {
            id: 0,
            name: "External".to_string(),
            basket_type: "external".to_string(),
        });

    let transaction_type =
        transaction_type_repository::find_by_id(pool, transaction.transaction_type_id)
            .await
            .map(|t| TransactionTypeInfo {
                id: t.id,
                name: t.name,
                parent_id: t.parent_id,
            })
            .unwrap_or(TransactionTypeInfo {
                id: 0,
                name: "Unknown".to_string(),
                parent_id: None,
            });

    let detail = transaction_detail_repository::find_by_transaction_id(pool, transaction_id)
        .await
        .ok()
        .map(|d| TransactionDetailResponse {
            id: d.id,
            transaction_id: d.transaction_id,
            title: d.title,
            description: d.description,
            created_at: d.created_at,
        });

    Ok(TransactionWithDetails {
        id: transaction.id,
        created_by_id: transaction.created_by_id,
        from_basket: from_basket_info,
        to_basket: to_basket_info,
        amount: transaction.amount,
        transaction_type,
        detail,
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

pub async fn get_user_transactions(
    pool: &sqlx::PgPool,
    user_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<TransactionResponse>, CommonErrorResponse> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let transactions = transaction_repository::find_by_user_id(pool, user_id, limit, offset)
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

// ============ Transaction Type Operations (Admin) ============

pub async fn create_transaction_type<'e, E>(
    executor: E,
    payload: CreateTransactionTypePayload,
) -> Result<TransactionTypeResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let transaction_type = transaction_type_repository::create(
        executor,
        payload.name,
        payload.description,
        payload.parent_id,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create transaction type".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(TransactionTypeResponse {
        id: transaction_type.id,
        parent_id: transaction_type.parent_id,
        name: transaction_type.name,
        description: transaction_type.description,
        children: vec![],
        created_at: transaction_type.created_at,
    })
}

pub async fn get_all_transaction_types(
    pool: &sqlx::PgPool,
) -> Result<Vec<TransactionTypeResponse>, CommonErrorResponse> {
    // Get root types
    let root_types = transaction_type_repository::find_root_types(pool)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve transaction types".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let mut result = Vec::new();

    for root in root_types {
        // Get children for each root
        let children = transaction_type_repository::find_children(pool, root.id)
            .await
            .unwrap_or_default();

        let children_response: Vec<TransactionTypeChildrenResponse> = children
            .into_iter()
            .map(|c| TransactionTypeChildrenResponse {
                id: c.id,
                parent_id: c.parent_id,
                name: c.name,
                description: c.description,
                created_at: c.created_at,
            })
            .collect();

        result.push(TransactionTypeResponse {
            id: root.id,
            parent_id: root.parent_id,
            name: root.name,
            description: root.description,
            children: children_response,
            created_at: root.created_at,
        });
    }

    Ok(result)
}

pub async fn get_flat_transaction_types<'e, E>(
    executor: E,
) -> Result<Vec<TransactionTypeResponse>, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let types = transaction_type_repository::find_all(executor)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve transaction types".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(types
        .into_iter()
        .map(|t| TransactionTypeResponse {
            id: t.id,
            parent_id: t.parent_id,
            name: t.name,
            description: t.description,
            children: vec![],
            created_at: t.created_at,
        })
        .collect())
}

pub async fn update_transaction_type<'e, E>(
    executor: E,
    type_id: i64,
    payload: UpdateTransactionTypePayload,
) -> Result<TransactionTypeResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let transaction_type = transaction_type_repository::update(
        executor,
        type_id,
        payload.name,
        payload.description,
        payload.parent_id,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to update transaction type".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(TransactionTypeResponse {
        id: transaction_type.id,
        parent_id: transaction_type.parent_id,
        name: transaction_type.name,
        description: transaction_type.description,
        children: vec![],
        created_at: transaction_type.created_at,
    })
}

pub async fn delete_transaction_type<'e, E>(
    executor: E,
    type_id: i64,
) -> Result<(), CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    transaction_type_repository::delete(executor, type_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to delete transaction type. It may be in use.".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(())
}
