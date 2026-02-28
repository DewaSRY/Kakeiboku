use crate::state::AppState;
use crate::utils::jwt_util::{
  create_access_token,
};
use crate::utils::hash_util::{hash_password, verify_password};
use  crate::dtos::auth_dto::UserProfile;
use crate::repositories::user_repository;
use crate::dtos::auth_dto::{
    AuthResponse, 
    LoginPayload, 
    RegisterPayload,
};


pub  async fn register_user(
    state: &AppState,
    payload: RegisterPayload,
) ->anyhow:: Result<AuthResponse> {
    if user_repository::find_by_email(&state.pool, &payload.email).await.is_ok() {
         anyhow::bail!("Email already registered")
    }

    let hashed_password = hash_password(&payload.password).map_err(|_| {
        anyhow::anyhow!("Failed to hash password")
    })?;

    let user_id = user_repository::create(&state.pool, payload.first_name, payload.last_name, hashed_password, payload.email).await?;
    let token = create_access_token(user_id, &state.jwt_secret.as_bytes())?;
    let user = user_repository::find_by_id(&state.pool, user_id).await?;

    let user_profile = UserProfile {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };

    Ok(AuthResponse { token, user:user_profile })
}

pub async fn login_user(
    state: &AppState,
    payload: LoginPayload,
) -> anyhow::Result<AuthResponse> {
    let user_id = user_repository::find_by_email(&state.pool, &payload.email).await?;
    let user = user_repository::find_by_id(&state.pool, user_id).await?;

    if !verify_password(&payload.password, &user.password) {
        anyhow::bail!("Invalid credentials")
    }

    let token = create_access_token(user_id, &state.jwt_secret.as_bytes())?;

    let user_profile = UserProfile {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };

    Ok(AuthResponse { token, user:user_profile })
}