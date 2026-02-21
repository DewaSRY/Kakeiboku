use sqlx::PgPool;
use crate::repositories::user_repository;
use crate::models::user::User;

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::find_all(pool).await
}

pub async fn get_user(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    user_repository::find_by_id(pool, id).await
}

pub async fn create_user(
    pool: &PgPool,
    name: String,
    email: String,
) -> Result<User, sqlx::Error> {
    user_repository::create(pool, name, email).await
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    user_repository::delete(pool, id).await
}