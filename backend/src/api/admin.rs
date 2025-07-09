use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::coupon::{CouponResponse, CreateCouponRequest, UpdateCouponRequest},
    models::user::UserResponse,
    repositories::{CouponRepository, UserRepository},
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct AdminStatsQuery {
    #[serde(default = "default_days")]
    pub days: i32,
}

fn default_days() -> i32 {
    7
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminStats {
    pub total_users: i64,
    pub total_coupons: i64,
    pub active_coupons: i64,
    pub total_orders: i64,
    pub total_revenue: i64,
    pub today_registrations: i64,
    pub today_orders: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminDashboard {
    pub stats: AdminStats,
    pub recent_users: Vec<UserResponse>,
    pub recent_coupons: Vec<CouponResponse>,
}

/// 获取管理员仪表板数据
///
/// 获取管理后台首页的统计数据和最新信息，包括用户统计、优惠券统计、
/// 订单统计等关键指标。
#[utoipa::path(
    get,
    path = "/dashboard",
    tag = "admin",
    operation_id = "get_admin_dashboard",
    responses(
        (status = 200, description = "获取仪表板数据成功", body = inline(crate::common::response_v2::ApiSuccessResponse<AdminDashboard>)),
        (status = 401, description = "未授权访问", body = crate::common::response_v2::ApiErrorResponse),
        (status = 403, description = "权限不足", body = crate::common::response_v2::ApiErrorResponse),
        (status = 500, description = "内部服务器错误", body = crate::common::response_v2::ApiErrorResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/dashboard")]
pub async fn get_admin_dashboard(
    user_repo: web::Data<UserRepository>,
    coupon_repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    // 获取用户统计
    let total_users = match user_repo.count_total().await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("获取用户总数失败: {}", e);
            0
        }
    };

    // 获取优惠券统计
    let (total_coupons, active_coupons) = match coupon_repo.get_stats().await {
        Ok((total, active)) => (total, active),
        Err(e) => {
            tracing::error!("获取优惠券统计失败: {}", e);
            (0, 0)
        }
    };

    // 获取最近注册用户
    let recent_users = match user_repo.list(1, 5, None, None).await {
        Ok((users, _)) => users.into_iter().map(UserResponse::from).collect(),
        Err(e) => {
            tracing::error!("获取最近用户失败: {}", e);
            vec![]
        }
    };

    // 获取最近创建的优惠券
    let recent_coupons = match coupon_repo.list(1, 5, false, false).await {
        Ok((coupons, _)) => coupons.into_iter().map(CouponResponse::from).collect(),
        Err(e) => {
            tracing::error!("获取最近优惠券失败: {}", e);
            vec![]
        }
    };

    let stats = AdminStats {
        total_users,
        total_coupons,
        active_coupons,
        total_orders: 0,        // TODO: 实现订单统计
        total_revenue: 0,       // TODO: 实现收入统计
        today_registrations: 0, // TODO: 实现今日注册统计
        today_orders: 0,        // TODO: 实现今日订单统计
    };

    let dashboard = AdminDashboard {
        stats,
        recent_users,
        recent_coupons,
    };

    let response = ApiResponse::success(dashboard);
    Ok(response.into_http_response())
}

/// 获取管理员统计数据
#[utoipa::path(
    get,
    path = "/stats",
    tag = "admin",
    operation_id = "get_admin_stats",
    responses(
        (status = 200, description = "获取统计数据成功", body = inline(crate::common::response_v2::ApiSuccessResponse<AdminStats>)),
        (status = 401, description = "未授权访问", body = crate::common::response_v2::ApiErrorResponse),
        (status = 500, description = "内部服务器错误", body = crate::common::response_v2::ApiErrorResponse)
    ),
    params(
        ("days" = i32, Query, description = "统计天数（默认7天）")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/stats")]
pub async fn get_admin_stats(
    query: web::Query<AdminStatsQuery>,
    user_repo: web::Data<UserRepository>,
    coupon_repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    let _days = query.days;

    // 获取各项统计数据
    let total_users = user_repo.count_total().await.unwrap_or(0);
    let (total_coupons, active_coupons) = coupon_repo.get_stats().await.unwrap_or((0, 0));

    let stats = AdminStats {
        total_users,
        total_coupons,
        active_coupons,
        total_orders: 0,        // TODO: 实现
        total_revenue: 0,       // TODO: 实现
        today_registrations: 0, // TODO: 实现
        today_orders: 0,        // TODO: 实现
    };

    let response = ApiResponse::success(stats);
    Ok(response.into_http_response())
}
