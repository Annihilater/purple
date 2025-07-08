use std::time::Instant;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

/// 请求日志中间件
#[derive(Clone)]
pub struct RequestLogging;

impl RequestLogging {
    pub fn new() -> Self {
        RequestLogging
    }
}

impl Default for RequestLogging {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggingMiddleware { service }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query = req.query_string().to_string();
        let remote_addr = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        // 获取用户ID（如果有的话）
        let user_id = req.extensions().get::<i32>().copied();

        let service = self.service.call(req);

        Box::pin(async move {
            let response = service.await?;
            let duration = start_time.elapsed();
            let status = response.status().as_u16();

            // 记录请求日志
            if status >= 400 {
                tracing::warn!(
                    method = %method,
                    path = %path,
                    query = %query,
                    status = %status,
                    duration_ms = %duration.as_millis(),
                    remote_addr = %remote_addr,
                    user_agent = %user_agent,
                    user_id = ?user_id,
                    "HTTP request completed with error"
                );
            } else {
                tracing::info!(
                    method = %method,
                    path = %path,
                    query = %query,
                    status = %status,
                    duration_ms = %duration.as_millis(),
                    remote_addr = %remote_addr,
                    user_agent = %user_agent,
                    user_id = ?user_id,
                    "HTTP request completed"
                );
            }

            Ok(response)
        })
    }
}

/// 从ServiceRequest中提取用户ID的辅助函数
pub fn extract_user_id(req: &ServiceRequest) -> Option<i32> {
    req.extensions().get::<i32>().copied()
}

/// 从ServiceRequest中提取客户端IP的辅助函数
pub fn extract_client_ip(req: &ServiceRequest) -> String {
    // 按优先级检查不同的IP来源
    req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
        })
        .or_else(|| req.connection_info().peer_addr().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}
