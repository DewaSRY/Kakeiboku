use crate::models::user::User;

pub async fn create<'e, E>(
    executor: E,
    first_name: String,
    last_name: String,
    hashed_password: String,
    email: String,
) -> anyhow::Result<User>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>  + 'e,
{
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, first_name, last_name, email, password, created_at, updated_at
        "#,
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(hashed_password)
    .fetch_one(executor)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email<'e, E>(executor: E, email: &str) -> Result<User, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, first_name, last_name, email, created_at, updated_at, password
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(executor)
    .await
}



pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<User, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, first_name, last_name, email, created_at, updated_at, password
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(executor)
    .await
}

