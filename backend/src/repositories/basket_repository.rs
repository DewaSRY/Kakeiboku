use crate::models::baskets::{Basket, BasketWithBalance};
use sqlx::Row;

pub async fn create<'e, E>(
    executor: E,
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
        INSERT INTO baskets (user_id, name, description, basket_category_id, type, status)
        VALUES ($1, $2, $3, $4, $5, 'active')
        RETURNING id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(description)
    .bind(basket_category_id)
    .bind(basket_type)
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

pub async fn find_user_main_basket<'e, E>(executor: E, user_id: i64) -> Result<Basket, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Basket>(
        r#"
        SELECT id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        FROM baskets
        WHERE user_id = $1 AND type = 'main'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_one(executor)
    .await
}

pub async fn find_user_main_basket_with_balance<'e, E>(
    executor: E,
    user_id: i64,
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
        WHERE b.user_id = $1 AND b.type = 'main'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_one(executor)
    .await
}

pub async fn find_all_by_user_id<'e, E>(
    executor: E,
    user_id: i64,
) -> Result<Vec<Basket>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Basket>(
        r#"
        SELECT id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        FROM baskets
        WHERE user_id = $1
        ORDER BY type DESC, created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
}

pub async fn find_all_by_user_id_with_balance<'e, E>(
    executor: E,
    user_id: i64,
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
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
}

pub async fn find_branch_baskets_by_user<'e, E>(
    executor: E,
    user_id: i64,
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
        WHERE b.user_id = $1 AND b.type = 'branch'
        ORDER BY b.created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
}

pub async fn update<'e, E>(
    executor: E,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    basket_category_id: Option<i64>,
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
            basket_category_id = COALESCE($4, basket_category_id),
            status = COALESCE($5, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(basket_category_id)
    .bind(status)
    .fetch_one(executor)
    .await
}

pub async fn update_status<'e, E>(
    executor: E,
    id: i64,
    status: String,
) -> Result<Basket, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Basket>(
        r#"
        UPDATE baskets
        SET status = $2, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, user_id, name, description, basket_category_id, type, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(status)
    .fetch_one(executor)
    .await
}

pub async fn delete<'e, E>(executor: E, id: i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query("DELETE FROM baskets WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn get_balance<'e, E>(executor: E, basket_id: i64) -> Result<f64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let row = sqlx::query(
        r#"
        SELECT COALESCE(
            SUM(CASE WHEN to_basket_id = $1 THEN amount ELSE 0 END) -
            SUM(CASE WHEN from_basket_id = $1 THEN amount ELSE 0 END), 0
        )::float8 as balance
        FROM transactions
        WHERE from_basket_id = $1 OR to_basket_id = $1
        "#,
    )
    .bind(basket_id)
    .fetch_one(executor)
    .await?;

    Ok(row.get("balance"))
}

pub async fn check_ownership<'e, E>(
    executor: E,
    basket_id: i64,
    user_id: i64,
) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let row = sqlx::query(
        r#"
        SELECT EXISTS(SELECT 1 FROM baskets WHERE id = $1 AND user_id = $2) as exists
        "#,
    )
    .bind(basket_id)
    .bind(user_id)
    .fetch_one(executor)
    .await?;

    Ok(row.get("exists"))
}
