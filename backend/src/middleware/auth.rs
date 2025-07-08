use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    common::{
        response_v2::{ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::auth::Claims,
    repositories::UserRepository,
};

/// 认证中间件
#[derive(Clone)]
pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // 获取 Authorization header
            let auth_header = req
                .headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "));

            let token = match auth_header {
                Some(token) => token,
                None => {
                    tracing::warn!("Missing authorization header");
                    let response = ApiResponse::error_with_details(
                        ErrorCode::Unauthorized,
                        Some("缺少授权令牌".to_string()),
                        None,
                    );
                    return Ok(ServiceResponse::new(
                        req.into_parts().0,
                        response.into_http_response(),
                    )
                    .map_body(|_, body| EitherBody::right(body)));
                }
            };

            // 验证 token
            let claims = match Claims::decode(token) {
                Ok(claims) => claims,
                Err(e) => {
                    tracing::warn!("Invalid token: {}", e);
                    let response = ApiResponse::error_with_details(
                        ErrorCode::InvalidToken,
                        Some("无效的授权令牌".to_string()),
                        None,
                    );
                    return Ok(ServiceResponse::new(
                        req.into_parts().0,
                        response.into_http_response(),
                    )
                    .map_body(|_, body| EitherBody::right(body)));
                }
            };

            // 检查token是否过期
            let now = chrono::Utc::now().timestamp();
            if claims.exp < now {
                tracing::warn!("Token expired for user: {}", claims.sub);
                let response = ApiResponse::error_with_details(
                    ErrorCode::TokenExpired,
                    Some("授权令牌已过期".to_string()),
                    None,
                );
                return Ok(
                    ServiceResponse::new(req.into_parts().0, response.into_http_response())
                        .map_body(|_, body| EitherBody::right(body)),
                );
            }

            // 检查用户是否存在
            let user_repository = match req.app_data::<web::Data<UserRepository>>() {
                Some(repo) => repo,
                None => {
                    tracing::error!("UserRepository not found in app data");
                    let response = ApiResponse::error_with_details(
                        ErrorCode::InternalError,
                        Some("服务器配置错误".to_string()),
                        None,
                    );
                    return Ok(ServiceResponse::new(
                        req.into_parts().0,
                        response.into_http_response(),
                    )
                    .map_body(|_, body| EitherBody::right(body)));
                }
            };

            match user_repository.get_ref().find_by_id(claims.sub).await {
                Ok(Some(user)) => {
                    // 检查用户是否被禁用
                    if user.banned.unwrap_or(false) {
                        tracing::warn!("Banned user {} attempted to access", claims.sub);
                        let response = ApiResponse::error_with_details(
                            ErrorCode::UserDisabled,
                            Some("账户已被禁用".to_string()),
                            None,
                        );
                        return Ok(ServiceResponse::new(
                            req.into_parts().0,
                            response.into_http_response(),
                        )
                        .map_body(|_, body| EitherBody::right(body)));
                    }

                    // 将用户ID添加到请求扩展中，供后续处理器使用
                    req.extensions_mut().insert(claims.sub);

                    // 可选：将完整的用户信息也添加到扩展中
                    req.extensions_mut().insert(user);

                    let res = service.call(req).await?;
                    Ok(res.map_body(|_, body| EitherBody::left(body)))
                }
                Ok(None) => {
                    tracing::warn!("User {} not found", claims.sub);
                    let response = ApiResponse::error_with_details(
                        ErrorCode::UserNotFound,
                        Some("用户不存在".to_string()),
                        None,
                    );
                    Ok(
                        ServiceResponse::new(req.into_parts().0, response.into_http_response())
                            .map_body(|_, body| EitherBody::right(body)),
                    )
                }
                Err(e) => {
                    tracing::error!("Failed to verify user {}: {}", claims.sub, e);
                    let response = ApiResponse::error_with_details(
                        ErrorCode::DatabaseError,
                        Some("验证用户失败".to_string()),
                        None,
                    );
                    Ok(
                        ServiceResponse::new(req.into_parts().0, response.into_http_response())
                            .map_body(|_, body| EitherBody::right(body)),
                    )
                }
            }
        })
    }
}

/// 用于从请求中提取当前用户ID的辅助函数
pub fn get_current_user_id(req: &ServiceRequest) -> Option<i32> {
    req.extensions().get::<i32>().copied()
}

/// 用于从请求中提取当前用户信息的辅助函数  
pub fn get_current_user(req: &ServiceRequest) -> Option<crate::models::user::User> {
    req.extensions().get::<crate::models::user::User>().cloned()
}
