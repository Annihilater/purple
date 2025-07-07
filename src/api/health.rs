use actix_web::{get, HttpResponse, Result};
use serde::Serialize;
use utoipa::ToSchema;

use crate::common::response_v2::{ApiResponse, IntoHttpResponse};

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub uptime: String,
    pub version: String,
}

/// 健康检查
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "健康检查成功")
    )
)]
#[get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    let health = HealthResponse {
        status: "ok".to_string(),
        uptime: chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string(),
        version: "0.1.0".to_string(),
    };

    let response = ApiResponse::success(health);
    Ok(response.into_http_response())
}
