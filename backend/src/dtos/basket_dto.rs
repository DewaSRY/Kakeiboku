use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;

// ============ Basket DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBasketPayload {
    pub name: String,
    pub description: Option<String>,
    pub basket_category_id: i64,
    #[serde(default = "default_basket_type")]
    pub basket_type: String, // "main" or "branch"
}

fn default_basket_type() -> String {
    "branch".to_string()
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBasketPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub basket_category_id: Option<i64>,
    pub status: Option<String>, // "active", "frozen", "archived"
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub basket_category_id: i64,
    pub basket_type: String,
    pub status: String,
    pub balance: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketWithCategory {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub basket_category: BasketCategoryResponse,
    pub basket_type: String,
    pub status: String,
    pub balance: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DepositToMainBasketPayload {
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransferToBranchPayload {
    pub to_basket_id: i64,
    pub amount: f64,
    pub transaction_type_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketBalanceResponse {
    pub basket_id: i64,
    pub basket_name: String,
    pub balance: f64,
}

// ============ Basket Category DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBasketCategoryPayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBasketCategoryPayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketCategoryResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
