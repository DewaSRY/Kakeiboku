use axum::{Json, extract::State, http::StatusCode};

use crate::dtos::{
    auth_dto::{AuthResponse, LoginPayload, RegisterPayload, UserProfile},
    common_dto::CommonErrorResponse,
};
use crate::services::auth_service;
use crate::state::AppState;
use crate::utils::jwt_util::AuthUser;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterPayload,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request",body = CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    auth_service::register_user(&state.pool, &state.jwt_secret, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponse),
        (status = 400, description = "Bad request", body= CommonErrorResponse),
        (status = 500, description = "Internal server error", body = CommonErrorResponse)
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<CommonErrorResponse>)> {
    auth_service::login_user(&state.pool, &state.jwt_secret, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}

pub async fn get_profile(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserProfile>, (StatusCode, Json<CommonErrorResponse>)> {
    auth_service::get_profile(&state.pool, user_id)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}
