use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    api::response::UserResponse,
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

#[derive(Debug, Serialize, ToSchema)]
pub struct UsersListResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<Vec<User>>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
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

/// 获取用户列表
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(
        GetUsersQuery
    ),
    responses(
        (status = 200, description = "获取用户列表成功", body = UsersListResponse),
        (status = 500, description = "服务器内部错误", body = ErrorResponse),
    )
)]
#[get("/users")]
pub async fn get_users(
    user_repo: web::Data<UserRepository>,
    query: web::Query<GetUsersQuery>,
) -> HttpResponse {
    match user_repo.find_all(query.page, query.page_size).await {
        Ok((users, total)) => HttpResponse::Ok().json(UsersListResponse {
            code: 200,
            message: "success".to_string(),
            data: Some(users),
            total,
            page: query.page,
            page_size: query.page_size,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            code: 500,
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
        (status = 200, description = "更新用户成功", body = UserResponse),
        (status = 404, description = "用户不存在", body = ErrorResponse),
        (status = 500, description = "服务器内部错误", body = ErrorResponse),
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
                Ok(updated_user) => HttpResponse::Ok().json(UserResponse {
                    code: 200,
                    message: "success".to_string(),
                    data: Some(updated_user),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    message: e.to_string(),
                }),
            }
        }
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

/// 删除用户
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "用户ID"),
    ),
    responses(
        (status = 200, description = "删除用户成功", body = UserResponse),
        (status = 404, description = "用户不存在", body = ErrorResponse),
        (status = 500, description = "服务器内部错误", body = ErrorResponse),
    )
)]
#[delete("/users/{id}")]
pub async fn delete_user(user_repo: web::Data<UserRepository>, id: web::Path<i32>) -> HttpResponse {
    match user_repo.delete(*id).await {
        Ok(true) => HttpResponse::Ok().json(UserResponse {
            code: 200,
            message: "success".to_string(),
            data: None,
        }),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            code: 404,
            message: format!("User {} not found", id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            code: 500,
            message: e.to_string(),
        }),
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
        (status = 200, description = "更新用户状态成功", body = UserResponse),
        (status = 404, description = "用户不存在", body = ErrorResponse),
        (status = 500, description = "服务器内部错误", body = ErrorResponse),
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
                Ok(updated_user) => HttpResponse::Ok().json(UserResponse {
                    code: 200,
                    message: "success".to_string(),
                    data: Some(updated_user),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    message: e.to_string(),
                }),
            }
        }
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
