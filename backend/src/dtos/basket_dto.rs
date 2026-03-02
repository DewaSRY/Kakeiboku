use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


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
pub struct BasketCategoryResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
