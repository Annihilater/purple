use actix_web::{get, HttpResponse, Result};

use crate::common::response::{ApiResponse, ProjectInfoApiResponse};
use crate::models::info::ProjectInfo;

/// 获取项目信息接口
///
/// 返回项目的基本信息，包括名称、版本、描述等
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "成功获取项目信息", body = ProjectInfoApiResponse),
        (status = 500, description = "服务器内部错误", body = ProjectInfoApiResponse)
    ),
    tag = "项目信息"
)]
#[get("/")]
pub async fn get_project_info() -> Result<HttpResponse> {
    let project_info = ProjectInfo::new();
    let response = ApiResponse::success(project_info);
    Ok(response.into_response())
}
