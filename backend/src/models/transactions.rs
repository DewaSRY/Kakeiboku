use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Transaction {
    pub id: i64,
    pub created_by_id: i64,
    pub from_basket_id: Option<i64>, // NULL for external deposits
    pub to_basket_id: Option<i64>,   // NULL for spending (money leaves system)
    pub amount: f64,
    pub transaction_type_id: i64,
    pub created_at: NaiveDateTime,
}
