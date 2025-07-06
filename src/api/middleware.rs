use std::future::{ready, Ready};

use actix_http::Extensions;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::{self, HeaderValue},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::auth::Claims;
use crate::repositories::UserRepository;

#[derive(Clone)]
pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // 获取 Authorization header
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "));

            let auth_header = match auth_header {
                Some(token) => token,
                None => {
                    return Err(ErrorUnauthorized("Missing authorization header"));
                }
            };

            // 验证 token
            let user_repository = req.app_data::<web::Data<UserRepository>>().unwrap();
            let claims = match Claims::decode(auth_header) {
                Ok(claims) => claims,
                Err(_) => {
                    return Err(ErrorUnauthorized("Invalid token"));
                }
            };

            // 检查用户是否存在
            match user_repository.get_ref().find_by_id(claims.sub).await {
                Ok(Some(_)) => {
                    // 将用户ID添加到请求扩展中
                    req.extensions_mut().insert(claims.sub);
                    service.call(req).await
                }
                Ok(None) => Err(ErrorUnauthorized("User not found")),
                Err(_) => Err(ErrorUnauthorized("Failed to verify user")),
            }
        })
    }
}
