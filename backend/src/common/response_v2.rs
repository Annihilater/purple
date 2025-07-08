//! # 统一 API 响应系统 v2
//!
//! 提供标准化的 RESTful API 响应格式，确保所有接口返回数据的一致性和可维护性。
//!
//! ## 核心设计原则
//!
//! 1. **语义明确**: `success` 字段明确表示操作是否成功
//! 2. **类型安全**: 错误代码使用字符串枚举，便于维护和理解
//! 3. **RESTful 兼容**: 配合 HTTP 状态码使用
//! 4. **扩展性**: `meta` 字段可包含时间戳、请求ID等元数据
//! 5. **调试友好**: 可选的 `request_id` 便于问题追踪
//!
//! ## 使用示例
//!
//! ### 成功响应
//! ```rust
//! use crate::common::response_v2::{ApiResponse, IntoHttpResponse};
//!
//! let response = ApiResponse::success("操作成功");
//! let http_response = response.into_http_response();
//! ```
//!
//! ### 分页响应
//! ```rust
//! use crate::common::response_v2::{ApiResponse, IntoHttpResponse};
//!
//! let data = vec!["item1", "item2"];
//! let response = ApiResponse::page(data, 1, 10, 100);
//! let http_response = response.into_http_response();
//! ```
//!
//! ### 错误响应
//! ```rust
//! use crate::common::response_v2::{ApiResponse, ApiError, IntoHttpResponse};
//! use crate::common::ErrorCode;
//!
//! // 简单错误响应
//! let response = ApiResponse::error(ErrorCode::UserNotFound);
//! let http_response = response.into_http_response();
//!
//! // 带详细信息的错误响应
//! let error = ApiError::with_details(
//!     ErrorCode::ValidationError,
//!     "用户名不能为空".to_string()
//! );
//! ```
//!
//! ## 响应格式示例
//!
//! 所有响应都遵循统一的 JSON 格式：
//!
//! ```json
//! // 成功响应
//! {
//!   "success": true,
//!   "data": { "id": 1, "name": "示例数据" },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//!
//! // 分页响应
//! {
//!   "success": true,
//!   "data": [{"id": 1}, {"id": 2}],
//!   "pagination": {
//!     "page": 1,
//!     "page_size": 10,
//!     "total": 100,
//!     "total_pages": 10,
//!     "has_next": true,
//!     "has_prev": false
//!   },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//!
//! // 错误响应
//! {
//!   "success": false,
//!   "error": {
//!     "code": "USER_NOT_FOUND",
//!     "message": "用户未找到",
//!     "details": "用户ID 123 不存在"
//!   },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//! ```

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::common::ErrorCode;

/// 统一API响应结构 - 成功响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiSuccessResponse<T> {
    /// 操作是否成功
    pub success: bool,
    /// 响应数据
    pub data: T,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 统一API响应结构 - 错误响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorResponse {
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    pub error: ErrorDetail,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 分页响应结构
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiPageResponse<T> {
    /// 操作是否成功
    pub success: bool,
    /// 响应数据
    pub data: Vec<T>,
    /// 分页信息
    pub pagination: PaginationMeta,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 错误详情
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDetail {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 详细错误信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// 字段级错误（用于表单验证）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

/// 响应元数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseMeta {
    /// 时间戳
    pub timestamp: i64,
    /// 请求ID（用于追踪）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// 分页元数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_prev: bool,
}

impl PaginationMeta {
    /// 创建分页元数据
    pub fn new(page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            total.div_ceil(page_size)
        };
        let has_next = page < total_pages;
        let has_prev = page > 1;

        Self {
            page,
            page_size,
            total,
            total_pages,
            has_next,
            has_prev,
        }
    }
}

impl ResponseMeta {
    /// 创建响应元数据
    pub fn new() -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp(),
            request_id: Some(Uuid::new_v4().to_string()),
        }
    }

    /// 创建不带请求ID的元数据
    pub fn new_without_request_id() -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp(),
            request_id: None,
        }
    }
}

impl Default for ResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// 统一API响应构建器
pub struct ApiResponse;

impl ApiResponse {
    /// 创建成功响应
    pub fn success<T>(data: T) -> ApiSuccessResponse<T> {
        ApiSuccessResponse {
            success: true,
            data,
            meta: ResponseMeta::new(),
        }
    }

    /// 创建成功响应（不带请求ID）
    pub fn success_simple<T>(data: T) -> ApiSuccessResponse<T> {
        ApiSuccessResponse {
            success: true,
            data,
            meta: ResponseMeta::new_without_request_id(),
        }
    }

    /// 创建分页响应
    pub fn page<T>(data: Vec<T>, page: u64, page_size: u64, total: u64) -> ApiPageResponse<T> {
        ApiPageResponse {
            success: true,
            data,
            pagination: PaginationMeta::new(page, page_size, total),
            meta: ResponseMeta::new(),
        }
    }

    /// 创建错误响应
    pub fn error(error_code: ErrorCode) -> ApiErrorResponse {
        ApiErrorResponse {
            success: false,
            error: ErrorDetail {
                code: error_code.to_string(),
                message: error_code.message().to_string(),
                details: None,
                field: None,
            },
            meta: ResponseMeta::new(),
        }
    }

    /// 创建带详细信息的错误响应
    pub fn error_with_details(
        error_code: ErrorCode,
        details: Option<String>,
        field: Option<String>,
    ) -> ApiErrorResponse {
        ApiErrorResponse {
            success: false,
            error: ErrorDetail {
                code: error_code.to_string(),
                message: error_code.message().to_string(),
                details,
                field,
            },
            meta: ResponseMeta::new(),
        }
    }

    /// 创建自定义错误响应
    pub fn custom_error(
        code: String,
        message: String,
        details: Option<String>,
        field: Option<String>,
    ) -> ApiErrorResponse {
        ApiErrorResponse {
            success: false,
            error: ErrorDetail {
                code,
                message,
                details,
                field,
            },
            meta: ResponseMeta::new(),
        }
    }
}

/// HTTP响应转换 trait
pub trait IntoHttpResponse {
    fn into_http_response(self) -> HttpResponse;
}

impl<T: Serialize> IntoHttpResponse for ApiSuccessResponse<T> {
    fn into_http_response(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl<T: Serialize> IntoHttpResponse for ApiPageResponse<T> {
    fn into_http_response(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl IntoHttpResponse for ApiErrorResponse {
    fn into_http_response(self) -> HttpResponse {
        // 根据错误代码确定HTTP状态码
        let status = if let Ok(error_code) = self.error.code.parse::<ErrorCode>() {
            error_code.http_status()
        } else {
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        };

        HttpResponse::build(status).json(self)
    }
}

/// API错误类型（用于错误传播）
#[derive(Debug)]
pub struct ApiError {
    pub error_code: ErrorCode,
    pub details: Option<String>,
    pub field: Option<String>,
}

impl ApiError {
    pub fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            details: None,
            field: None,
        }
    }

    pub fn with_details(error_code: ErrorCode, details: String) -> Self {
        Self {
            error_code,
            details: Some(details),
            field: None,
        }
    }

    pub fn with_field(error_code: ErrorCode, field: String) -> Self {
        Self {
            error_code,
            details: None,
            field: Some(field),
        }
    }

    pub fn with_details_and_field(error_code: ErrorCode, details: String, field: String) -> Self {
        Self {
            error_code,
            details: Some(details),
            field: Some(field),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code, self.error_code.message())
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        ApiResponse::error_with_details(self.error_code, self.details.clone(), self.field.clone())
            .into_http_response()
    }
}

/// 便捷的结果类型
pub type ApiResult<T> = Result<T, ApiError>;

/// 便捷的宏定义
#[macro_export]
macro_rules! success_response_v2 {
    ($data:expr) => {
        Ok($crate::common::response_v2::ApiResponse::success($data).into_http_response())
    };
}

#[macro_export]
macro_rules! page_response_v2 {
    ($data:expr, $page:expr, $page_size:expr, $total:expr) => {
        Ok(
            $crate::common::response_v2::ApiResponse::page($data, $page, $page_size, $total)
                .into_http_response(),
        )
    };
}

#[macro_export]
macro_rules! error_response_v2 {
    ($error_code:expr) => {
        Ok($crate::common::response_v2::ApiResponse::error($error_code).into_http_response())
    };
    ($error_code:expr, $details:expr) => {
        Ok(
            $crate::common::response_v2::ApiResponse::error_with_details(
                $error_code,
                Some($details.to_string()),
                None,
            )
            .into_http_response(),
        )
    };
    ($error_code:expr, $details:expr, $field:expr) => {
        Ok(
            $crate::common::response_v2::ApiResponse::error_with_details(
                $error_code,
                Some($details.to_string()),
                Some($field.to_string()),
            )
            .into_http_response(),
        )
    };
}

// 错误转换
impl From<ErrorCode> for ApiError {
    fn from(error_code: ErrorCode) -> Self {
        Self::new(error_code)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Anyhow error: {}", err);
        Self::with_details(ErrorCode::InternalError, err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", err);
        match err {
            sqlx::Error::RowNotFound => Self::new(ErrorCode::DatabaseError),
            _ => Self::with_details(ErrorCode::DatabaseError, err.to_string()),
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        let details = err
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_msgs = errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .unwrap_or(&std::borrow::Cow::Borrowed("validation error"))
                            .to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", field, error_msgs)
            })
            .collect::<Vec<_>>()
            .join("; ");

        Self::with_details(ErrorCode::ValidationError, details)
    }
}
