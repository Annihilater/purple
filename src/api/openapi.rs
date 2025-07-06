use utoipa::OpenApi;

use crate::api::{
    health::{health_check, HealthResponse},
    response::ApiResponse,
    user::{create_user, get_user, CreateUserRequest},
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
            ApiResponse<User>,
            CreateUserRequest,
            User,
            HealthResponse,
        )
    ),
    tags(
        (name = "health", description = "健康检查接口"),
        (name = "users", description = "用户管理接口")
    )
)]
pub struct ApiDoc;
