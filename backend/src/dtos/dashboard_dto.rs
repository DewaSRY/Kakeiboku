use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

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

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct BranchStats {
    pub name: String,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BranchSummaryResponse {
    pub data: Vec<BranchStats>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct DateRangeQuery {
    #[param(example = "2026-01-01")]
    pub start_date: Option<NaiveDate>,
    #[param(example = "2026-12-31")]
    pub end_date: Option<NaiveDate>,
}