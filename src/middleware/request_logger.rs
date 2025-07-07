use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

/// 请求和响应打印中间件
#[derive(Clone)]
pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        RequestLogger
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
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
            // 记录请求信息
            let method = req.method().clone();
            let path = req.path().to_string();
            let query = req.query_string().to_string();
            let headers = req.headers().clone();

            // 获取客户端IP
            let client_ip = req
                .connection_info()
                .peer_addr()
                .unwrap_or("unknown")
                .to_string();

            // 获取User-Agent
            let user_agent = headers
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown");

            // 打印请求信息
            tracing::info!(
                "📥 请求: {} {} {} - IP: {} - User-Agent: {}",
                method,
                path,
                if query.is_empty() {
                    String::new()
                } else {
                    format!("?{}", query)
                },
                client_ip,
                user_agent
            );

            // 打印请求头（仅在debug级别）
            if tracing::level_enabled!(tracing::Level::DEBUG) {
                tracing::debug!("📋 请求头:");
                for (name, value) in headers.iter() {
                    if let Ok(value_str) = value.to_str() {
                        // 隐藏敏感信息
                        let display_value =
                            if name.as_str().to_lowercase().contains("authorization") {
                                "Bearer ***"
                            } else {
                                value_str
                            };
                        tracing::debug!("  {}: {}", name, display_value);
                    }
                }
            }

            // 调用下一个服务
            let response = service.call(req).await;

            match response {
                Ok(res) => {
                    // 记录响应信息
                    let status = res.status();
                    let status_code = status.as_u16();

                    // 根据状态码使用不同的日志级别和emoji
                    match status_code {
                        200..=299 => {
                            tracing::info!(
                                "📤 响应: {} {} - 状态码: {} ✅",
                                method,
                                path,
                                status_code
                            );
                        }
                        300..=399 => {
                            tracing::info!(
                                "📤 响应: {} {} - 状态码: {} ↗️",
                                method,
                                path,
                                status_code
                            );
                        }
                        400..=499 => {
                            tracing::warn!(
                                "📤 响应: {} {} - 状态码: {} ⚠️",
                                method,
                                path,
                                status_code
                            );
                        }
                        500..=599 => {
                            tracing::error!(
                                "📤 响应: {} {} - 状态码: {} ❌",
                                method,
                                path,
                                status_code
                            );
                        }
                        _ => {
                            tracing::info!(
                                "📤 响应: {} {} - 状态码: {} ❓",
                                method,
                                path,
                                status_code
                            );
                        }
                    }

                    // 打印响应头（仅在debug级别）
                    if tracing::level_enabled!(tracing::Level::DEBUG) {
                        tracing::debug!("📋 响应头:");
                        for (name, value) in res.headers().iter() {
                            if let Ok(value_str) = value.to_str() {
                                tracing::debug!("  {}: {}", name, value_str);
                            }
                        }
                    }

                    Ok(res.map_body(|_, body| EitherBody::left(body)))
                }
                Err(e) => {
                    // 记录错误响应
                    tracing::error!("📤 响应: {} {} - 错误: {} ❌", method, path, e);
                    Err(e)
                }
            }
        })
    }
}
