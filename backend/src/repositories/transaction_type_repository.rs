use crate::models::transactions_type::TransactionType;

pub async fn create<'e, E>(
    executor: E,
    name: String,
    description: Option<String>,
    parent_id: Option<i64>,
) -> anyhow::Result<TransactionType>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let transaction_type = sqlx::query_as::<_, TransactionType>(
        r#"
        INSERT INTO transactions_type (name, description, parent_id)
        VALUES ($1, $2, $3)
        RETURNING id, parent_id, name, description, created_at
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(parent_id)
    .fetch_one(executor)
    .await?;

    Ok(transaction_type)
}

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

pub async fn find_all<'e, E>(executor: E) -> Result<Vec<TransactionType>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        SELECT id, parent_id, name, description, created_at
        FROM transactions_type
        ORDER BY parent_id NULLS FIRST, name ASC
        "#,
    )
    .fetch_all(executor)
    .await
}

pub async fn find_root_types<'e, E>(executor: E) -> Result<Vec<TransactionType>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        SELECT id, parent_id, name, description, created_at
        FROM transactions_type
        WHERE parent_id IS NULL
        ORDER BY name ASC
        "#,
    )
    .fetch_all(executor)
    .await
}

pub async fn find_children<'e, E>(
    executor: E,
    parent_id: i64,
) -> Result<Vec<TransactionType>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        SELECT id, parent_id, name, description, created_at
        FROM transactions_type
        WHERE parent_id = $1
        ORDER BY name ASC
        "#,
    )
    .bind(parent_id)
    .fetch_all(executor)
    .await
}

pub async fn update<'e, E>(
    executor: E,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<i64>,
) -> Result<TransactionType, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, TransactionType>(
        r#"
        UPDATE transactions_type
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            parent_id = COALESCE($4, parent_id)
        WHERE id = $1
        RETURNING id, parent_id, name, description, created_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(parent_id)
    .fetch_one(executor)
    .await
}

pub async fn delete<'e, E>(executor: E, id: i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query("DELETE FROM transactions_type WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn has_children<'e, E>(executor: E, id: i64) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let row: (bool,) = sqlx::query_as(
        r#"
        SELECT EXISTS(SELECT 1 FROM transactions_type WHERE parent_id = $1) as has_children
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await?;

    Ok(row.0)
}
