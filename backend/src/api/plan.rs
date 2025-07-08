use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::plan::{CreatePlanRequest, PlanListResponse, PlanResponse, UpdatePlanRequest},
    repositories::PlanRepository,
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetPlansQuery {
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

#[derive(Debug, serde::Deserialize)]
pub struct ListPlansQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub only_enabled: Option<bool>,
}

/// 创建套餐
#[utoipa::path(
    post,
    path = "/api/plans",
    tag = "plans",
    request_body = CreatePlanRequest,
    responses(
        (status = 200, description = "Plan created successfully", body = PlanResponse),
        (status = 400, description = "Invalid request", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[post("/api/plans")]
pub async fn create_plan(
    plan: web::Json<CreatePlanRequest>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = plan.validate() {
        return Err(ApiError::from(validation_errors));
    }

    match repo.get_ref().create(&plan.into_inner()).await {
        Ok(plan) => {
            let response = ApiResponse::success(PlanResponse::from(plan));
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建套餐失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取套餐列表
#[utoipa::path(
    get,
    path = "/api/plans",
    tag = "plans",
    responses(
        (status = 200, description = "Plans retrieved successfully", body = PlanListResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("page" = i64, Query, description = "Page number (default: 1)"),
        ("page_size" = i64, Query, description = "Page size (default: 10)"),
        ("only_enabled" = bool, Query, description = "Only show enabled plans (default: false)")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans")]
pub async fn list_plans(
    query: web::Query<ListPlansQuery>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let only_enabled = query.only_enabled.unwrap_or(false);

    let (plans, total) = if only_enabled {
        match repo.get_ref().find_enabled().await {
            Ok(plans) => {
                let total = plans.len() as i64;
                (plans, total)
            }
            Err(e) => {
                tracing::error!("获取已启用套餐失败: {}", e);
                return Err(ApiError::with_details(
                    ErrorCode::DatabaseError,
                    "数据库操作失败".to_string(),
                ));
            }
        }
    } else {
        match repo.get_ref().find_all(page as i32, page_size as i32).await {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("获取套餐列表失败: {}", e);
                return Err(ApiError::with_details(
                    ErrorCode::DatabaseError,
                    "数据库操作失败".to_string(),
                ));
            }
        }
    };

    let plans = plans.into_iter().map(PlanResponse::from).collect();
    let response = ApiResponse::success(PlanListResponse { plans, total });
    Ok(response.into_http_response())
}

/// 获取套餐信息
#[utoipa::path(
    get,
    path = "/api/plans/{id}",
    tag = "plans",
    responses(
        (status = 200, description = "Plan found", body = PlanResponse),
        (status = 404, description = "Plan not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Plan id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans/{id}")]
pub async fn get_plan(
    id: web::Path<i32>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    let plan_id = id.into_inner();

    match repo.get_ref().find_by_id(plan_id).await {
        Ok(Some(plan)) => {
            let response = ApiResponse::success(PlanResponse::from(plan));
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::PlanNotFound)),
        Err(e) => {
            tracing::error!("获取套餐失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 更新套餐信息
#[utoipa::path(
    put,
    path = "/api/plans/{id}",
    tag = "plans",
    request_body = UpdatePlanRequest,
    responses(
        (status = 200, description = "Plan updated successfully", body = PlanResponse),
        (status = 400, description = "Invalid request", body = EmptyApiResponse),
        (status = 404, description = "Plan not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Plan id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/api/plans/{id}")]
pub async fn update_plan(
    id: web::Path<i32>,
    plan: web::Json<UpdatePlanRequest>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = plan.validate() {
        return Err(ApiError::from(validation_errors));
    }

    let plan_id = id.into_inner();
    match repo.get_ref().update(plan_id, &plan.into_inner()).await {
        Ok(plan) => {
            let response = ApiResponse::success(PlanResponse::from(plan));
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("更新套餐失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 删除套餐
#[utoipa::path(
    delete,
    path = "/api/plans/{id}",
    tag = "plans",
    responses(
        (status = 200, description = "Plan deleted successfully"),
        (status = 404, description = "Plan not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Plan id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[delete("/api/plans/{id}")]
pub async fn delete_plan(
    id: web::Path<i32>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    let plan_id = id.into_inner();
    match repo.get_ref().delete(plan_id).await {
        Ok(_) => {
            let response = ApiResponse::success(());
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("删除套餐失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取已启用的套餐列表
#[utoipa::path(
    get,
    path = "/api/plans/enabled",
    tag = "plans",
    responses(
        (status = 200, description = "Plans retrieved successfully", body = PlanListResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans/enabled")]
pub async fn get_enabled_plans(repo: web::Data<PlanRepository>) -> Result<HttpResponse, ApiError> {
    match repo.get_ref().find_enabled().await {
        Ok(plans) => {
            let total = plans.len() as i64;
            let plans = plans.into_iter().map(PlanResponse::from).collect();
            let response = ApiResponse::success(PlanListResponse { plans, total });
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取已启用套餐失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取套餐统计信息
#[utoipa::path(
    get,
    path = "/api/plans/stats",
    tag = "plans",
    responses(
        (status = 200, description = "Plan statistics retrieved successfully", body = PlanStatsApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans/stats")]
pub async fn get_plan_stats(repo: web::Data<PlanRepository>) -> Result<HttpResponse, ApiError> {
    // 获取所有套餐进行统计
    match repo.get_ref().find_all(1, 1000).await {
        Ok((plans, total)) => {
            let active_plans = plans.iter().filter(|p| p.show).count() as i64;
            let hidden_plans = plans.iter().filter(|p| !p.show).count() as i64;
            let renewable_plans = plans.iter().filter(|p| p.renew).count() as i64;

            let stats = crate::models::plan::PlanStats {
                total_plans: total,
                active_plans,
                hidden_plans,
                renewable_plans,
            };

            let response = ApiResponse::success(stats);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取套餐统计失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 获取套餐价格信息
#[utoipa::path(
    get,
    path = "/api/plans/{id}/pricing",
    tag = "plans",
    responses(
        (status = 200, description = "Plan pricing information retrieved successfully", body = PlanPricingApiResponse),
        (status = 404, description = "Plan not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Plan id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans/{id}/pricing")]
pub async fn get_plan_pricing(
    id: web::Path<i32>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    let plan_id = id.into_inner();

    match repo.get_ref().find_by_id(plan_id).await {
        Ok(Some(plan)) => {
            let mut pricing = Vec::new();

            // 添加各种周期的价格选项
            if let Some(price) = plan.month_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "month".to_string(),
                    period_days: 30,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.quarter_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "quarter".to_string(),
                    period_days: 90,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.half_year_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "half_year".to_string(),
                    period_days: 180,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.year_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "year".to_string(),
                    period_days: 365,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.two_year_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "two_year".to_string(),
                    period_days: 730,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.three_year_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "three_year".to_string(),
                    period_days: 1095,
                    price,
                    available: true,
                });
            }

            if let Some(price) = plan.onetime_price {
                pricing.push(crate::models::plan::PriceOption {
                    period_type: "onetime".to_string(),
                    period_days: 0,
                    price,
                    available: true,
                });
            }

            let plan_pricing = crate::models::plan::PlanPricing {
                id: plan.id,
                name: plan.name,
                pricing,
            };

            let response = ApiResponse::success(plan_pricing);
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::PlanNotFound)),
        Err(e) => {
            tracing::error!("获取套餐价格信息失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 检查套餐可用性
#[utoipa::path(
    get,
    path = "/api/plans/{id}/availability",
    tag = "plans",
    responses(
        (status = 200, description = "Plan availability checked successfully", body = PlanAvailabilityApiResponse),
        (status = 404, description = "Plan not found", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    params(
        ("id" = i32, Path, description = "Plan id")
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[get("/api/plans/{id}/availability")]
pub async fn check_plan_availability(
    id: web::Path<i32>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    let plan_id = id.into_inner();

    match repo.get_ref().find_by_id(plan_id).await {
        Ok(Some(plan)) => {
            let (available, reason) = if !plan.show {
                (false, Some("套餐已隐藏".to_string()))
            } else if plan.month_price.is_none()
                && plan.quarter_price.is_none()
                && plan.half_year_price.is_none()
                && plan.year_price.is_none()
                && plan.two_year_price.is_none()
                && plan.three_year_price.is_none()
                && plan.onetime_price.is_none()
            {
                (false, Some("套餐未设置价格".to_string()))
            } else {
                (true, None)
            };

            let availability = crate::models::plan::PlanAvailability {
                id: plan.id,
                name: plan.name,
                available,
                reason,
            };

            let response = ApiResponse::success(availability);
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::PlanNotFound)),
        Err(e) => {
            tracing::error!("检查套餐可用性失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 批量更新套餐状态
#[utoipa::path(
    put,
    path = "/api/plans/batch/status",
    tag = "plans",
    request_body = BatchUpdatePlanStatusRequest,
    responses(
        (status = 200, description = "Plans status updated successfully", body = EmptyApiResponse),
        (status = 400, description = "Invalid request", body = EmptyApiResponse),
        (status = 500, description = "Internal server error", body = EmptyApiResponse)
    ),
    security(
        ("jwt_token" = [])
    )
)]
#[put("/api/plans/batch/status")]
pub async fn batch_update_plan_status(
    request: web::Json<BatchUpdatePlanStatusRequest>,
    repo: web::Data<PlanRepository>,
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    let plan_ids = &request.plan_ids;
    let show = request.show;

    // 这里需要在 repository 中实现批量更新方法
    // 目前暂时使用循环更新
    let mut updated_count = 0;

    for &plan_id in plan_ids {
        let update_request = crate::models::plan::UpdatePlanRequest {
            group_id: None,
            transfer_enable: None,
            name: None,
            speed_limit: None,
            show: Some(show),
            sort: None,
            renew: None,
            content: None,
            month_price: None,
            quarter_price: None,
            half_year_price: None,
            year_price: None,
            two_year_price: None,
            three_year_price: None,
            onetime_price: None,
            reset_price: None,
            reset_traffic_method: None,
            capacity_limit: None,
            daily_unit_price: None,
            transfer_unit_price: None,
        };

        match repo.get_ref().update(plan_id, &update_request).await {
            Ok(_) => updated_count += 1,
            Err(e) => {
                tracing::error!("批量更新套餐状态失败, plan_id: {}, error: {}", plan_id, e);
            }
        }
    }

    let response = ApiResponse::success(());
    Ok(response.into_http_response())
}

/// 批量更新套餐状态请求
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct BatchUpdatePlanStatusRequest {
    /// 套餐ID列表
    #[validate(length(min = 1))]
    pub plan_ids: Vec<i32>,
    /// 显示状态
    pub show: bool,
}
