use crate::models::transactions::Transaction;

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
        RETURNING id, created_by_id, from_basket_id, to_basket_id, amount::FLOAT8 as amount, transaction_type_id, created_at
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
        SELECT id, created_by_id, from_basket_id, to_basket_id, amount::FLOAT8 as amount, transaction_type_id, created_at
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
