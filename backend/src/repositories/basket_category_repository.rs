use crate::models::baskets_category::BasketCategory;


pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<BasketCategory, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketCategory>(
        r#"
        SELECT id, name, description, created_at
        FROM basket_category
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
) -> Result<Vec<BasketCategory>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketCategory>(
        r#"
        SELECT id, name, description, created_at
        FROM basket_category
        ORDER BY name ASC
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
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM basket_category")
        .fetch_one(executor)
        .await?;
    Ok(result.0)
}
