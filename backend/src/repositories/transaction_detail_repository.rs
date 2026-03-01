use crate::models::transactions_detail::TransactionDetail;

pub async fn create<'e, E>(
    executor: E,
    transaction_id: i64,
    title: String,
    description: Option<String>,
) -> anyhow::Result<TransactionDetail>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let detail = sqlx::query_as::<_, TransactionDetail>(
        r#"
        INSERT INTO transactions_detail (transaction_id, title, description)
        VALUES ($1, $2, $3)
        RETURNING id, transaction_id, title, description, created_at
        "#,
    )
    .bind(transaction_id)
    .bind(title)
    .bind(description)
    .fetch_one(executor)
    .await?;

    Ok(detail)
}

pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<TransactionDetail, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionDetail>(
        r#"
        SELECT id, transaction_id, title, description, created_at
        FROM transactions_detail
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}

pub async fn find_by_transaction_id<'e, E>(executor: E, transaction_id: i64) -> Result<TransactionDetail, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionDetail>(
        r#"
        SELECT id, transaction_id, title, description, created_at
        FROM transactions_detail
        WHERE transaction_id = $1
        "#,
    )
    .bind(transaction_id)
    .fetch_one(executor)
    .await
}

pub async fn update<'e, E>(
    executor: E,
    id: i64,
    title: Option<String>,
    description: Option<String>,
) -> Result<TransactionDetail, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionDetail>(
        r#"
        UPDATE transactions_detail
        SET 
            title = COALESCE($2, title),
            description = COALESCE($3, description)
        WHERE id = $1
        RETURNING id, transaction_id, title, description, created_at
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .fetch_one(executor)
    .await
}

pub async fn delete<'e, E>(executor: E, id: i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query("DELETE FROM transactions_detail WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}
