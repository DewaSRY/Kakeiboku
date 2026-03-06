use crate::models::transactions_type::TransactionType;

pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<TransactionType, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        SELECT id, parent_id, name, description, created_at
        FROM transactions_type
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}


pub async fn find_paginated<'e, E>(
    executor: E,
    limit: i64,
    offset: i64,
) -> Result<Vec<TransactionType>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        SELECT id, parent_id, name, description, created_at
        FROM transactions_type
        ORDER BY parent_id NULLS FIRST, name ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(executor)
    .await
}

pub async fn count<'e, E>(executor: E) -> Result<i64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM transactions_type")
        .fetch_one(executor)
        .await?;
    Ok(result.0)
}
