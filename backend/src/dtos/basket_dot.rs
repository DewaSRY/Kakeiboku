use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BasketItem {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}


