use utoipa::OpenApi;

use crate::api::{
    health::HealthResponse,
    response::UserResponse,
    user::{CreateUserRequest, ErrorResponse},
};
use crate::models::user::User;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::health::health_check,
        crate::api::user::create_user,
        crate::api::user::get_user,
    ),
    components(
        schemas(
            UserResponse,
            CreateUserRequest,
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
