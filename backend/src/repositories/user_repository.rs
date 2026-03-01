use crate::models::user::User;
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    first_name: String,
    last_name: String,
    hashed_password: String,
    email: String,
) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, first_name, last_name, email, created_at -- List all fields needed for User struct
        "#,
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(hashed_password)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, first_name, last_name, email, created_at, updated_at, password
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, first_name, last_name, email, created_at, updated_at 
        FROM users"#,
    )
    .fetch_all(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
