use sqlx::PgPool;
use crate::models::user::User;

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn create(pool: &PgPool, name: String, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"
    )
    .bind(name)
    .bind(email)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}