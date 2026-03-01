use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::NaiveDateTime;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Basket {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub basket_category_id: i64,
    #[sqlx(rename = "type")]
    pub basket_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
