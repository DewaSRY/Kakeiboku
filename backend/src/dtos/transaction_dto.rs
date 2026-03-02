use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ============ Transaction DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTransactionPayload {
    pub from_basket_id: i64,
    pub to_basket_id: i64,
    pub amount: f64,
    pub transaction_type_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub id: i64,
    pub created_by_id: i64,
    pub from_basket_id: Option<i64>, // NULL for external deposits
    pub to_basket_id: i64,
    pub amount: f64,
    pub transaction_type_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionWithDetails {
    pub id: i64,
    pub created_by_id: i64,
    pub from_basket: TransactionBasketInfo,
    pub to_basket: TransactionBasketInfo,
    pub amount: f64,
    pub transaction_type: TransactionTypeInfo,
    pub detail: Option<TransactionDetailResponse>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionBasketInfo {
    pub id: i64,
    pub name: String,
    pub basket_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionTypeInfo {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionHistoryQuery {
    pub basket_id: Option<i64>,
    pub transaction_type_id: Option<i64>,
    pub from_date: Option<NaiveDateTime>,
    pub to_date: Option<NaiveDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============ Transaction Type DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTransactionTypePayload {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTransactionTypePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionTypeChildrenResponse {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionTypeResponse {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub children: Vec<TransactionTypeChildrenResponse>,
    pub created_at: NaiveDateTime,
}

// ============ Transaction Detail DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTransactionDetailPayload {
    pub transaction_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTransactionDetailPayload {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionDetailResponse {
    pub id: i64,
    pub transaction_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
