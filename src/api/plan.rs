use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::plan::{CreatePlanRequest, Plan, PlanListResponse, PlanResponse, UpdatePlanRequest},
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
