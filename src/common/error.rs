use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

/// 业务错误代码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorCode {
    // 通用错误 (1000-1999)
    #[serde(rename = "SUCCESS")]
    Success = 1000,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError = 1001,
    #[serde(rename = "INVALID_PARAMS")]
    InvalidParams = 1002,
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError = 1003,
    #[serde(rename = "DATABASE_ERROR")]
    DatabaseError = 1004,
    #[serde(rename = "NETWORK_ERROR")]
    NetworkError = 1005,

    // 认证相关错误 (2000-2999)
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized = 2000,
    #[serde(rename = "INVALID_TOKEN")]
    InvalidToken = 2001,
    #[serde(rename = "TOKEN_EXPIRED")]
    TokenExpired = 2002,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials = 2003,
    #[serde(rename = "ACCOUNT_LOCKED")]
    AccountLocked = 2004,
    #[serde(rename = "PERMISSION_DENIED")]
    PermissionDenied = 2005,

    // 用户相关错误 (3000-3999)
    #[serde(rename = "USER_NOT_FOUND")]
    UserNotFound = 3000,
    #[serde(rename = "USER_ALREADY_EXISTS")]
    UserAlreadyExists = 3001,
    #[serde(rename = "INVALID_EMAIL")]
    InvalidEmail = 3002,
    #[serde(rename = "INVALID_PASSWORD")]
    InvalidPassword = 3003,
    #[serde(rename = "USER_DISABLED")]
    UserDisabled = 3004,

    // 套餐相关错误 (4000-4999)
    #[serde(rename = "PLAN_NOT_FOUND")]
    PlanNotFound = 4000,
    #[serde(rename = "PLAN_UNAVAILABLE")]
    PlanUnavailable = 4001,
    #[serde(rename = "PLAN_QUOTA_EXCEEDED")]
    PlanQuotaExceeded = 4002,

    // 优惠券相关错误 (5000-5999)
    #[serde(rename = "COUPON_NOT_FOUND")]
    CouponNotFound = 5000,
    #[serde(rename = "COUPON_EXPIRED")]
    CouponExpired = 5001,
    #[serde(rename = "COUPON_USED")]
    CouponUsed = 5002,
    #[serde(rename = "COUPON_INVALID")]
    CouponInvalid = 5003,

    // 订单相关错误 (6000-6999)
    #[serde(rename = "ORDER_NOT_FOUND")]
    OrderNotFound = 6000,
    #[serde(rename = "ORDER_ALREADY_PAID")]
    OrderAlreadyPaid = 6001,
    #[serde(rename = "ORDER_EXPIRED")]
    OrderExpired = 6002,
    #[serde(rename = "PAYMENT_FAILED")]
    PaymentFailed = 6003,
    #[serde(rename = "INSUFFICIENT_BALANCE")]
    InsufficientBalance = 6004,
}

impl ErrorCode {
    /// 获取错误代码的数值
    pub fn code(&self) -> i32 {
        *self as i32
    }

    /// 获取错误代码的默认中文消息
    pub fn message(&self) -> &'static str {
        match self {
            // 通用错误
            ErrorCode::Success => "操作成功",
            ErrorCode::InternalError => "内部服务器错误",
            ErrorCode::InvalidParams => "请求参数无效",
            ErrorCode::ValidationError => "数据验证失败",
            ErrorCode::DatabaseError => "数据库操作失败",
            ErrorCode::NetworkError => "网络连接错误",

            // 认证相关错误
            ErrorCode::Unauthorized => "未授权访问",
            ErrorCode::InvalidToken => "令牌无效",
            ErrorCode::TokenExpired => "令牌已过期",
            ErrorCode::InvalidCredentials => "用户名或密码错误",
            ErrorCode::AccountLocked => "账户已被锁定",
            ErrorCode::PermissionDenied => "权限不足",

            // 用户相关错误
            ErrorCode::UserNotFound => "用户不存在",
            ErrorCode::UserAlreadyExists => "用户已存在",
            ErrorCode::InvalidEmail => "邮箱格式无效",
            ErrorCode::InvalidPassword => "密码格式无效",
            ErrorCode::UserDisabled => "用户已被禁用",

            // 套餐相关错误
            ErrorCode::PlanNotFound => "套餐不存在",
            ErrorCode::PlanUnavailable => "套餐不可用",
            ErrorCode::PlanQuotaExceeded => "套餐配额已超限",

            // 优惠券相关错误
            ErrorCode::CouponNotFound => "优惠券不存在",
            ErrorCode::CouponExpired => "优惠券已过期",
            ErrorCode::CouponUsed => "优惠券已使用",
            ErrorCode::CouponInvalid => "优惠券无效",

            // 订单相关错误
            ErrorCode::OrderNotFound => "订单不存在",
            ErrorCode::OrderAlreadyPaid => "订单已支付",
            ErrorCode::OrderExpired => "订单已过期",
            ErrorCode::PaymentFailed => "支付失败",
            ErrorCode::InsufficientBalance => "余额不足",
        }
    }

    /// 获取错误代码的英文消息
    pub fn message_en(&self) -> &'static str {
        match self {
            // 通用错误
            ErrorCode::Success => "Operation successful",
            ErrorCode::InternalError => "Internal server error",
            ErrorCode::InvalidParams => "Invalid request parameters",
            ErrorCode::ValidationError => "Data validation failed",
            ErrorCode::DatabaseError => "Database operation failed",
            ErrorCode::NetworkError => "Network connection error",

            // 认证相关错误
            ErrorCode::Unauthorized => "Unauthorized access",
            ErrorCode::InvalidToken => "Invalid token",
            ErrorCode::TokenExpired => "Token expired",
            ErrorCode::InvalidCredentials => "Invalid username or password",
            ErrorCode::AccountLocked => "Account locked",
            ErrorCode::PermissionDenied => "Permission denied",

            // 用户相关错误
            ErrorCode::UserNotFound => "User not found",
            ErrorCode::UserAlreadyExists => "User already exists",
            ErrorCode::InvalidEmail => "Invalid email format",
            ErrorCode::InvalidPassword => "Invalid password format",
            ErrorCode::UserDisabled => "User disabled",

            // 套餐相关错误
            ErrorCode::PlanNotFound => "Plan not found",
            ErrorCode::PlanUnavailable => "Plan unavailable",
            ErrorCode::PlanQuotaExceeded => "Plan quota exceeded",

            // 优惠券相关错误
            ErrorCode::CouponNotFound => "Coupon not found",
            ErrorCode::CouponExpired => "Coupon expired",
            ErrorCode::CouponUsed => "Coupon used",
            ErrorCode::CouponInvalid => "Coupon invalid",

            // 订单相关错误
            ErrorCode::OrderNotFound => "Order not found",
            ErrorCode::OrderAlreadyPaid => "Order already paid",
            ErrorCode::OrderExpired => "Order expired",
            ErrorCode::PaymentFailed => "Payment failed",
            ErrorCode::InsufficientBalance => "Insufficient balance",
        }
    }

    /// 判断是否为成功状态
    pub fn is_success(&self) -> bool {
        matches!(self, ErrorCode::Success)
    }

    /// 判断是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        let code = self.code();
        (2000..3000).contains(&code) || (5000..7000).contains(&code)
    }

    /// 判断是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        let code = self.code();
        (1000..2000).contains(&code) || (4000..5000).contains(&code)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> Self {
        match code {
            1000 => ErrorCode::Success,
            1001 => ErrorCode::InternalError,
            1002 => ErrorCode::InvalidParams,
            1003 => ErrorCode::ValidationError,
            1004 => ErrorCode::DatabaseError,
            1005 => ErrorCode::NetworkError,
            2000 => ErrorCode::Unauthorized,
            2001 => ErrorCode::InvalidToken,
            2002 => ErrorCode::TokenExpired,
            2003 => ErrorCode::InvalidCredentials,
            2004 => ErrorCode::AccountLocked,
            2005 => ErrorCode::PermissionDenied,
            3000 => ErrorCode::UserNotFound,
            3001 => ErrorCode::UserAlreadyExists,
            3002 => ErrorCode::InvalidEmail,
            3003 => ErrorCode::InvalidPassword,
            3004 => ErrorCode::UserDisabled,
            4000 => ErrorCode::PlanNotFound,
            4001 => ErrorCode::PlanUnavailable,
            4002 => ErrorCode::PlanQuotaExceeded,
            5000 => ErrorCode::CouponNotFound,
            5001 => ErrorCode::CouponExpired,
            5002 => ErrorCode::CouponUsed,
            5003 => ErrorCode::CouponInvalid,
            6000 => ErrorCode::OrderNotFound,
            6001 => ErrorCode::OrderAlreadyPaid,
            6002 => ErrorCode::OrderExpired,
            6003 => ErrorCode::PaymentFailed,
            6004 => ErrorCode::InsufficientBalance,
            _ => ErrorCode::InternalError,
        }
    }
} 