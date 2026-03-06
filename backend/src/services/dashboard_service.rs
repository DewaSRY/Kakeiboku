use axum::http::StatusCode;
use chrono::NaiveDate;
use sqlx::PgPool;

use crate::dtos::common_dto::{CommonErrorResponse};
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

    if data.is_empty() {
        return Err(CommonErrorResponse::new(
            "Data not found".to_string(),
            StatusCode::NOT_FOUND,
        ));
    }

    Ok(BranchSummaryResponse { data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_setup_util::setup;

    #[tokio::test]
    async fn test_get_user_money_stash_success() {
        let state = setup().await;
        let user_id = 1;
        let result = get_user_money_stash(&state.pool, user_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_branch_summary_success() {
        let state = setup().await;
        let user_id = 1;
        let start_date = None;
        let end_date = None;
        let result = get_branch_summary(&state.pool, user_id, start_date, end_date).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_branch_summary_with_dates() {
        let state = setup().await;
        let user_id = 1;
        // Use a valid date range for your test DB
        let start_date = Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        let end_date = Some(chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap());
        let result = get_branch_summary(&state.pool, user_id, start_date, end_date).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_branch_summary_error() {
        let state = setup().await;
        let user_id = -9999;
        let start_date = None;
        let end_date = None;
        let result = get_branch_summary(&state.pool, user_id, start_date, end_date).await;
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.error, "Data not found");
    }
}
