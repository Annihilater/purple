use actix_web::{delete, get, patch, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::user::{CreateUser, User},
    repositories::UserRepository,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub invite_user_id: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub remarks: Option<String>,
    pub group_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub speed_limit: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserStatusRequest {
    pub banned: bool,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetUsersQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "创建用户成功"),
        (status = 400, description = "创建用户失败"),
    )
)]
#[post("/users")]
pub async fn create_user(
    user_repo: web::Data<UserRepository>,
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = CreateUser {
        email: user.email.clone(),
        password: user.password.clone(),
        invite_user_id: user.invite_user_id,
        uuid: Uuid::new_v4().to_string(),
        token: Uuid::new_v4().to_string(),
    };

    match user_repo.create(user).await {
        Ok(user) => {
            let response = ApiResponse::success(user);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建用户失败: {}", e);
            // 根据错误类型选择适当的错误代码
            if e.to_string().contains("duplicate") || e.to_string().contains("already exists") {
                Err(ApiError::with_details(
                    ErrorCode::UserAlreadyExists,
                    "用户已存在".to_string(),
                ))
            } else {
                Err(ApiError::with_details(
                    ErrorCode::DatabaseError,
                    "数据库操作失败".to_string(),
                ))
            }
        }
    }
}

/// 获取用户列表
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(
        GetUsersQuery
    ),
    responses(
        (status = 200, description = "获取用户列表成功"),
        (status = 500, description = "服务器内部错误"),
    )
)]
#[get("/users")]
pub async fn get_users(
    user_repo: web::Data<UserRepository>,
    query: web::Query<GetUsersQuery>,
) -> Result<HttpResponse, ApiError> {
    match user_repo
        .find_all(query.page as i32, query.page_size as i32)
        .await
    {
        Ok((users, total)) => {
            let response = ApiResponse::page(users, query.page, query.page_size, total as u64);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取用户列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 根据ID获取用户
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "获取用户成功"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器内部错误"),
    )
)]
#[get("/users/{id}")]
pub async fn get_user(
    user_repo: web::Data<UserRepository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    match user_repo.find_by_id(user_id).await {
        Ok(Some(user)) => {
            let response = ApiResponse::success(user);
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::UserNotFound)),
        Err(e) => {
            tracing::error!("获取用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 更新用户信息
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "更新用户成功"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器内部错误"),
    )
)]
#[put("/users/{id}")]
pub async fn update_user(
    user_repo: web::Data<UserRepository>,
    path: web::Path<i32>,
    request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    // 首先获取现有用户
    match user_repo.find_by_id(user_id).await {
        Ok(Some(mut user)) => {
            // 更新用户字段
            if let Some(email) = &request.email {
                user.email = email.clone();
            }
            if let Some(password) = &request.password {
                user.password = password.clone();
            }
            // 注意：根据实际的 User 模型调整这些字段
            // 这里假设用户模型有这些字段，如果没有可以忽略或调整

            // 更新用户
            match user_repo.update(&user).await {
                Ok(updated_user) => {
                    let response = ApiResponse::success(updated_user);
                    Ok(response.into_http_response())
                }
                Err(e) => {
                    tracing::error!("更新用户失败: {}", e);
                    Err(ApiError::with_details(
                        ErrorCode::DatabaseError,
                        "数据库操作失败".to_string(),
                    ))
                }
            }
        }
        Ok(None) => Err(ApiError::new(ErrorCode::UserNotFound)),
        Err(e) => {
            tracing::error!("查找用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "删除用户成功"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器内部错误"),
    )
)]
#[delete("/users/{id}")]
pub async fn delete_user(
    user_repo: web::Data<UserRepository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    match user_repo.delete(user_id).await {
        Ok(true) => {
            let response = ApiResponse::success(());
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::new(ErrorCode::UserNotFound)),
        Err(e) => {
            tracing::error!("删除用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}

/// 更新用户状态
#[utoipa::path(
    patch,
    path = "/users/{id}/status",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID")
    ),
    request_body = UpdateUserStatusRequest,
    responses(
        (status = 200, description = "更新用户状态成功"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器内部错误"),
    )
)]
#[patch("/users/{id}/status")]
pub async fn update_user_status(
    user_repo: web::Data<UserRepository>,
    path: web::Path<i32>,
    request: web::Json<UpdateUserStatusRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();

    // 首先获取现有用户
    match user_repo.find_by_id(user_id).await {
        Ok(Some(mut user)) => {
            // 更新用户状态
            user.banned = Some(request.banned);

            // 保存更新后的用户
            match user_repo.update(&user).await {
                Ok(updated_user) => {
                    let response = ApiResponse::success(updated_user);
                    Ok(response.into_http_response())
                }
                Err(e) => {
                    tracing::error!("更新用户状态失败: {}", e);
                    Err(ApiError::with_details(
                        ErrorCode::DatabaseError,
                        "数据库操作失败".to_string(),
                    ))
                }
            }
        }
        Ok(None) => Err(ApiError::new(ErrorCode::UserNotFound)),
        Err(e) => {
            tracing::error!("查找用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}
