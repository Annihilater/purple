use actix_web::{post, web, HttpResponse, Result};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::auth::{LoginRequest, RegisterRequest},
    services::AuthService,
};

/// 用户注册
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    operation_id = "register_user",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "注册成功", body = inline(crate::common::response_v2::ApiSuccessResponse<i32>)),
        (status = 400, description = "请求参数无效", body = crate::common::response_v2::ApiErrorResponse),
        (status = 409, description = "用户已存在", body = crate::common::response_v2::ApiErrorResponse),
        (status = 500, description = "内部服务器错误", body = crate::common::response_v2::ApiErrorResponse)
    )
)]
#[post("/api/auth/register")]
pub async fn register(
    request: web::Json<RegisterRequest>,
    service: web::Data<AuthService>,
) -> Result<HttpResponse, ApiError> {
    // 验证请求参数
    if let Err(validation_errors) = request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    match service.register(request.into_inner()).await {
        Ok(user_id) => {
            let response = ApiResponse::success(user_id);
            Ok(response.into_http_response())
        }
        Err(e) => {
            let error_msg = e.to_string();
            tracing::error!("用户注册失败: {}", error_msg);

            if error_msg.contains("already exists") || error_msg.contains("duplicate") {
                Err(ApiError::with_details(
                    ErrorCode::UserAlreadyExists,
                    "用户已存在".to_string(),
                ))
            } else if error_msg.contains("email") && error_msg.contains("invalid") {
                Err(ApiError::with_details(
                    ErrorCode::InvalidEmail,
                    "邮箱格式无效".to_string(),
                ))
            } else if error_msg.contains("password") {
                Err(ApiError::with_details(
                    ErrorCode::InvalidPassword,
                    "密码格式无效".to_string(),
                ))
            } else {
                Err(ApiError::with_details(
                    ErrorCode::InternalError,
                    "注册失败，请稍后重试".to_string(),
                ))
            }
        }
    }
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    operation_id = "login_user",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = inline(crate::common::response_v2::ApiSuccessResponse<purple_shared::LoginResponse>)),
        (status = 400, description = "请求参数无效", body = crate::common::response_v2::ApiErrorResponse),
        (status = 401, description = "用户名或密码错误", body = crate::common::response_v2::ApiErrorResponse),
        (status = 403, description = "账户已被禁用", body = crate::common::response_v2::ApiErrorResponse),
        (status = 500, description = "内部服务器错误", body = crate::common::response_v2::ApiErrorResponse)
    )
)]
#[post("/api/auth/login")]
pub async fn login(
    request: web::Json<LoginRequest>,
    service: web::Data<AuthService>,
) -> Result<HttpResponse, ApiError> {
    // 验证请求参数
    if let Err(validation_errors) = request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    match service.login(request.into_inner()).await {
        Ok(login_response) => {
            let response = ApiResponse::success(login_response);
            Ok(response.into_http_response())
        }
        Err(e) => {
            let error_msg = e.to_string();
            tracing::error!("用户登录失败: {}", error_msg);

            if error_msg.contains("not found") || error_msg.contains("用户不存在") {
                Err(ApiError::with_details(
                    ErrorCode::UserNotFound,
                    "用户不存在".to_string(),
                ))
            } else if error_msg.contains("password") || error_msg.contains("密码") {
                Err(ApiError::with_details(
                    ErrorCode::InvalidCredentials,
                    "用户名或密码错误".to_string(),
                ))
            } else if error_msg.contains("disabled")
                || error_msg.contains("banned")
                || error_msg.contains("禁用")
            {
                Err(ApiError::with_details(
                    ErrorCode::UserDisabled,
                    "账户已被禁用".to_string(),
                ))
            } else if error_msg.contains("email") && error_msg.contains("invalid") {
                Err(ApiError::with_details(
                    ErrorCode::InvalidEmail,
                    "邮箱格式无效".to_string(),
                ))
            } else {
                Err(ApiError::with_details(
                    ErrorCode::InternalError,
                    "登录失败，请稍后重试".to_string(),
                ))
            }
        }
    }
}
