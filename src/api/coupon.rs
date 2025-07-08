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
#[utoipa::path(
    post,
    path = "/api/coupons",
    tag = "coupons",
    request_body = CreateCouponRequest,
    responses(
        (status = 200, description = "Coupon created successfully", body = CouponResponse),
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
#[utoipa::path(
    get,
    path = "/api/coupons",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupons retrieved successfully", body = CouponListResponse),
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
        (status = 200, description = "Coupon found", body = CouponResponse),
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
        (status = 200, description = "Coupon updated successfully", body = CouponResponse),
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
        (status = 200, description = "Coupon deleted successfully"),
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
        (status = 200, description = "验证优惠码成功", body = CouponResponse),
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
