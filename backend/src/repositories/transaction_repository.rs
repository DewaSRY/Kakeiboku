use crate::models::transactions::Transaction;
use chrono::NaiveDateTime;

pub async fn create<'e, E>(
    executor: E,
    created_by_id: i64,
    from_basket_id: Option<i64>,
    to_basket_id: i64,
    amount: f64,
    transaction_type_id: i64,
) -> anyhow::Result<Transaction>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at
        "#,
    )
    .bind(created_by_id)
    .bind(from_basket_id)
    .bind(to_basket_id)
    .bind(amount)
    .bind(transaction_type_id)
    .fetch_one(executor)
    .await?;

    Ok(transaction)
}

pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<Transaction, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at
        FROM transactions
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}

pub async fn find_by_basket_id<'e, E>(
    executor: E,
    basket_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Transaction>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at
        FROM transactions
        WHERE from_basket_id = $1 OR to_basket_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(basket_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn find_by_user_id<'e, E>(
    executor: E,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Transaction>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT t.id, t.created_by_id, t.from_basket_id, t.to_basket_id, t.amount, t.transaction_type_id, t.created_at
        FROM transactions t
        JOIN baskets b ON (t.from_basket_id = b.id OR t.to_basket_id = b.id)
        WHERE b.user_id = $1
        GROUP BY t.id
        ORDER BY t.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn find_by_transaction_type<'e, E>(
    executor: E,
    basket_id: i64,
    transaction_type_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Transaction>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at
        FROM transactions
        WHERE (from_basket_id = $1 OR to_basket_id = $1) AND transaction_type_id = $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(basket_id)
    .bind(transaction_type_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn find_by_date_range<'e, E>(
    executor: E,
    basket_id: i64,
    from_date: NaiveDateTime,
    to_date: NaiveDateTime,
    limit: i64,
    offset: i64,
) -> Result<Vec<Transaction>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at
        FROM transactions
        WHERE (from_basket_id = $1 OR to_basket_id = $1) 
          AND created_at >= $2 AND created_at <= $3
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(basket_id)
    .bind(from_date)
    .bind(to_date)
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn count_by_basket_id<'e, E>(executor: E, basket_id: i64) -> Result<i64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM transactions
        WHERE from_basket_id = $1 OR to_basket_id = $1
        "#,
    )
    .bind(basket_id)
    .fetch_one(executor)
    .await?;

    Ok(row.0)
}
