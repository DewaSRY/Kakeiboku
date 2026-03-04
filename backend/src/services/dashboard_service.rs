use axum::http::StatusCode;
use chrono::NaiveDate;
use sqlx::PgPool;

use crate::dtos::common_dto::CommonErrorResponse;
use crate::dtos::dashboard_dto::{BranchSummaryResponse, UserMoneyStash, UserMoneyStashResponse};
use crate::repositories::dashboard_repository;

pub async fn get_user_money_stash(
    pool: &PgPool,
    user_id: i64,
) -> Result<UserMoneyStashResponse, CommonErrorResponse> {
    let main_branch = dashboard_repository::get_main_branch_balance(pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to fetch main branch balance".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .unwrap_or(0.0);

    let total_spend = dashboard_repository::get_total_spend(pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to fetch total spend".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let total_save = dashboard_repository::get_total_save(pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to fetch total save".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let branch_percentages = dashboard_repository::get_branch_category_percentages(pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to fetch branch percentages".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(UserMoneyStashResponse {
        money_stash: UserMoneyStash {
            main_branch,
            total_spend,
            total_save,
        },
        branch_category_percentages: branch_percentages,
    })
}

pub async fn get_branch_summary(
    pool: &PgPool,
    user_id: i64,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<BranchSummaryResponse, CommonErrorResponse> {
    let data = dashboard_repository::get_branch_summary(pool, user_id, start_date, end_date)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to fetch branch summary".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(BranchSummaryResponse { data })
}
