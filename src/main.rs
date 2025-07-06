use actix_web::{web, App, HttpServer};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod models;
mod repositories;
mod services;
mod utils;

use crate::{api::openapi::ApiDoc, config::Config, repositories::UserRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let addr = format!("{}:{}", config.server_addr, config.server_port);

    // 创建数据库连接池
    let db_pool = config
        .database
        .create_pool()
        .await
        .expect("Failed to create database pool");

    // 创建用户仓库
    let user_repository = UserRepository::new(db_pool.clone());

    info!("数据库连接成功");
    info!("API服务启动于: http://{}", addr);
    info!("Swagger文档地址: http://{}/swagger-ui/", addr);

    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
            // API文档
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // API路由
            .service(api::health_check)
            .service(api::create_user)
            .service(api::get_user)
    })
    .bind(&addr)?
    .run()
    .await
}
