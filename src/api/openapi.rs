use utoipa::OpenApi;

use crate::api::{
    health::HealthResponse,
    response::UserResponse,
    user::{
        CreateUserRequest, ErrorResponse, GetUsersQuery, UpdateUserRequest,
        UpdateUserStatusRequest, UsersListResponse,
    },
};
use crate::models::user::User;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::health::health_check,
        crate::api::user::create_user,
        crate::api::user::get_users,
        crate::api::user::get_user,
        crate::api::user::update_user,
        crate::api::user::delete_user,
        crate::api::user::update_user_status,
    ),
    components(
        schemas(
            UserResponse,
            UsersListResponse,
            CreateUserRequest,
            UpdateUserRequest,
            UpdateUserStatusRequest,
            GetUsersQuery,
            User,
            HealthResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "health", description = "健康检查接口"),
        (name = "users", description = "用户管理接口")
    )
)]
pub struct ApiDoc;
