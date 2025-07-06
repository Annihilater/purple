use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{api::response::UserResponse, models::user::CreateUser, repositories::UserRepository};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub invite_user_id: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "创建用户成功", body = UserResponse),
        (status = 400, description = "创建用户失败", body = ErrorResponse),
    )
)]
#[post("/users")]
pub async fn create_user(
    user_repo: web::Data<UserRepository>,
    user: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let user = CreateUser {
        email: user.email.clone(),
        password: user.password.clone(),
        invite_user_id: user.invite_user_id,
        uuid: Uuid::new_v4().to_string(),
        token: Uuid::new_v4().to_string(),
    };

    match user_repo.create(user).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            code: 200,
            message: "success".to_string(),
            data: Some(user),
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            code: 400,
            message: e.to_string(),
        }),
    }
}

/// 获取用户信息
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID"),
    ),
    responses(
        (status = 200, description = "获取用户成功", body = UserResponse),
        (status = 404, description = "用户不存在", body = ErrorResponse),
        (status = 500, description = "服务器内部错误", body = ErrorResponse),
    )
)]
#[get("/users/{id}")]
pub async fn get_user(user_repo: web::Data<UserRepository>, id: web::Path<i32>) -> HttpResponse {
    match user_repo.find_by_id(*id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponse {
            code: 200,
            message: "success".to_string(),
            data: Some(user),
        }),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            code: 404,
            message: format!("User {} not found", id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            code: 500,
            message: e.to_string(),
        }),
    }
}
