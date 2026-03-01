use axum::http::StatusCode;

use crate::dtos::auth_dto::{AuthResponse, LoginPayload, RegisterPayload};
use crate::dtos::{auth_dto::UserProfile, common_dto::CommonErrorResponse};
use crate::repositories::user_repository;
use crate::state::AppState;
use crate::utils::hash_util::{hash_password, verify_password};
use crate::utils::jwt_util::create_access_token;

pub async fn register_user(
    state: &AppState,
    payload: RegisterPayload,
) -> Result<AuthResponse, CommonErrorResponse> {
    let hashed_password = hash_password(&payload.password).map_err(|_| {
        CommonErrorResponse::new(
            "Failed to hash password".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let user = user_repository::create(
        &state.pool,
        payload.first_name,
        payload.last_name,
        hashed_password,
        payload.email,
    )
    .await
    .map_err(|_| {
        CommonErrorResponse::new("Failed to create user".to_string(), StatusCode::CONFLICT)
    })?;

    let token = create_access_token(user.id, &state.jwt_secret.as_bytes()).map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create access token".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let user_profile = UserProfile {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };

    Ok(AuthResponse {
        token,
        user: user_profile,
    })
}

pub async fn login_user(
    state: &AppState,
    payload: LoginPayload,
) -> Result<AuthResponse, CommonErrorResponse> {
    let user_id = user_repository::find_by_email(&state.pool, &payload.email)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Invalid email or password".to_string(),
                StatusCode::UNAUTHORIZED,
            )
        })?;
    let user = user_repository::find_by_id(&state.pool, user_id)
        .await
        .map_err(|_| {
            CommonErrorResponse::new(
                "Failed to retrieve user".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    if !verify_password(&payload.password, &user.password) {
        return Err(CommonErrorResponse::new(
            "Invalid email or password".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let token = create_access_token(user_id, &state.jwt_secret.as_bytes()).map_err(|_| {
        CommonErrorResponse::new(
            "Failed to create access token".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let user_profile = UserProfile {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };

    Ok(AuthResponse {
        token,
        user: user_profile,
    })
}

#[cfg(test)]
mod tests {
   use  super::*;
   use  crate::utils::test_setup_util::setup;

    #[sqlx::test]
    async fn test_register() {
        let state = setup().await;
        let email = format!("test_{}@email.com", uuid::Uuid::new_v4());

        let payload = RegisterPayload {
            first_name: "Dewa".to_string(),
            last_name: "Surya".to_string(),
            email: email.clone(),
            password: "password123".to_string(),
        };

        let result = register_user(&state, payload).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        
        assert_eq!(response.user.first_name, "Dewa");
        assert_eq!(response.user.last_name, "Surya");
        assert_eq!(response.user.email, email);
        assert!(!response.token.is_empty());

    }
}
