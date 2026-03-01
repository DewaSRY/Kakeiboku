use axum::{Json, extract::State, http::StatusCode};

use crate::dtos::{
    auth_dto::{AuthResponse, LoginPayload, RegisterPayload},
    common_dto::CommonErrorResponse,
};
use crate::services::auth_service;
use crate::state::AppState;

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
    auth_service::register_user(&state, payload)
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
    auth_service::login_user(&state, payload)
        .await
        .map(Json)
        .map_err(|err| err.to_response())
}
