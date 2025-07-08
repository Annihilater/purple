//! 具体的 API 响应类型定义
//!
//! 为 OpenAPI 文档生成提供具体的响应类型，避免泛型类型导致的 Schema 问题。
//! 所有响应类型都基于统一的响应格式标准。

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::response_v2::{ErrorDetail, PaginationMeta, ResponseMeta};
use crate::models::{auth::TokenResponse, coupon::CouponResponse, plan::PlanResponse, user::User};

/// 空响应（用于错误响应或无数据响应）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmptyApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 错误信息（仅在失败时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 健康检查数据
    pub data: HealthData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 健康检查数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthData {
    /// 状态
    pub status: String,
    /// 时间戳
    pub timestamp: i64,
    /// 版本信息
    pub version: String,
}

/// 项目信息响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectInfoApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 项目信息数据
    pub data: ProjectInfoData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 项目信息数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectInfoData {
    /// 项目名称
    pub name: String,
    /// 项目版本
    pub version: String,
    /// 项目描述
    pub description: String,
    /// API 版本
    pub api_version: String,
}

/// 认证令牌响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 令牌数据
    pub data: TokenResponse,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 用户 ID 响应（用于创建用户等操作）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserIdApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 用户 ID 数据
    pub data: UserIdData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 用户 ID 数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserIdData {
    /// 用户 ID
    pub id: i32,
}

/// 单个用户响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 用户数据
    pub data: User,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 用户分页响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPageApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 用户列表数据
    pub data: Vec<User>,
    /// 分页信息
    pub pagination: PaginationMeta,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 单个套餐响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 套餐数据
    pub data: PlanResponse,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐分页响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanPageApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 套餐列表数据
    pub data: Vec<PlanResponse>,
    /// 分页信息
    pub pagination: PaginationMeta,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐列表响应（不分页）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanListApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 套餐列表数据
    pub data: Vec<PlanResponse>,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐统计响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanStatsApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 统计数据
    pub data: PlanStatsData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐统计数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanStatsData {
    /// 总数
    pub total: i64,
    /// 启用数量
    pub enabled: i64,
    /// 禁用数量
    pub disabled: i64,
}

/// 套餐价格响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanPricingApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 价格数据
    pub data: PlanPricingData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐价格数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanPricingData {
    /// 套餐 ID
    pub plan_id: i32,
    /// 月付价格（分）
    pub monthly_price: i32,
    /// 季付价格（分）
    pub quarterly_price: Option<i32>,
    /// 年付价格（分）
    pub yearly_price: Option<i32>,
    /// 折扣信息
    pub discount: Option<String>,
}

/// 套餐可用性响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanAvailabilityApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 可用性数据
    pub data: PlanAvailabilityData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 套餐可用性数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanAvailabilityData {
    /// 套餐 ID
    pub plan_id: i32,
    /// 是否可用
    pub available: bool,
    /// 原因（不可用时）
    pub reason: Option<String>,
}

/// 批量更新结果响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchUpdateApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 批量更新结果
    pub data: BatchUpdateData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 批量更新数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchUpdateData {
    /// 更新总数
    pub total: i32,
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 失败的 ID 列表
    pub failed_ids: Vec<i32>,
}

/// 单个优惠券响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 优惠券数据
    pub data: CouponResponse,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 优惠券分页响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponPageApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 优惠券列表数据
    pub data: Vec<CouponResponse>,
    /// 分页信息
    pub pagination: PaginationMeta,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 优惠券验证响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponValidationApiResponse {
    /// 操作是否成功
    pub success: bool,
    /// 验证结果
    pub data: CouponValidationData,
    /// 元数据
    pub meta: ResponseMeta,
}

/// 优惠券验证数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CouponValidationData {
    /// 优惠券 ID
    pub coupon_id: i32,
    /// 优惠券代码
    pub code: String,
    /// 是否有效
    pub valid: bool,
    /// 折扣金额（分）
    pub discount_amount: i32,
    /// 折扣类型（1: 固定金额, 2: 百分比）
    pub discount_type: i32,
    /// 过期时间
    pub expires_at: i32,
}
