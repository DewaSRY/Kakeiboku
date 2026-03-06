use chrono::NaiveDate;

use crate::dtos::dashboard_dto::{BranchStats, UserBranchPercent};

pub async fn get_main_branch_balance<'e, E>(executor: E, user_id: i64) -> Result<Option<f64>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_scalar(
        r#"
        SELECT COALESCE(
            (SELECT SUM(CASE WHEN t.to_basket_id = b.id THEN t.amount ELSE 0 END) -
                    SUM(CASE WHEN t.from_basket_id = b.id THEN t.amount ELSE 0 END)
             FROM transactions t
             WHERE t.from_basket_id = b.id OR t.to_basket_id = b.id), 0
        )::float8
        FROM baskets b
        WHERE b.user_id = $1 AND b.type = 'main'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(executor)
    .await
}

pub async fn get_total_spend<'e, E>(executor: E, user_id: i64) -> Result<f64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(t.amount), 0)::float8
        FROM transactions t
        JOIN baskets b ON t.from_basket_id = b.id
        WHERE b.user_id = $1 AND t.from_basket_id IS NOT NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(executor)
    .await
}

pub async fn get_total_save<'e, E>(executor: E, user_id: i64) -> Result<f64, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(t.amount), 0)::float8
        FROM transactions t
        JOIN baskets b ON t.to_basket_id = b.id
        WHERE b.user_id = $1 AND t.from_basket_id IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(executor)
    .await
}

// TODO: update this letter
pub async fn get_branch_category_percentages<'e, E>(
    executor: E,
    user_id: i64,
) -> Result<Vec<UserBranchPercent>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, UserBranchPercent>(
        r#"
        WITH basket_balances AS (
            SELECT 
                b.id,
                b.basket_category_id,
                COALESCE(
                    (SELECT SUM(CASE WHEN t.to_basket_id = b.id THEN t.amount ELSE 0 END) -
                            SUM(CASE WHEN t.from_basket_id = b.id THEN t.amount ELSE 0 END)
                     FROM transactions t
                     WHERE t.from_basket_id = b.id OR t.to_basket_id = b.id), 0
                )::float8 as balance
            FROM baskets b
            WHERE b.user_id = $1 AND b.type = 'branch'
        ),
        category_totals AS (
            SELECT 
                bc.name as branch_category_name,
                COALESCE(SUM(bb.balance), 0) as category_balance
            FROM basket_category bc
            LEFT JOIN basket_balances bb ON bc.id = bb.basket_category_id
            GROUP BY bc.id, bc.name
        ),
        total_balance AS (
            SELECT COALESCE(SUM(balance), 0) as total FROM basket_balances WHERE balance > 0
        )
        SELECT 
            ct.branch_category_name,
            CASE 
                WHEN tb.total > 0 THEN ROUND((ct.category_balance / tb.total * 100)::numeric, 2)::float8
                ELSE 0.0
            END as total_percent
        FROM category_totals ct, total_balance tb
        WHERE ct.category_balance > 0
        ORDER BY ct.branch_category_name
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await
}

pub async fn get_branch_summary<'e, E>(
    executor: E,
    user_id: i64,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<BranchStats>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e,
{
    sqlx::query_as::<_, BranchStats>(
        r#"
        SELECT 
            b.name,
            COALESCE(
                (SELECT SUM(CASE WHEN t.to_basket_id = b.id THEN t.amount ELSE 0 END) -
                        SUM(CASE WHEN t.from_basket_id = b.id THEN t.amount ELSE 0 END)
                 FROM transactions t
                 WHERE (t.from_basket_id = b.id OR t.to_basket_id = b.id)
                   AND ($2::date IS NULL OR t.created_at >= $2::date)
                   AND ($3::date IS NULL OR t.created_at < ($3::date + interval '1 day'))
                ), 0
            )::float8 as total
        FROM baskets b
        WHERE b.user_id = $1 AND b.type = 'branch'
        ORDER BY b.name
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(executor)
    .await
}
