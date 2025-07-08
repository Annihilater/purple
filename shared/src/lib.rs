use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// 重新导出常用类型
pub use chrono;
pub use uuid;
pub use validator;

// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ResponseMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMeta {
    pub timestamp: i64,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

// 用户相关类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub is_admin: bool,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "邮箱格式无效"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码至少6位"))]
    pub password: String,
    pub username: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub is_admin: Option<bool>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "邮箱格式无效"))]
    pub email: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

// 套餐相关类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub duration_days: i32,
    pub traffic_limit: i64,
    pub device_limit: i32,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePlanRequest {
    #[validate(length(min = 1, message = "套餐名称不能为空"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0, message = "价格不能为负数"))]
    pub price: i32,
    #[validate(range(min = 1, message = "持续天数至少为1天"))]
    pub duration_days: i32,
    #[validate(range(min = 0, message = "流量限制不能为负数"))]
    pub traffic_limit: i64,
    #[validate(range(min = 1, message = "设备限制至少为1"))]
    pub device_limit: i32,
    pub is_enabled: Option<bool>,
}

// 优惠券相关类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: i32,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i32,
    pub min_amount: Option<i32>,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCouponRequest {
    #[validate(length(min = 1, message = "优惠券代码不能为空"))]
    pub code: String,
    pub description: Option<String>,
    #[validate(length(min = 1, message = "折扣类型不能为空"))]
    pub discount_type: String,
    #[validate(range(min = 0, message = "折扣值不能为负数"))]
    pub discount_value: i32,
    pub min_amount: Option<i32>,
    pub max_uses: Option<i32>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub is_enabled: Option<bool>,
}

// 错误代码枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    // 通用错误 1000-1999
    InternalError,
    ValidationError,
    InvalidParams,
    DatabaseError,

    // 认证错误 2000-2999
    Unauthorized,
    InvalidToken,
    TokenExpired,
    InvalidCredentials,

    // 用户错误 3000-3999
    UserNotFound,
    UserAlreadyExists,
    UserDisabled,

    // 套餐错误 4000-4999
    PlanNotFound,
    PlanDisabled,
    PlanNotAvailable,

    // 优惠券错误 5000-5999
    CouponNotFound,
    CouponExpired,
    CouponDisabled,
    CouponExhausted,
    CouponNotApplicable,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        match self {
            ErrorCode::InternalError => 1000,
            ErrorCode::ValidationError => 1001,
            ErrorCode::InvalidParams => 1002,
            ErrorCode::DatabaseError => 1003,

            ErrorCode::Unauthorized => 2000,
            ErrorCode::InvalidToken => 2001,
            ErrorCode::TokenExpired => 2002,
            ErrorCode::InvalidCredentials => 2003,

            ErrorCode::UserNotFound => 3000,
            ErrorCode::UserAlreadyExists => 3001,
            ErrorCode::UserDisabled => 3002,

            ErrorCode::PlanNotFound => 4000,
            ErrorCode::PlanDisabled => 4001,
            ErrorCode::PlanNotAvailable => 4002,

            ErrorCode::CouponNotFound => 5000,
            ErrorCode::CouponExpired => 5001,
            ErrorCode::CouponDisabled => 5002,
            ErrorCode::CouponExhausted => 5003,
            ErrorCode::CouponNotApplicable => 5004,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::InternalError => "内部服务器错误",
            ErrorCode::ValidationError => "数据验证失败",
            ErrorCode::InvalidParams => "请求参数无效",
            ErrorCode::DatabaseError => "数据库操作失败",

            ErrorCode::Unauthorized => "未授权访问",
            ErrorCode::InvalidToken => "无效的访问令牌",
            ErrorCode::TokenExpired => "访问令牌已过期",
            ErrorCode::InvalidCredentials => "用户名或密码错误",

            ErrorCode::UserNotFound => "用户不存在",
            ErrorCode::UserAlreadyExists => "用户已存在",
            ErrorCode::UserDisabled => "用户账户已禁用",

            ErrorCode::PlanNotFound => "套餐不存在",
            ErrorCode::PlanDisabled => "套餐已禁用",
            ErrorCode::PlanNotAvailable => "套餐暂不可用",

            ErrorCode::CouponNotFound => "优惠券不存在",
            ErrorCode::CouponExpired => "优惠券已过期",
            ErrorCode::CouponDisabled => "优惠券已禁用",
            ErrorCode::CouponExhausted => "优惠券已用完",
            ErrorCode::CouponNotApplicable => "优惠券不适用",
        }
    }
}
