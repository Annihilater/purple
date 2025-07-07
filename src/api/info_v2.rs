use actix_web::{get, HttpResponse, Result};

use crate::common::response_new::{ApiResponse, IntoHttpResponse};
use crate::models::info::ProjectInfo;

/// 获取项目信息接口 (使用新的响应格式)
///
/// 这是使用新响应格式的示例接口
#[get("/v2")]
pub async fn get_project_info_v2() -> Result<HttpResponse> {
    let project_info = ProjectInfo::new();
    let response = ApiResponse::success(project_info);
    Ok(response.into_http_response())
}

/// 错误响应示例接口
#[get("/v2/error")]
pub async fn error_example() -> Result<HttpResponse> {
    use crate::common::{response_new::ApiResponse, ErrorCode};
    
    let response = ApiResponse::error(ErrorCode::UserNotFound);
    Ok(response.into_http_response())
}

/// 分页响应示例接口
#[get("/v2/users")]
pub async fn users_page_example() -> Result<HttpResponse> {
    use crate::common::response_new::ApiResponse;
    use crate::models::user::User;
    
    // 模拟用户数据
    let users = vec![
        User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password: "".to_string(), // 实际中不应该返回密码
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            is_active: true,
        },
        User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password: "".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            is_active: true,
        }
    ];
    
    let response = ApiResponse::page(users, 1, 10, 50);
    Ok(response.into_http_response())
}