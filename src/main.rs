use actix_web::{App, HttpServer};
use log::info;

mod api;
mod config;
mod models;
mod services;
mod utils;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");
    let addr = format!("{}:{}", config.server_addr, config.server_port);
    
    info!("Starting server at: {}", addr);
    
    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .service(api::health_check)
    })
    .bind(&addr)?
    .run()
    .await
}
