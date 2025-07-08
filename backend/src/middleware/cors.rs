use actix_cors::Cors as ActixCors;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

/// CORS中间件配置
pub struct Cors;

impl Cors {
    /// 创建开发环境的CORS配置（宽松）
    pub fn development() -> ActixCors {
        ActixCors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600)
    }

    /// 创建生产环境的CORS配置（严格）
    pub fn production(allowed_origins: Vec<&str>) -> ActixCors {
        let mut cors = ActixCors::default();

        for origin in allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        cors.allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600)
    }

    /// 创建自定义CORS配置
    pub fn custom() -> ActixCors {
        ActixCors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://yourdomain.com")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                AUTHORIZATION,
                ACCEPT,
                CONTENT_TYPE,
                actix_web::http::header::HeaderName::from_static("x-requested-with"),
            ])
            .supports_credentials()
            .max_age(3600)
    }
}
