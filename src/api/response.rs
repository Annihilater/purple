use actix_web::{Error, HttpResponse, ResponseError};
use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use validator::ValidationErrors;

use crate::models::user::User;

pub type Response<T> = actix_web::Result<T>;

#[derive(Debug)]
pub struct ApiError(AnyhowError);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, self.0.to_string()))
    }
}

impl From<AnyhowError> for ApiError {
    fn from(err: AnyhowError) -> Self {
        Self(err)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        Self(AnyhowError::msg(err.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<User>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}
