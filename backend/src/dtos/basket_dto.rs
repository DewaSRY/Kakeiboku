use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{baskets::BasketWithBalance, baskets_category::BasketCategory};

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct BasketTransaction {
//     pub from_basket_name: String,
//     pub to_basket_name: String,
//     pub amount: f64,
//     pub transaction_type_name: String,
//     pub transaction_description: Option<String>,
//     pub transaction_title: Option<String>,
//     pub created_at: NaiveDateTime,
// }

// ============ Basket DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBasketPayload {
    pub name: String,
    pub description: Option<String>,
    pub basket_category_id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBasketPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>, // "active", "frozen", "archived"
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
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

impl BasketResponse {
    pub fn from_model(b: BasketWithBalance) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            name: b.name,
            description: b.description,
            basket_category_id: b.basket_category_id,
            basket_type: b.basket_type,
            status: b.status,
            balance: b.balance,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketDetailResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub basket_type: String,
    pub status: String,
    pub balance: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub basket_category: BasketCategoryResponse,
}

impl BasketDetailResponse {
    pub fn from_model(b: BasketWithBalance, category: BasketCategory) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            name: b.name,
            description: b.description,
            basket_type: b.basket_type,
            status: b.status,
            balance: b.balance,
            created_at: b.created_at,
            updated_at: b.updated_at,
            basket_category: BasketCategoryResponse::from(category),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct BasketCategoryResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<BasketCategory> for BasketCategoryResponse {
    fn from(category: BasketCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            created_at: category.created_at,
        }
    }
}
