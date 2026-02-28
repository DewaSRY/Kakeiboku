use axum::{
    extract::{ State},
    http::StatusCode,
    Json,
};


use crate::dtos::auth_dto::{LoginPayload, RegisterPayload, AuthResponse};
use crate::state::AppState;
use crate::services::auth_service;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterPayload,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    auth_service::register_user(&state, payload)
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Registration error: {:?}", e);
            StatusCode::BAD_REQUEST
        })
}   


#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub  async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    auth_service::login_user(&state, payload)
        .await
        .map(Json)
        .map_err(|e| {
            eprintln!("Login error: {:?}", e);
            StatusCode::BAD_REQUEST
        })
}   

