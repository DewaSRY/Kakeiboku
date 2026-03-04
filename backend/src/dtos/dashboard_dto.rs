use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserMoneyStash {
    pub main_branch: f64,
    pub total_spend: f64,
    pub total_save: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct UserBranchPercent {
    pub branch_category_name: String,
    pub total_percent: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub  struct UserMoneyStashResponse {
    pub money_stash: UserMoneyStash,
    pub branch_category_percentages: Vec<UserBranchPercent>,
}