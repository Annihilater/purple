use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    common::{ErrorCode, PageResponse, ResponseBuilder},
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
        (status = 200, description = "创建用户成功", body = crate::common::ApiResponse<User>),
        (status = 400, description = "创建用户失败", body = crate::common::ApiResponse<()>),
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
        Ok(user) => ResponseBuilder::success_with_message(user, "用户创建成功".to_string()),
        Err(e) => {
            tracing::error!("创建用户失败: {}", e);
            // 根据错误类型选择适当的错误代码
            if e.to_string().contains("duplicate") || e.to_string().contains("already exists") {
                ResponseBuilder::error_with_message(
                    ErrorCode::UserAlreadyExists,
                    "用户已存在".to_string(),
                )
            } else {
                ResponseBuilder::error_with_message(
                    ErrorCode::DatabaseError,
                    "数据库操作失败".to_string(),
                )
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
        (status = 200, description = "获取用户列表成功", body = crate::common::ApiResponse<PageResponse<User>>),
        (status = 500, description = "服务器内部错误", body = crate::common::ApiResponse<()>),
    )
)]
#[get("/users")]
pub async fn get_users(
    user_repo: web::Data<UserRepository>,
    query: web::Query<GetUsersQuery>,
) -> HttpResponse {
    match user_repo
        .find_all(query.page as i32, query.page_size as i32)
        .await
    {
        Ok((users, total)) => {
            let page_response = PageResponse::new(users, total as u64, query.page, query.page_size);
            ResponseBuilder::success_with_message(page_response, "获取用户列表成功".to_string())
        }
        Err(e) => {
            tracing::error!("获取用户列表失败: {}", e);
            ResponseBuilder::error_with_message(
                ErrorCode::DatabaseError,
                "获取用户列表失败".to_string(),
            )
        }
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
        (status = 200, description = "获取用户成功", body = crate::common::ApiResponse<User>),
        (status = 404, description = "用户不存在", body = crate::common::ApiResponse<()>),
        (status = 500, description = "服务器内部错误", body = crate::common::ApiResponse<()>),
    )
)]
#[get("/users/{id}")]
pub async fn get_user(user_repo: web::Data<UserRepository>, id: web::Path<i32>) -> HttpResponse {
    match user_repo.find_by_id(*id).await {
        Ok(Some(user)) => ResponseBuilder::success_with_message(user, "获取用户成功".to_string()),
        Ok(None) => {
            ResponseBuilder::error_with_message(ErrorCode::UserNotFound, "用户不存在".to_string())
        }
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            ResponseBuilder::error_with_message(
                ErrorCode::DatabaseError,
                "查询用户失败".to_string(),
            )
        }
    }
}

/// 更新用户信息
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID"),
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "更新用户成功", body = crate::common::ApiResponse<User>),
        (status = 404, description = "用户不存在", body = crate::common::ApiResponse<()>),
        (status = 500, description = "服务器内部错误", body = crate::common::ApiResponse<()>),
    )
)]
#[put("/users/{id}")]
pub async fn update_user(
    user_repo: web::Data<UserRepository>,
    id: web::Path<i32>,
    update: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    match user_repo.find_by_id(*id).await {
        Ok(Some(mut user)) => {
            if let Some(email) = update.email.clone() {
                user.email = email;
            }
            if let Some(password) = update.password.clone() {
                user.password = password;
            }
            if let Some(remarks) = update.remarks.clone() {
                user.remarks = Some(remarks);
            }
            if let Some(group_id) = update.group_id {
                user.group_id = Some(group_id);
            }
            if let Some(plan_id) = update.plan_id {
                user.plan_id = Some(plan_id);
            }
            if let Some(speed_limit) = update.speed_limit {
                user.speed_limit = Some(speed_limit);
            }

            match user_repo.update(&user).await {
                Ok(user) => ResponseBuilder::success_with_message(user, "更新用户成功".to_string()),
                Err(e) => {
                    tracing::error!("更新用户失败: {}", e);
                    ResponseBuilder::error_with_message(
                        ErrorCode::DatabaseError,
                        "更新用户失败".to_string(),
                    )
                }
            }
        }
        Ok(None) => {
            ResponseBuilder::error_with_message(ErrorCode::UserNotFound, "用户不存在".to_string())
        }
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            ResponseBuilder::error_with_message(
                ErrorCode::DatabaseError,
                "查询用户失败".to_string(),
            )
        }
    }
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID"),
    ),
    responses(
        (status = 200, description = "删除用户成功", body = crate::common::ApiResponse<()>),
        (status = 404, description = "用户不存在", body = crate::common::ApiResponse<()>),
        (status = 500, description = "服务器内部错误", body = crate::common::ApiResponse<()>),
    )
)]
#[delete("/users/{id}")]
pub async fn delete_user(user_repo: web::Data<UserRepository>, id: web::Path<i32>) -> HttpResponse {
    match user_repo.find_by_id(*id).await {
        Ok(Some(_)) => match user_repo.delete(*id).await {
            Ok(_) => ResponseBuilder::success_with_message((), "删除用户成功".to_string()),
            Err(e) => {
                tracing::error!("删除用户失败: {}", e);
                ResponseBuilder::error_with_message(
                    ErrorCode::DatabaseError,
                    "删除用户失败".to_string(),
                )
            }
        },
        Ok(None) => {
            ResponseBuilder::error_with_message(ErrorCode::UserNotFound, "用户不存在".to_string())
        }
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            ResponseBuilder::error_with_message(
                ErrorCode::DatabaseError,
                "查询用户失败".to_string(),
            )
        }
    }
}

/// 更新用户状态
#[utoipa::path(
    patch,
    path = "/users/{id}/status",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID"),
    ),
    request_body = UpdateUserStatusRequest,
    responses(
        (status = 200, description = "更新用户状态成功", body = crate::common::ApiResponse<User>),
        (status = 404, description = "用户不存在", body = crate::common::ApiResponse<()>),
        (status = 500, description = "服务器内部错误", body = crate::common::ApiResponse<()>),
    )
)]
#[patch("/users/{id}/status")]
pub async fn update_user_status(
    user_repo: web::Data<UserRepository>,
    id: web::Path<i32>,
    status: web::Json<UpdateUserStatusRequest>,
) -> HttpResponse {
    match user_repo.find_by_id(*id).await {
        Ok(Some(mut user)) => {
            user.banned = Some(status.banned);

            match user_repo.update(&user).await {
                Ok(user) => {
                    ResponseBuilder::success_with_message(user, "更新用户状态成功".to_string())
                }
                Err(e) => {
                    tracing::error!("更新用户状态失败: {}", e);
                    ResponseBuilder::error_with_message(
                        ErrorCode::DatabaseError,
                        "更新用户状态失败".to_string(),
                    )
                }
            }
        }
        Ok(None) => {
            ResponseBuilder::error_with_message(ErrorCode::UserNotFound, "用户不存在".to_string())
        }
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            ResponseBuilder::error_with_message(
                ErrorCode::DatabaseError,
                "查询用户失败".to_string(),
            )
        }
    }
}
