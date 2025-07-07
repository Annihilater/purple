use actix_web::{Error, HttpResponse, ResponseError};
use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use validator::ValidationErrors;

use crate::models::user::User;

pub type Response<T> = actix_web::Result<T>;

/// 旧版API错误类型（保持向后兼容）
#[derive(Debug)]
pub struct ApiError(AnyhowError);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        crate::common::ResponseBuilder::error_with_message(
            crate::common::ErrorCode::InternalError,
            self.0.to_string(),
        )
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

/// 旧版API响应结构（保持向后兼容）
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// 用户响应结构（保持向后兼容）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<User>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应（保持向后兼容）
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    /// 创建错误响应（保持向后兼容）
    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

/// 便捷的响应构建函数
pub mod response_helpers {
    use crate::common;
    use actix_web::HttpResponse;
    use serde::Serialize;

    /// 构建成功响应
    pub fn success<T: Serialize>(data: T) -> HttpResponse {
        common::ResponseBuilder::success(data)
    }

    /// 构建成功响应（带消息）
    pub fn success_with_message<T: Serialize>(data: T, message: &str) -> HttpResponse {
        common::ResponseBuilder::success_with_message(data, message.to_string())
    }

    /// 构建错误响应
    pub fn error(error_code: common::ErrorCode) -> HttpResponse {
        common::ResponseBuilder::error(error_code)
    }

    /// 构建错误响应（带消息）
    pub fn error_with_message(error_code: common::ErrorCode, message: &str) -> HttpResponse {
        common::ResponseBuilder::error_with_message(error_code, message.to_string())
    }

    /// 构建分页响应
    pub fn page<T: Serialize>(
        items: Vec<T>,
        total: u64,
        page: u64,
        page_size: u64,
    ) -> HttpResponse {
        common::ResponseBuilder::page(items, total, page, page_size)
    }
}

/// 重新导出新的通用响应类型，推荐使用这些
pub mod common_types {
    pub use crate::common::{
        ApiError as NewApiError, ApiResponse as NewApiResponse, ApiResult, ErrorCode, PageResponse,
        ResponseBuilder,
    };
}
