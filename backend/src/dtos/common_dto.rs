use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CommonErrorResponse {
    pub error: String,
    pub code: u16,
}

impl CommonErrorResponse {
    pub fn new(error: String, code: StatusCode) -> Self {
        Self {
            error,
            code: code.as_u16(),
        }
    }

    pub fn to_response(&self) -> (StatusCode, Json<Self>) {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self.clone()))
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CommonResponse {
    pub message: String,
    pub code: u16,
}

impl CommonResponse {
    pub fn new(message: String, code: StatusCode) -> Self {
        Self {
            message,
            code: code.as_u16(),
        }
    }

    pub fn to_response(&self) -> (StatusCode, Json<Self>) {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self.clone()))
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
