use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
};

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Missing or invalid token")]
    InvalidToken,
    #[error("Token expired")]
    ExpiredToken,
    #[error("Internal error")]
    Internal,
}

pub fn create_access_token(user_id: i64, secret: &[u8]) -> Result<String, AuthError> {
    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|_| AuthError::Internal)
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, AuthError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
            _ => AuthError::InvalidToken,
        })
        .map(|data| data.claims)
}

#[derive(Debug)]
pub struct AuthUser(pub i64);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?;

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::BAD_REQUEST,
            "Invalid Authorization format".to_string(),
        ))?;

        let claims = validate_token(token, state.jwt_secret.as_bytes())
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        Ok(AuthUser(claims.sub))
    }
}
