use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct TransactionDetail {
    pub id: i64,
    pub transaction_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
