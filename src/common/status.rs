use crate::common::ErrorCode;
use actix_web::http::StatusCode;

/// HTTP状态码映射
impl ErrorCode {
    /// 获取对应的HTTP状态码
    pub fn http_status(&self) -> StatusCode {
        match self {
            // 成功状态
            ErrorCode::Success => StatusCode::OK,

            // 客户端错误 (4xx)
            ErrorCode::InvalidParams
            | ErrorCode::ValidationError
            | ErrorCode::InvalidEmail
            | ErrorCode::InvalidPassword
            | ErrorCode::InvalidSubscribeToken => StatusCode::BAD_REQUEST,

            ErrorCode::Unauthorized
            | ErrorCode::InvalidToken
            | ErrorCode::TokenExpired
            | ErrorCode::InvalidCredentials => StatusCode::UNAUTHORIZED,

            ErrorCode::PermissionDenied
            | ErrorCode::AccountLocked
            | ErrorCode::UserDisabled
            | ErrorCode::SubscribeBanned => StatusCode::FORBIDDEN,

            ErrorCode::UserNotFound
            | ErrorCode::PlanNotFound
            | ErrorCode::CouponNotFound
            | ErrorCode::OrderNotFound
            | ErrorCode::TicketNotFound
            | ErrorCode::NoticeNotFound
            | ErrorCode::KnowledgeNotFound
            | ErrorCode::SubscribeNotFound => StatusCode::NOT_FOUND,

            ErrorCode::UserAlreadyExists | ErrorCode::CouponUsed | ErrorCode::OrderAlreadyPaid => {
                StatusCode::CONFLICT
            }

            ErrorCode::CouponExpired
            | ErrorCode::CouponInvalid
            | ErrorCode::CouponDisabled
            | ErrorCode::CouponNotValid
            | ErrorCode::OrderExpired
            | ErrorCode::PlanUnavailable
            | ErrorCode::PlanQuotaExceeded
            | ErrorCode::InsufficientBalance
            | ErrorCode::TicketClosed
            | ErrorCode::SubscribeExpired
            | ErrorCode::TrafficExceeded
            | ErrorCode::NodeNotAvailable => StatusCode::UNPROCESSABLE_ENTITY,

            // 服务器错误 (5xx)
            ErrorCode::InternalError
            | ErrorCode::DatabaseError
            | ErrorCode::NetworkError
            | ErrorCode::PaymentFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 业务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessStatus {
    /// 操作成功
    Success,
    /// 客户端错误
    ClientError,
    /// 服务器错误
    ServerError,
}

impl From<ErrorCode> for BusinessStatus {
    fn from(error_code: ErrorCode) -> Self {
        match error_code {
            ErrorCode::Success => BusinessStatus::Success,
            code if code.is_client_error() => BusinessStatus::ClientError,
            _ => BusinessStatus::ServerError,
        }
    }
}

impl BusinessStatus {
    /// 获取默认的HTTP状态码
    pub fn default_http_status(&self) -> StatusCode {
        match self {
            BusinessStatus::Success => StatusCode::OK,
            BusinessStatus::ClientError => StatusCode::BAD_REQUEST,
            BusinessStatus::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 常用的状态码常量
pub mod status_codes {
    use super::ErrorCode;

    // 成功状态
    pub const SUCCESS: ErrorCode = ErrorCode::Success;

    // 通用错误
    pub const INTERNAL_ERROR: ErrorCode = ErrorCode::InternalError;
    pub const INVALID_PARAMS: ErrorCode = ErrorCode::InvalidParams;
    pub const VALIDATION_ERROR: ErrorCode = ErrorCode::ValidationError;

    // 认证错误
    pub const UNAUTHORIZED: ErrorCode = ErrorCode::Unauthorized;
    pub const INVALID_TOKEN: ErrorCode = ErrorCode::InvalidToken;
    pub const TOKEN_EXPIRED: ErrorCode = ErrorCode::TokenExpired;
    pub const INVALID_CREDENTIALS: ErrorCode = ErrorCode::InvalidCredentials;

    // 用户错误
    pub const USER_NOT_FOUND: ErrorCode = ErrorCode::UserNotFound;
    pub const USER_ALREADY_EXISTS: ErrorCode = ErrorCode::UserAlreadyExists;

    // 套餐错误
    pub const PLAN_NOT_FOUND: ErrorCode = ErrorCode::PlanNotFound;

    // 优惠券错误
    pub const COUPON_NOT_FOUND: ErrorCode = ErrorCode::CouponNotFound;
    pub const COUPON_EXPIRED: ErrorCode = ErrorCode::CouponExpired;

    // 订单错误
    pub const ORDER_NOT_FOUND: ErrorCode = ErrorCode::OrderNotFound;
    pub const PAYMENT_FAILED: ErrorCode = ErrorCode::PaymentFailed;
}
