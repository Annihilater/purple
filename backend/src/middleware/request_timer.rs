use std::{
    future::{ready, Ready},
    rc::Rc,
    time::Instant,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

/// 请求耗时记录中间件
#[derive(Clone)]
pub struct RequestTimer;

impl RequestTimer {
    pub fn new() -> Self {
        RequestTimer
    }
}

impl Default for RequestTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestTimer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestTimerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestTimerMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestTimerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestTimerMiddleware<S>
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
            // 记录请求开始时间
            let start_time = Instant::now();

            // 获取请求信息用于日志
            let method = req.method().clone();
            let path = req.path().to_string();
            let query = req.query_string().to_string();

            // 获取客户端IP
            let client_ip = req
                .connection_info()
                .peer_addr()
                .unwrap_or("unknown")
                .to_string();

            // 调用下一个服务
            let response = service.call(req).await;

            // 计算耗时
            let duration = start_time.elapsed();
            let duration_ms = duration.as_millis();
            let duration_us = duration.as_micros();

            match response {
                Ok(res) => {
                    let status_code = res.status().as_u16();

                    // 根据耗时使用不同的日志级别和emoji
                    let (log_level, emoji) = match duration_ms {
                        0..=100 => ("info", "🚀"),     // 快速响应
                        101..=500 => ("info", "⚡"),   // 正常响应
                        501..=1000 => ("warn", "🐢"),  // 较慢响应
                        1001..=5000 => ("warn", "🦕"), // 慢响应
                        _ => ("error", "🐌"),          // 非常慢的响应
                    };

                    // 格式化耗时显示
                    let duration_str = if duration_ms < 1 {
                        format!("{}μs", duration_us)
                    } else if duration_ms < 1000 {
                        format!("{}ms", duration_ms)
                    } else {
                        format!("{:.2}s", duration.as_secs_f64())
                    };

                    let full_path = if query.is_empty() {
                        path.clone()
                    } else {
                        format!("{}?{}", path, query)
                    };

                    // 根据日志级别输出
                    match log_level {
                        "info" => {
                            tracing::info!(
                                "⏱️ {} {} {} - {} - IP: {} - 耗时: {} {}",
                                method,
                                full_path,
                                status_code,
                                if (200..300).contains(&status_code) {
                                    "✅"
                                } else {
                                    "⚠️"
                                },
                                client_ip,
                                duration_str,
                                emoji
                            );
                        }
                        "warn" => {
                            tracing::warn!(
                                "⏱️ {} {} {} - {} - IP: {} - 耗时: {} {} (响应较慢)",
                                method,
                                full_path,
                                status_code,
                                if (200..300).contains(&status_code) {
                                    "✅"
                                } else {
                                    "⚠️"
                                },
                                client_ip,
                                duration_str,
                                emoji
                            );
                        }
                        "error" => {
                            tracing::error!(
                                "⏱️ {} {} {} - {} - IP: {} - 耗时: {} {} (响应过慢)",
                                method,
                                full_path,
                                status_code,
                                if (200..300).contains(&status_code) {
                                    "✅"
                                } else {
                                    "⚠️"
                                },
                                client_ip,
                                duration_str,
                                emoji
                            );
                        }
                        _ => {}
                    }

                    // 在debug级别记录更详细的性能信息
                    if tracing::level_enabled!(tracing::Level::DEBUG) {
                        tracing::debug!(
                            "🔍 性能详情: {} {} - 精确耗时: {}μs ({}ms) - 状态: {}",
                            method,
                            path,
                            duration_us,
                            duration_ms,
                            status_code
                        );
                    }

                    Ok(res.map_body(|_, body| EitherBody::left(body)))
                }
                Err(e) => {
                    // 记录错误请求的耗时
                    let duration_str = if duration_ms < 1 {
                        format!("{}μs", duration_us)
                    } else if duration_ms < 1000 {
                        format!("{}ms", duration_ms)
                    } else {
                        format!("{:.2}s", duration.as_secs_f64())
                    };

                    tracing::error!(
                        "⏱️ {} {} - 错误: {} - IP: {} - 耗时: {} ❌",
                        method,
                        path,
                        e,
                        client_ip,
                        duration_str
                    );
                    Err(e)
                }
            }
        })
    }
}
