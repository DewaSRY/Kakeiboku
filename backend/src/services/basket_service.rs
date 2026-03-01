use axum::http::StatusCode;

use crate::dtos::basket_dto::{
    BasketCategoryResponse, BasketResponse, CreateBasketCategoryPayload,
    CreateBasketPayload, DepositToMainBasketPayload, TransferToBranchPayload, UpdateBasketPayload,
};
use crate::dtos::common_dto::CommonErrorResponse;
use crate::repositories::{
    basket_category_repository, basket_repository, transaction_detail_repository,
    transaction_repository,
};


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
        CommonErrorResponse::new("Failed to create basket".to_string(), StatusCode::INTERNAL_SERVER_ERROR)
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

pub async fn get_or_create_main_basket<'e>(
    pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<BasketResponse, CommonErrorResponse> {
    // Try to find existing main basket
    match basket_repository::find_user_main_basket_with_balance(pool, user_id).await {
        Ok(basket) => Ok(BasketResponse {
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
        }),
        Err(_) => {
            // Create main basket if it doesn't exist
            // First, get or create a default category
            let default_category = match basket_category_repository::find_by_name(pool, "Default").await {
                Ok(category) => category,
                Err(_) => {
                    basket_category_repository::create(
                        pool,
                        "Default".to_string(),
                        Some("Default basket category".to_string()),
                    )
                    .await
                    .map_err(|_| {
                        CommonErrorResponse::new(
                            "Failed to create default category".to_string(),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    })?
                }
            };

            let basket = basket_repository::create(
                pool,
                user_id,
                "Main Basket".to_string(),
                Some("Your main wallet for deposits".to_string()),
                default_category.id,
                "main".to_string(),
            )
            .await
            .map_err(|_| {
                CommonErrorResponse::new(
                    "Failed to create main basket".to_string(),
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
    }
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

pub async fn get_branch_baskets<'e, E>(
    executor: E,
    user_id: i64,
) -> Result<Vec<BasketResponse>, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let baskets = basket_repository::find_branch_baskets_by_user(executor, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve branch baskets".to_string(),
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
        .map_err(|_| CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND))?;

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

pub async fn delete_basket(
    pool: &sqlx::PgPool,
    basket_id: i64,
    user_id: i64,
) -> Result<(), CommonErrorResponse> {
    let basket = basket_repository::find_by_id_with_balance(pool, basket_id)
        .await
        .map_err(|_| CommonErrorResponse::new("Basket not found".to_string(), StatusCode::NOT_FOUND))?;

    if basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "You don't have permission to delete this basket".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if basket.basket_type == "main" {
        return Err(CommonErrorResponse::new(
            "Cannot delete main basket".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    if basket.balance != 0.0 {
        return Err(CommonErrorResponse::new(
            "Cannot delete basket with non-zero balance. Transfer funds first.".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    basket_repository::delete(pool, basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to delete basket".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(())
}

// ============ Money Operations ============

pub async fn deposit_to_main_basket(
    pool: &sqlx::PgPool,
    user_id: i64,
    payload: DepositToMainBasketPayload,
) -> Result<BasketResponse, CommonErrorResponse> {
    if payload.amount <= 0.0 {
        return Err(CommonErrorResponse::new(
            "Amount must be positive".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Get or create main basket
    let main_basket = get_or_create_main_basket(pool, user_id).await?;

    if main_basket.status != "active" {
        return Err(CommonErrorResponse::new(
            "Main basket is not active".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Get or create deposit transaction type
    let deposit_type = match crate::repositories::transaction_type_repository::find_by_id(pool, 1).await {
        Ok(t) => t,
        Err(_) => {
            crate::repositories::transaction_type_repository::create(
                pool,
                "Deposit".to_string(),
                Some("External deposit to main basket".to_string()),
                None,
            )
            .await
            .map_err(|_| {
                CommonErrorResponse::new(
                    "Failed to create deposit type".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?
        }
    };

    // Create transaction (from_basket_id = None indicates external deposit)
    let transaction = transaction_repository::create(
        pool,
        user_id,
        None, // External source
        main_basket.id,
        payload.amount,
        deposit_type.id,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create deposit transaction".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // Create transaction detail
    if let Some(description) = payload.description {
        let _ = transaction_detail_repository::create(
            pool,
            transaction.id,
            "Deposit".to_string(),
            Some(description),
        )
        .await;
    }

    // Return updated basket with new balance
    get_basket_by_id(pool, main_basket.id, user_id).await
}

pub async fn transfer_to_branch(
    pool: &sqlx::PgPool,
    user_id: i64,
    payload: TransferToBranchPayload,
) -> Result<BasketResponse, CommonErrorResponse> {
    if payload.amount <= 0.0 {
        return Err(CommonErrorResponse::new(
            "Amount must be positive".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Get main basket
    let main_basket = basket_repository::find_user_main_basket_with_balance(pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Main basket not found. Please deposit money first.".to_string(),
                StatusCode::NOT_FOUND,
            )
        })?;

    if main_basket.status != "active" {
        return Err(CommonErrorResponse::new(
            "Main basket is not active".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Check sufficient balance
    if main_basket.balance < payload.amount {
        return Err(CommonErrorResponse::new(
            format!(
                "Insufficient balance. Available: {}, Required: {}",
                main_basket.balance, payload.amount
            ),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Check target basket exists and belongs to user
    let target_basket = basket_repository::find_by_id_with_balance(pool, payload.to_basket_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new("Target basket not found".to_string(), StatusCode::NOT_FOUND)
        })?;

    if target_basket.user_id != user_id {
        return Err(CommonErrorResponse::new(
            "Target basket does not belong to you".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if target_basket.basket_type != "branch" {
        return Err(CommonErrorResponse::new(
            "Can only transfer to branch baskets".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    if target_basket.status != "active" {
        return Err(CommonErrorResponse::new(
            "Target basket is not active".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Create transaction
    let transaction = transaction_repository::create(
        pool,
        user_id,
        Some(main_basket.id),
        payload.to_basket_id,
        payload.amount,
        payload.transaction_type_id,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create transfer transaction".to_string(),
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

    // Return updated target basket
    get_basket_by_id(pool, payload.to_basket_id, user_id).await
}

// ============ Basket Category Operations ============

pub async fn create_basket_category<'e, E>(
    executor: E,
    payload: CreateBasketCategoryPayload,
) -> Result<BasketCategoryResponse, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let category = basket_category_repository::create(executor, payload.name, payload.description)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to create basket category".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(BasketCategoryResponse {
        id: category.id,
        name: category.name,
        description: category.description,
        created_at: category.created_at,
    })
}

pub async fn get_all_basket_categories<'e, E>(
    executor: E,
) -> Result<Vec<BasketCategoryResponse>, CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let categories = basket_category_repository::find_all(executor)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve basket categories".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(categories
        .into_iter()
        .map(|c| BasketCategoryResponse {
            id: c.id,
            name: c.name,
            description: c.description,
            created_at: c.created_at,
        })
        .collect())
}

pub async fn delete_basket_category<'e, E>(
    executor: E,
    category_id: i64,
) -> Result<(), CommonErrorResponse>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    basket_category_repository::delete(executor, category_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to delete basket category. It may be in use.".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(())
}
