use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::response::ApiResponse,
    models::user::{CreateUser, User},
    repositories::UserRepository,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub invite_code: Option<String>,
}

/// 创建新用户
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "创建用户成功", body = ApiResponse<User>),
        (status = 400, description = "创建用户失败", body = ApiResponse<String>)
    ),
    tag = "users"
)]
#[post("/users")]
pub async fn create_user(
    repo: web::Data<UserRepository>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    // 生成UUID和Token
    let uuid = uuid::Uuid::new_v4().to_string();
    let token = uuid::Uuid::new_v4().to_string().replace("-", "");

    let user = CreateUser {
        email: req.email.clone(),
        password: req.password.clone(), // 注意：实际使用时需要对密码进行哈希处理
        invite_user_id: None, // 这里需要根据invite_code查询邀请人ID
        uuid,
        token,
    };

    match repo.create(user).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "创建用户失败".to_string(),
        )),
    }
}

/// 获取用户信息
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "获取用户成功", body = ApiResponse<User>),
        (status = 404, description = "用户不存在", body = ApiResponse<String>)
    ),
    tag = "users"
)]
#[get("/users/{id}")]
pub async fn get_user(
    repo: web::Data<UserRepository>,
    id: web::Path<i32>,
) -> HttpResponse {
    match repo.find_by_id(id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "用户不存在".to_string(),
        )),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "获取用户信息失败".to_string(),
        )),
    }
} 