use crate::api::response::ApiResponse;
use actix_web::{get, HttpResponse};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "服务健康检查", body = ApiResponse<String>)
    ),
    tag = "health"
)]
#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("Service is healthy".to_string()))
}
