use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Coupon {
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub r#type: bool,
    pub value: i32,
    pub show: bool,
    pub limit_use: Option<i32>,
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: i32,
    pub ended_at: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "discount_type", rename_all = "snake_case")]
pub enum DiscountType {
    Fixed,   // 固定金额
    Percent, // 百分比
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCouponRequest {
    #[validate(length(min = 1, max = 255))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: bool,
    #[validate(range(min = 1))]
    pub value: i32,
    pub show: bool,
    #[validate(range(min = 0))]
    pub limit_use: Option<i32>,
    #[validate(range(min = 0))]
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: i32,
    pub ended_at: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCouponRequest {
    #[validate(length(min = 1, max = 255))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<bool>,
    #[validate(range(min = 1))]
    pub value: Option<i32>,
    pub show: Option<bool>,
    #[validate(range(min = 0))]
    pub limit_use: Option<i32>,
    #[validate(range(min = 0))]
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: Option<i32>,
    pub ended_at: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponResponse {
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: bool,
    pub value: i32,
    pub show: bool,
    pub limit_use: Option<i32>,
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: i32,
    pub ended_at: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

impl From<Coupon> for CouponResponse {
    fn from(coupon: Coupon) -> Self {
        Self {
            id: coupon.id,
            code: coupon.code,
            name: coupon.name,
            r#type: coupon.r#type,
            value: coupon.value,
            show: coupon.show,
            limit_use: coupon.limit_use,
            limit_use_with_user: coupon.limit_use_with_user,
            limit_plan_ids: coupon.limit_plan_ids,
            limit_period: coupon.limit_period,
            started_at: coupon.started_at,
            ended_at: coupon.ended_at,
            created_at: coupon.created_at,
            updated_at: coupon.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponListResponse {
    pub coupons: Vec<CouponResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidateCouponResponse {
    pub is_valid: bool,
    pub message: Option<String>,
    pub discount_amount: Option<i32>, // 实际折扣金额（分）
}
