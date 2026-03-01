use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

use chrono::NaiveDateTime;

#[derive(Serialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
