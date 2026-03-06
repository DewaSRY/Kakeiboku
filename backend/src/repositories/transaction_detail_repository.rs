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



