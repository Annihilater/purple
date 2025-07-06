use actix_web::{web, App, HttpServer};

use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod models;
mod repositories;
mod services;
mod utils;

use crate::{
    api::openapi::ApiDoc,
    config::{database::DatabaseConfig, Config},
    repositories::{CouponRepository, PlanRepository, UserRepository},
    services::AuthService,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");

    // 初始化日志
    let env_filter = match config.log.level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // 创建日志文件目录
    let log_file_path = std::path::Path::new(&config.log.file_path);
    if let Some(parent) = log_file_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create log directory");
    }

    // 创建文件日志输出器，支持自定义文件名格式
    let log_directory = log_file_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("logs"));

    let file_name = log_file_path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("app.log"))
        .to_string_lossy();

    // 解析文件名，获取基础名称和扩展名
    let (base_name, extension) = if let Some(pos) = file_name.rfind('.') {
        (&file_name[..pos], &file_name[pos..])
    } else {
        (file_name.as_ref(), "")
    };

    // 创建自定义格式的日志文件名: app-2025-07-06.log
    // 使用 RollingFileAppender 来精确控制文件名格式
    use tracing_appender::rolling::{RollingFileAppender, Rotation};

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_directory,
        format!("{}{}", base_name, extension),
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 创建分层日志订阅器
    let console_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(config.log.with_thread_ids)
        .with_line_number(config.log.with_line_number)
        .with_file(config.log.with_file)
        .with_target(config.log.with_target);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_thread_ids(config.log.with_thread_ids)
        .with_line_number(config.log.with_line_number)
        .with_file(config.log.with_file)
        .with_target(config.log.with_target);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(env_filter.into()))
        .with(console_layer)
        .with(file_layer)
        .init();

    // 必须保持_guard活跃以确保日志文件写入
    let _guard = Box::leak(Box::new(_guard));

    // 加载数据库配置
    let database_config = DatabaseConfig::from_env().expect("Failed to load database config");

    // 创建数据库连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_config.url)
        .await
        .expect("Failed to create pool");

    // 初始化仓库
    let user_repository = UserRepository::new(pool.clone());
    let plan_repository = PlanRepository::new(pool.clone());
    let coupon_repository = CouponRepository::new(pool.clone());

    // 创建服务实例
    let auth_service = AuthService::new(
        user_repository.clone(),
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    );

    info!("数据库连接成功");
    info!(
        "API服务启动于: http://{}:{}/",
        config.server_addr, config.server_port
    );
    info!(
        "Swagger文档地址: http://{}:{}/swagger-ui/",
        config.server_addr, config.server_port
    );
    info!(
        "OpenAPI规范地址: http://{}:{}/api-docs/openapi.json",
        config.server_addr, config.server_port
    );

    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
            .app_data(web::Data::new(plan_repository.clone()))
            .app_data(web::Data::new(coupon_repository.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            // OpenAPI JSON 端点
            .route(
                "/api-docs/openapi.json",
                web::get().to(|| async {
                    actix_web::HttpResponse::Ok()
                        .content_type("application/json")
                        .json(ApiDoc::openapi())
                }),
            )
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // API路由
            .service(api::health_check)
            // 认证接口
            .service(api::register)
            .service(api::login)
            // 用户接口
            .service(
                web::scope("/api/users")
                    .service(api::create_user)
                    .service(api::get_users)
                    .service(api::get_user)
                    .service(api::update_user)
                    .service(api::delete_user)
                    .service(api::update_user_status),
            )
            // 套餐接口
            .service(
                web::scope("/api/plans")
                    .service(api::create_plan)
                    .service(api::list_plans)
                    .service(api::get_plan)
                    .service(api::update_plan)
                    .service(api::delete_plan)
                    .service(api::get_enabled_plans),
            )
            // 优惠券接口
            .service(
                web::scope("/api/coupons")
                    .service(api::create_coupon)
                    .service(api::list_coupons)
                    .service(api::get_coupon)
                    .service(api::update_coupon)
                    .service(api::delete_coupon)
                    .service(api::verify_coupon),
            )
    })
    .bind((config.server_addr.as_str(), config.server_port))?
    .run()
    .await
}
