use actix_web::{post, web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    api::response::{ApiError, ApiResponse, Response, UserResponse},
    models::auth::{LoginRequest, RegisterRequest, TokenResponse},
    services::AuthService,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<i32>,
}

/// 用户注册
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = UserResponse),
        (status = 400, description = "Invalid request", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    )
)]
#[post("/api/auth/register")]
pub async fn register(
    request: web::Json<RegisterRequest>,
    service: web::Data<AuthService>,
) -> Response<HttpResponse> {
    request.validate().map_err(ApiError::from)?;

    let user = service.register(request.into_inner()).await.map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = TokenResponse),
        (status = 400, description = "Invalid request", body = ResponseError),
        (status = 500, description = "Internal server error", body = ResponseError)
    )
)]
#[post("/api/auth/login")]
pub async fn login(
    request: web::Json<LoginRequest>,
    service: web::Data<AuthService>,
) -> Response<HttpResponse> {
    request.validate().map_err(ApiError::from)?;

    let token = service.login(request.into_inner()).await.map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(token)))
}
