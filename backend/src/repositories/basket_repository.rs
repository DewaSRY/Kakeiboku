use crate::models::baskets::{Basket, BasketWithBalance};
use sqlx::Row;

pub async fn create<'e, E>(
    executor: E,
    auth_user_id: i64,
    user_id: i64,
    name: String,
    description: Option<String>,
    basket_category_id: i64,
    basket_type: String,
) -> anyhow::Result<Basket>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let basket = sqlx::query_as::<_, Basket>(
        r#"
        INSERT INTO baskets (user_id, name, description, basket_category_id, type, created_by_id ,status)
        VALUES ($1, $2, $3, $4, $5, $6, 'active')
        RETURNING id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(description)
    .bind(basket_category_id)
    .bind(basket_type)
    .bind(auth_user_id)
    .fetch_one(executor)
    .await?;

    Ok(basket)
}

pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<Basket, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Basket>(
        r#"
        SELECT id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        FROM baskets
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}

pub async fn find_by_id_with_balance<'e, E>(
    executor: E,
    id: i64,
) -> Result<BasketWithBalance, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketWithBalance>(
        r#"
        SELECT 
            b.id, b.user_id, b.name, b.description, b.basket_category_id, 
            b.type, b.status, b.created_at, b.updated_at,
            COALESCE(
                (SELECT SUM(CASE WHEN t.to_basket_id = b.id THEN t.amount ELSE 0 END) -
                        SUM(CASE WHEN t.from_basket_id = b.id THEN t.amount ELSE 0 END)
                 FROM transactions t
                 WHERE t.from_basket_id = b.id OR t.to_basket_id = b.id), 0
            )::float8 as balance
        FROM baskets b
        WHERE b.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}

pub async fn count_user_baskets<'e, E>(executor: E, user_id: i64) -> Result<i64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let row = sqlx::query("SELECT COUNT(*) as count FROM baskets WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(executor)
        .await?;

    Ok(row.get("count"))
}

pub async fn find_all_by_user_id_with_balance<'e, E>(
    executor: E,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<BasketWithBalance>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketWithBalance>(
        r#"
        SELECT 
            b.id, b.user_id, b.name, b.description, b.basket_category_id, 
            b.type, b.status, b.created_at, b.updated_at,
            COALESCE(
                (SELECT SUM(CASE WHEN t.to_basket_id = b.id THEN t.amount ELSE 0 END) -
                        SUM(CASE WHEN t.from_basket_id = b.id THEN t.amount ELSE 0 END)
                 FROM transactions t
                 WHERE t.from_basket_id = b.id OR t.to_basket_id = b.id), 0
            )::float8 as balance
        FROM baskets b
        WHERE b.user_id = $1
        ORDER BY b.type DESC, b.created_at ASC
         LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn update<'e, E>(
    executor: E,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
) -> Result<Basket, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Basket>(
        r#"
        UPDATE baskets
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            status = COALESCE($5, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(status)
    .fetch_one(executor)
    .await
}
