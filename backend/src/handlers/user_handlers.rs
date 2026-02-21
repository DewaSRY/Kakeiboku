use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::state::AppState;
use crate::dtos::user_dto::UserPayload;
use crate::services::user_service;

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::user::User>>, StatusCode> {
    user_service::list_users(&state.pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserPayload>,
) -> Result<(StatusCode, Json<crate::models::user::User>), StatusCode> {
    let user = user_service::create_user(
        &state.pool,
        payload.name,
        payload.email,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<crate::models::user::User>), StatusCode> {
    user_service::get_user(&state.pool, id)
        .await
        .map(|user| (StatusCode::OK, Json(user)))
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows = user_service::delete_user(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}