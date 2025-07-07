use actix_web::{post, web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    common::{EmptyApiResponse, ErrorCode, ResponseBuilder, TokenApiResponse, UserIdApiResponse},
    models::auth::{LoginRequest, RegisterRequest, TokenResponse},
    services::AuthService,
};

/// 用户注册
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "用户注册成功", body = UserIdApiResponse),
        (status = 400, description = "请求参数无效", body = EmptyApiResponse),
        (status = 409, description = "用户已存在", body = EmptyApiResponse),
        (status = 500, description = "内部服务器错误", body = EmptyApiResponse)
    )
)]
#[post("/api/auth/register")]
pub async fn register(
    request: web::Json<RegisterRequest>,
    service: web::Data<AuthService>,
) -> HttpResponse {
    if let Err(validation_errors) = request.validate() {
        let error_msg = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_msgs = errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .unwrap_or(&std::borrow::Cow::Borrowed("validation error"))
                            .to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", field, error_msgs)
            })
            .collect::<Vec<_>>()
            .join("; ");

        return ResponseBuilder::error_with_message(ErrorCode::ValidationError, error_msg);
    }

    match service.register(request.into_inner()).await {
        Ok(user_id) => ResponseBuilder::success_with_message(user_id, "用户注册成功".to_string()),
        Err(e) => {
            let error_msg = e.to_string();
            tracing::error!("用户注册失败: {}", error_msg);

            if error_msg.contains("already exists") || error_msg.contains("duplicate") {
                ResponseBuilder::error_with_message(
                    ErrorCode::UserAlreadyExists,
                    "用户已存在".to_string(),
                )
            } else if error_msg.contains("email") && error_msg.contains("invalid") {
                ResponseBuilder::error_with_message(
                    ErrorCode::InvalidEmail,
                    "邮箱格式无效".to_string(),
                )
            } else if error_msg.contains("password") {
                ResponseBuilder::error_with_message(
                    ErrorCode::InvalidPassword,
                    "密码格式无效".to_string(),
                )
            } else {
                ResponseBuilder::error_with_message(
                    ErrorCode::InternalError,
                    "注册失败，请稍后重试".to_string(),
                )
            }
        }
    }
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "用户登录成功", body = TokenApiResponse),
        (status = 400, description = "请求参数无效", body = EmptyApiResponse),
        (status = 401, description = "用户名或密码错误", body = EmptyApiResponse),
        (status = 403, description = "账户已被禁用", body = EmptyApiResponse),
        (status = 500, description = "内部服务器错误", body = EmptyApiResponse)
    )
)]
#[post("/api/auth/login")]
pub async fn login(
    request: web::Json<LoginRequest>,
    service: web::Data<AuthService>,
) -> HttpResponse {
    if let Err(validation_errors) = request.validate() {
        let error_msg = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_msgs = errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .unwrap_or(&std::borrow::Cow::Borrowed("validation error"))
                            .to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", field, error_msgs)
            })
            .collect::<Vec<_>>()
            .join("; ");

        return ResponseBuilder::error_with_message(ErrorCode::ValidationError, error_msg);
    }

    match service.login(request.into_inner()).await {
        Ok(token) => ResponseBuilder::success_with_message(token, "登录成功".to_string()),
        Err(e) => {
            let error_msg = e.to_string();
            tracing::error!("用户登录失败: {}", error_msg);

            if error_msg.contains("not found") || error_msg.contains("用户不存在") {
                ResponseBuilder::error_with_message(
                    ErrorCode::UserNotFound,
                    "用户不存在".to_string(),
                )
            } else if error_msg.contains("password") || error_msg.contains("密码") {
                ResponseBuilder::error_with_message(
                    ErrorCode::InvalidCredentials,
                    "用户名或密码错误".to_string(),
                )
            } else if error_msg.contains("disabled")
                || error_msg.contains("banned")
                || error_msg.contains("禁用")
            {
                ResponseBuilder::error_with_message(
                    ErrorCode::UserDisabled,
                    "账户已被禁用".to_string(),
                )
            } else if error_msg.contains("email") && error_msg.contains("invalid") {
                ResponseBuilder::error_with_message(
                    ErrorCode::InvalidEmail,
                    "邮箱格式无效".to_string(),
                )
            } else {
                ResponseBuilder::error_with_message(
                    ErrorCode::InternalError,
                    "登录失败，请稍后重试".to_string(),
                )
            }
        }
    }
}
