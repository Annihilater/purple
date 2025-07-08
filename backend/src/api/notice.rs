use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::notice::{
        CreateNoticeRequest, NoticeListResponse, NoticeResponse, UpdateNoticeRequest,
    },
    // repositories::NoticeRepository,
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetNoticesQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    /// 是否只显示已发布的公告
    pub show: Option<bool>,
    /// 标签过滤
    pub tag: Option<String>,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

/// 创建公告（管理员）
#[utoipa::path(
    post,
    path = "/api/notices",
    tag = "notices",
    request_body = CreateNoticeRequest,
    responses(
        (status = 200, description = "创建公告成功"),
        (status = 400, description = "请求参数无效"),
        (status = 403, description = "权限不足"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/notices")]
pub async fn create_notice(
    notice_request: web::Json<CreateNoticeRequest>,
    // repo: web::Data<NoticeRepository>,
    // TODO: 验证管理员权限
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = notice_request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    // TODO: 实现创建公告逻辑
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 获取公告列表
#[utoipa::path(
    get,
    path = "/api/notices",
    tag = "notices",
    params(
        GetNoticesQuery
    ),
    responses(
        (status = 200, description = "获取公告列表成功"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/notices")]
pub async fn get_notices(
    query: web::Query<GetNoticesQuery>,
    // repo: web::Data<NoticeRepository>,
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现获取公告列表
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 根据ID获取公告
#[utoipa::path(
    get,
    path = "/api/notices/{id}",
    tag = "notices",
    params(
        ("id" = i32, Path, description = "公告ID")
    ),
    responses(
        (status = 200, description = "获取公告成功"),
        (status = 404, description = "公告不存在"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/notices/{id}")]
pub async fn get_notice(
    id: web::Path<i32>,
    // repo: web::Data<NoticeRepository>,
) -> Result<HttpResponse, ApiError> {
    let notice_id = id.into_inner();
    // TODO: 实现获取公告详情
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 更新公告（管理员）
#[utoipa::path(
    put,
    path = "/api/notices/{id}",
    tag = "notices",
    params(
        ("id" = i32, Path, description = "公告ID")
    ),
    request_body = UpdateNoticeRequest,
    responses(
        (status = 200, description = "更新公告成功"),
        (status = 400, description = "请求参数无效"),
        (status = 403, description = "权限不足"),
        (status = 404, description = "公告不存在"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[put("/api/notices/{id}")]
pub async fn update_notice(
    id: web::Path<i32>,
    notice_request: web::Json<UpdateNoticeRequest>,
    // repo: web::Data<NoticeRepository>,
    // TODO: 验证管理员权限
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = notice_request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    let notice_id = id.into_inner();
    // TODO: 实现更新公告逻辑
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 删除公告（管理员）
#[utoipa::path(
    delete,
    path = "/api/notices/{id}",
    tag = "notices",
    params(
        ("id" = i32, Path, description = "公告ID")
    ),
    responses(
        (status = 200, description = "删除公告成功"),
        (status = 403, description = "权限不足"),
        (status = 404, description = "公告不存在"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[delete("/api/notices/{id}")]
pub async fn delete_notice(
    id: web::Path<i32>,
    // repo: web::Data<NoticeRepository>,
    // TODO: 验证管理员权限
) -> Result<HttpResponse, ApiError> {
    let notice_id = id.into_inner();
    // TODO: 实现删除公告逻辑
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 获取最新公告（用于首页显示）
#[utoipa::path(
    get,
    path = "/api/notices/latest",
    tag = "notices",
    params(
        ("limit" = Option<i32>, Query, description = "返回数量限制，默认5条")
    ),
    responses(
        (status = 200, description = "获取最新公告成功"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/notices/latest")]
pub async fn get_latest_notices(
    query: web::Query<serde_json::Value>,
    // repo: web::Data<NoticeRepository>,
) -> Result<HttpResponse, ApiError> {
    let limit = query.get("limit").and_then(|v| v.as_i64()).unwrap_or(5) as i32;

    // TODO: 实现获取最新公告逻辑
    Err(ApiError::new(ErrorCode::InternalError))
}
