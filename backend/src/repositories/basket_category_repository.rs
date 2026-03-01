use crate::models::baskets_category::BasketCategory;

pub async fn create<'e, E>(
    executor: E,
    name: String,
    description: Option<String>,
) -> anyhow::Result<BasketCategory>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    let category = sqlx::query_as::<_, BasketCategory>(
        r#"
        INSERT INTO basket_category (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at
        "#,
    )
    .bind(name)
    .bind(description)
    .fetch_one(executor)
    .await?;

    Ok(category)
}

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

pub async fn find_all<'e, E>(executor: E) -> Result<Vec<BasketCategory>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketCategory>(
        r#"
        SELECT id, name, description, created_at
        FROM basket_category
        ORDER BY name ASC
        "#,
    )
    .fetch_all(executor)
    .await
}

pub async fn update<'e, E>(
    executor: E,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<BasketCategory, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketCategory>(
        r#"
        UPDATE basket_category
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description)
        WHERE id = $1
        RETURNING id, name, description, created_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .fetch_one(executor)
    .await
}

pub async fn delete<'e, E>(executor: E, id: i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query("DELETE FROM basket_category WHERE id = $1")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn find_by_name<'e, E>(executor: E, name: &str) -> Result<BasketCategory, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BasketCategory>(
        r#"
        SELECT id, name, description, created_at
        FROM basket_category
        WHERE LOWER(name) = LOWER($1)
        "#,
    )
    .bind(name)
    .fetch_one(executor)
    .await
}
