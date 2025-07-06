use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    api::response::{ApiError, ApiResponse, Response},
    models::coupon::{
        Coupon, CouponListResponse, CouponResponse, CreateCouponRequest, UpdateCouponRequest,
    },
    repositories::CouponRepository,
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetCouponsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CouponsListResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<Vec<Coupon>>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

/// 创建优惠券
#[utoipa::path(
    post,
    path = "/api/coupons",
    tag = "coupons",
    request_body = CreateCouponRequest,
    responses(
        (status = 200, description = "Coupon created successfully", body = CouponResponse),
        (status = 400, description = "Invalid request", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/api/coupons")]
pub async fn create_coupon(
    coupon: web::Json<CreateCouponRequest>,
    repo: web::Data<CouponRepository>,
) -> Response<HttpResponse> {
    coupon.validate().map_err(ApiError::from)?;

    let coupon = repo
        .create(&coupon.into_inner())
        .await
        .map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(CouponResponse::from(coupon))))
}

/// 获取优惠券列表
#[utoipa::path(
    get,
    path = "/api/coupons",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupons retrieved successfully", body = CouponListResponse),
        (status = 500, description = "Internal server error", body = ResponseError)
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
#[get("/api/coupons")]
pub async fn list_coupons(
    query: web::Query<GetCouponsQuery>,
    repo: web::Data<CouponRepository>,
) -> Response<HttpResponse> {
    let page = query.page;
    let page_size = query.page_size;

    let (coupons, total) = repo
        .list(page as i64, page_size as i64, false, false)
        .await
        .map_err(ApiError::from)?;
    let coupons = coupons.into_iter().map(CouponResponse::from).collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(CouponListResponse { coupons, total })))
}

/// 获取优惠券信息
#[utoipa::path(
    get,
    path = "/api/coupons/{id}",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupon found", body = CouponResponse),
        (status = 404, description = "Coupon not found", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/coupons/{id}")]
pub async fn get_coupon(
    id: web::Path<i32>,
    repo: web::Data<CouponRepository>,
) -> Response<HttpResponse> {
    let coupon = repo
        .find_by_id(id.into_inner())
        .await
        .map_err(ApiError::from)?;

    match coupon {
        Some(coupon) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(CouponResponse::from(coupon))))
        }
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "Coupon not found".to_string(),
        ))),
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
        (status = 400, description = "Invalid request", body = ResponseError),
        (status = 404, description = "Coupon not found", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/api/coupons/{id}")]
pub async fn update_coupon(
    id: web::Path<i32>,
    coupon: web::Json<UpdateCouponRequest>,
    repo: web::Data<CouponRepository>,
) -> Response<HttpResponse> {
    coupon.validate().map_err(ApiError::from)?;

    let coupon = repo
        .update(id.into_inner(), &coupon.into_inner())
        .await
        .map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(CouponResponse::from(coupon))))
}

/// 删除优惠券
#[utoipa::path(
    delete,
    path = "/api/coupons/{id}",
    tag = "coupons",
    responses(
        (status = 200, description = "Coupon deleted successfully"),
        (status = 404, description = "Coupon not found", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    ),
    params(
        ("id" = i32, Path, description = "Coupon id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/api/coupons/{id}")]
pub async fn delete_coupon(
    id: web::Path<i32>,
    repo: web::Data<CouponRepository>,
) -> Response<HttpResponse> {
    repo.delete(id.into_inner()).await.map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(())))
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
        (status = 404, description = "优惠码不存在或已失效", body = Response),
        (status = 500, description = "服务器内部错误", body = Response),
    )
)]
#[get("/coupons/verify/{code}")]
pub async fn verify_coupon(
    coupon_repo: web::Data<CouponRepository>,
    code: web::Path<String>,
) -> Response<HttpResponse> {
    match coupon_repo
        .find_by_code(&code)
        .await
        .map_err(ApiError::from)?
    {
        Some(coupon) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;

            // 检查优惠券是否可用
            if !coupon.show {
                return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    404,
                    "Coupon is disabled".to_string(),
                )));
            }

            // 检查使用次数
            if let Some(usage_limit) = coupon.limit_use {
                // 这里简化处理，因为数据库表中没有 used_count 字段
                // 实际应用中需要另外的表来跟踪使用次数
            }

            // 检查有效期
            if now < coupon.started_at {
                return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    404,
                    "Coupon is not yet valid".to_string(),
                )));
            }

            if now > coupon.ended_at {
                return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    404,
                    "Coupon has expired".to_string(),
                )));
            }

            Ok(HttpResponse::Ok().json(ApiResponse::success(CouponResponse::from(coupon))))
        }
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "Coupon not found".to_string(),
        ))),
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
