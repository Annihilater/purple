use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::coupon::{Coupon, CouponResponse, CreateCouponRequest, UpdateCouponRequest},
    repositories::CouponRepository,
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetCouponsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    #[serde(default = "default_false")]
    pub only_enabled: bool,
    #[serde(default = "default_false")]
    pub only_valid: bool,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

fn default_false() -> bool {
    false
}

/// 创建优惠券
///
/// 创建新的优惠券，支持设置优惠金额、使用限制、有效期等参数。
/// 需要管理员权限。
///
/// # 参数
///
/// - `coupon`: 优惠券创建请求数据
/// - `repo`: 优惠券仓库
///
/// # 返回
///
/// 成功时返回创建的优惠券信息，使用统一的 ApiResponse 格式。
/// 失败时返回相应的错误响应。
///
/// # 示例
///
/// ```json
/// // 请求
/// POST /api/coupons
/// {
///   "name": "新用户优惠",
///   "code": "NEWUSER2024",
///   "type": 1,
///   "value": 1000,
///   "started_at": 1704067200,
///   "ended_at": 1735689600,
///   "limit_use": 100,
///   "show": true
/// }
///
/// // 响应
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "name": "新用户优惠",
///     "code": "NEWUSER2024",
///     "type": 1,
///     "value": 1000,
///     "started_at": 1704067200,
///     "ended_at": 1735689600,
///     "limit_use": 100,
///     "show": true,
///     "created_at": 1704067200,
///     "updated_at": 1704067200
///   },
///   "meta": {
///     "timestamp": 1751938399,
///     "request_id": "uuid-here"
///   }
/// }
/// ```
#[utoipa::path(
    post,
    path = "/api/coupons",
    tag = "coupons",
    request_body = CreateCouponRequest,
    responses(
        (status = 200, description = "Coupon created successfully", body = CouponApiResponse),
        (status = 400, description = "Invalid request", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("")]
pub async fn create_coupon(
    coupon: web::Json<CreateCouponRequest>,
    repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = coupon.validate() {
        return Err(ApiError::from(validation_errors));
    }

    match repo.create(&coupon.into_inner()).await {
        Ok(coupon) => {
            let response = ApiResponse::success(CouponResponse::from(coupon));
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建优惠券失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取优惠券列表
///
/// 支持分页查询优惠券列表，可以通过参数过滤启用状态和有效性。
/// 需要认证访问。
///
/// # 查询参数
///
/// - `page`: 页码，默认为 1
/// - `page_size`: 每页大小，默认为 10
/// - `only_enabled`: 仅显示启用的优惠券，默认为 false
/// - `only_valid`: 仅显示有效的优惠券，默认为 false
///
/// # 返回
///
/// 返回分页的优惠券列表，使用统一的分页响应格式。
/// 空数据时返回空数组和正确的分页信息。
///
/// # 示例
///
/// ```bash
/// # 带认证的请求
/// curl 'http://127.0.0.1:8080/api/coupons?page=1&page_size=10&only_enabled=true' \
///   -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...'
/// ```
///
/// ```json
/// // 成功响应（有数据）
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 1,
///       "name": "新用户优惠",
///       "code": "NEWUSER2024",
///       "type": 1,
///       "value": 1000,
///       "show": true
///     }
///   ],
///   "pagination": {
///     "page": 1,
///     "page_size": 10,
///     "total": 1,
///     "total_pages": 1,
///     "has_next": false,
///     "has_prev": false
///   },
///   "meta": {
///     "timestamp": 1751938399,
///     "request_id": "uuid-here"
///   }
/// }
///
/// // 成功响应（空数据）
/// {
///   "success": true,
///   "data": [],
///   "pagination": {
///     "page": 1,
///     "page_size": 10,
///     "total": 0,
///     "total_pages": 0,
///     "has_next": false,
///     "has_prev": false
///   },
///   "meta": {
///     "timestamp": 1751938399,
///     "request_id": "uuid-here"
///   }
/// }
/// ```
///
/// # 注意事项
///
/// - 该接口需要 JWT 认证
/// - 空数据时返回 200 状态码而非 401，确保认证用户能正确获取空结果
/// - 路由配置使用相对路径 `""` 配合 scope `/api/coupons`
#[utoipa::path(
    get,
    path = "/api/coupons",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupons retrieved successfully", body = CouponPageApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("page" = i64, Query, description = "Page number (default: 1)"),
        ("page_size" = i64, Query, description = "Page size (default: 10)"),
        ("only_enabled" = bool, Query, description = "Only show enabled coupons (default: false)"),
        ("only_valid" = bool, Query, description = "Only show valid coupons (default: false)")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("")]
pub async fn list_coupons(
    query: web::Query<GetCouponsQuery>,
    repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    let page = query.page;
    let page_size = query.page_size;
    let only_enabled = query.only_enabled;
    let only_valid = query.only_valid;

    match repo
        .list(page as i64, page_size as i64, only_enabled, only_valid)
        .await
    {
        Ok((coupons, total)) => {
            let coupons = coupons.into_iter().map(CouponResponse::from).collect();
            let response = ApiResponse::page(coupons, page as u64, page_size as u64, total as u64);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取优惠券列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取优惠券信息
#[utoipa::path(
    get,
    path = "/api/coupons/{id}",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupon found", body = CouponApiResponse),
        (status = 404, description = "Coupon not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/{id}")]
pub async fn get_coupon(
    id: web::Path<i32>,
    repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    let coupon_id = id.into_inner();

    match repo.find_by_id(coupon_id).await {
        Ok(Some(coupon)) => {
            let response = ApiResponse::success(CouponResponse::from(coupon));
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::CouponNotFound)),
        Err(e) => {
            tracing::error!("获取优惠券失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 更新优惠券信息
#[utoipa::path(
    put,
    path = "/api/coupons/{id}",
    tag = "coupons",
    request_body = UpdateCouponRequest,
    responses(
        (status = 200, description = "Coupon updated successfully", body = CouponApiResponse),
        (status = 400, description = "Invalid request", body = EmptyApiResponse),
        (status = 404, description = "Coupon not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/{id}")]
pub async fn update_coupon(
    id: web::Path<i32>,
    coupon: web::Json<UpdateCouponRequest>,
    repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = coupon.validate() {
        return Err(ApiError::from(validation_errors));
    }

    let coupon_id = id.into_inner();
    match repo.update(coupon_id, &coupon.into_inner()).await {
        Ok(coupon) => {
            let response = ApiResponse::success(CouponResponse::from(coupon));
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("更新优惠券失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 删除优惠券
#[utoipa::path(
    delete,
    path = "/api/coupons/{id}",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupon deleted successfully", body = EmptyApiResponse),
        (status = 404, description = "Coupon not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_coupon(
    id: web::Path<i32>,
    repo: web::Data<CouponRepository>,
) -> Result<HttpResponse, ApiError> {
    let coupon_id = id.into_inner();
    match repo.delete(coupon_id).await {
        Ok(_) => {
            let response = ApiResponse::success(());
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("删除优惠券失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 验证优惠码
#[utoipa::path(
    get,
    path = "/coupons/verify/{code}",
    tag = "coupons",
    params(
        ("code" = String, Path, description = "优惠码"),
    ),
    responses(
        (status = 200, description = "验证优惠码成功", body = CouponApiResponse),
        (status = 404, description = "优惠码不存在或已失效", body = EmptyApiResponse),
        (status = 500, description = "服务器内部错误", body = EmptyApiResponse),
    )
)]
#[get("/coupons/verify/{code}")]
pub async fn verify_coupon(
    coupon_repo: web::Data<CouponRepository>,
    code: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let code = code.into_inner();

    match coupon_repo.find_by_code(&code).await {
        Ok(Some(coupon)) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;

            // 检查优惠券是否可用
            if !coupon.show {
                return Err(ApiError::with_details(
                    ErrorCode::CouponDisabled,
                    "优惠券已停用".to_string(),
                ));
            }

            // 检查使用次数
            if let Some(usage_limit) = coupon.limit_use {
                // 这里简化处理，因为数据库表中没有 used_count 字段
                // 实际应用中需要另外的表来跟踪使用次数
            }

            // 检查有效期
            if now < coupon.started_at {
                return Err(ApiError::with_details(
                    ErrorCode::CouponNotValid,
                    "优惠券尚未生效".to_string(),
                ));
            }

            if now > coupon.ended_at {
                return Err(ApiError::with_details(
                    ErrorCode::CouponExpired,
                    "优惠券已过期".to_string(),
                ));
            }

            let response = ApiResponse::success(CouponResponse::from(coupon));
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::CouponNotFound)),
        Err(e) => {
            tracing::error!("查找优惠券失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ListCouponsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub only_enabled: Option<bool>,
    pub only_valid: Option<bool>,
}

#[derive(Debug, serde::Deserialize, Validate)]
pub struct ValidateCouponRequest {
    #[validate(length(min = 1, max = 50))]
    pub code: String,
    #[validate(range(min = 1))]
    pub amount: i32,
}
