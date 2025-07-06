use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

use crate::common::{ErrorCode, BusinessStatus};

/// 通用API响应结构
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// 业务错误代码
    pub code: i32,
    /// 业务状态码字符串
    pub status: String,
    /// 响应消息
    pub message: String,
    /// 响应数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 时间戳
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self::success_with_message(data, None)
    }

    /// 创建成功响应（带自定义消息）
    pub fn success_with_message(data: T, message: Option<String>) -> Self {
        let error_code = ErrorCode::Success;
        Self {
            code: error_code.code(),
            status: format!("{:?}", error_code),
            message: message.unwrap_or_else(|| error_code.message().to_string()),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// 创建失败响应
    pub fn error(error_code: ErrorCode) -> Self {
        Self::error_with_message(error_code, None)
    }

    /// 创建失败响应（带自定义消息）
    pub fn error_with_message(error_code: ErrorCode, message: Option<String>) -> Self {
        Self {
            code: error_code.code(),
            status: format!("{:?}", error_code),
            message: message.unwrap_or_else(|| error_code.message().to_string()),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// 创建空数据的成功响应
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse::success(())
    }

    /// 将响应转换为HTTP响应
    pub fn into_response(self) -> HttpResponse
    where
        T: Serialize,
    {
        let error_code = ErrorCode::from(self.code);
        HttpResponse::build(error_code.http_status()).json(self)
    }
}

/// 分页响应数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总记录数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_prev: bool,
}

impl<T> PageResponse<T> {
    /// 创建分页响应
    pub fn new(items: Vec<T>, total: u64, page: u64, page_size: u64) -> Self {
        let total_pages = (total + page_size - 1) / page_size;
        let has_next = page < total_pages;
        let has_prev = page > 1;

        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
            has_next,
            has_prev,
        }
    }
}

/// API错误类型
#[derive(Debug)]
pub struct ApiError {
    pub error_code: ErrorCode,
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ApiError {
    /// 创建API错误
    pub fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            message: None,
            cause: None,
        }
    }

    /// 创建带消息的API错误
    pub fn with_message(error_code: ErrorCode, message: String) -> Self {
        Self {
            error_code,
            message: Some(message),
            cause: None,
        }
    }

    /// 创建带原因的API错误
    pub fn with_cause(error_code: ErrorCode, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self {
            error_code,
            message: None,
            cause: Some(cause),
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> String {
        self.message
            .clone()
            .unwrap_or_else(|| self.error_code.message().to_string())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code.code(), self.message())
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &dyn std::error::Error)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        ApiResponse::<()>::error_with_message(self.error_code, Some(self.message()))
            .into_response()
    }
}

/// 方便的错误转换
impl From<ErrorCode> for ApiError {
    fn from(error_code: ErrorCode) -> Self {
        Self::new(error_code)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Anyhow error: {}", err);
        Self::with_message(ErrorCode::InternalError, err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", err);
        match err {
            sqlx::Error::RowNotFound => Self::new(ErrorCode::DatabaseError),
            _ => Self::with_cause(ErrorCode::DatabaseError, Box::new(err)),
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        let message = err.field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_msgs = errors.iter()
                    .map(|e| e.message.as_ref().unwrap_or(&std::borrow::Cow::Borrowed("validation error")).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", field, error_msgs)
            })
            .collect::<Vec<_>>()
            .join("; ");
        
        Self::with_message(ErrorCode::ValidationError, message)
    }
}

/// 便捷的结果类型
pub type ApiResult<T> = Result<T, ApiError>;

/// 响应构建器
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// 构建成功响应
    pub fn success<T: Serialize>(data: T) -> HttpResponse {
        ApiResponse::success(data).into_response()
    }

    /// 构建成功响应（带消息）
    pub fn success_with_message<T: Serialize>(data: T, message: String) -> HttpResponse {
        ApiResponse::success_with_message(data, Some(message)).into_response()
    }

    /// 构建错误响应
    pub fn error(error_code: ErrorCode) -> HttpResponse {
        ApiResponse::<()>::error(error_code).into_response()
    }

    /// 构建错误响应（带消息）
    pub fn error_with_message(error_code: ErrorCode, message: String) -> HttpResponse {
        ApiResponse::<()>::error_with_message(error_code, Some(message)).into_response()
    }

    /// 构建分页响应
    pub fn page<T: Serialize>(items: Vec<T>, total: u64, page: u64, page_size: u64) -> HttpResponse {
        let page_data = PageResponse::new(items, total, page, page_size);
        Self::success(page_data)
    }
}

/// 便捷的宏定义
#[macro_export]
macro_rules! success_response {
    ($data:expr) => {
        Ok(crate::common::ResponseBuilder::success($data))
    };
    ($data:expr, $message:expr) => {
        Ok(crate::common::ResponseBuilder::success_with_message($data, $message.to_string()))
    };
}

#[macro_export]
macro_rules! error_response {
    ($error_code:expr) => {
        Ok(crate::common::ResponseBuilder::error($error_code))
    };
    ($error_code:expr, $message:expr) => {
        Ok(crate::common::ResponseBuilder::error_with_message($error_code, $message.to_string()))
    };
} 