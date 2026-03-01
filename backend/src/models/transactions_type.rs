use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::NaiveDateTime;

#[derive(Serialize, FromRow, ToSchema)]
pub struct TransactionType {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
