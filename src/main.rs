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

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(env_filter.into()))
        .with_thread_ids(config.log.with_thread_ids)
        .with_line_number(config.log.with_line_number)
        .with_file(config.log.with_file)
        .with_target(config.log.with_target)
        .init();

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
    info!("API服务启动于: http://{}:{}", config.server_addr, config.server_port);
    info!("Swagger文档地址: http://{}:{}/swagger-ui/", config.server_addr, config.server_port);

    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
            .app_data(web::Data::new(plan_repository.clone()))
            .app_data(web::Data::new(coupon_repository.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            // API文档
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
