use actix_web::{App, HttpServer, web};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod models;
mod services;
mod utils;
mod repositories;

use crate::{
    api::openapi::ApiDoc,
    config::Config,
    repositories::UserRepository,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");
    let addr = format!("{}:{}", config.server_addr, config.server_port);
    
    // 创建数据库连接池
    let db_pool = config.database.create_pool()
        .await
        .expect("Failed to create database pool");
    
    // 创建用户仓库
    let user_repository = UserRepository::new(db_pool.clone());
    
    info!("Starting server at: {}", addr);
    
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
