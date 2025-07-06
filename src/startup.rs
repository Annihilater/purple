use actix_web::{web, App, HttpServer};
use anyhow::Result;
use tracing::info;

use crate::{
    app_state::AppState,
    config::{Config, DatabaseConfig},
    logging::{init_logging, LogGuard},
    routes::configure_routes,
};

/// 应用启动器
///
/// 负责应用的完整启动流程，包括：
/// - 环境变量加载
/// - 配置初始化
/// - 日志系统初始化
/// - 应用状态创建
/// - HTTP服务器启动
pub struct Application {
    server: actix_web::dev::Server,
    _log_guard: LogGuard,
}

impl Application {
    /// 构建应用实例
    ///
    /// 执行完整的应用初始化流程
    pub async fn build() -> Result<Self> {
        // 加载环境变量
        dotenv::dotenv().ok();

        // 加载配置
        let config = Config::from_env()?;
        let database_config = DatabaseConfig::from_env()?;

        // 初始化日志系统
        let log_guard = init_logging(&config.log)?;

        // 创建应用状态
        let app_state = AppState::new(&database_config).await?;

        // 记录启动信息
        log_startup_info(&config);

        // 创建HTTP服务器
        let server = create_server(app_state, &config)?;

        Ok(Self {
            server,
            _log_guard: log_guard,
        })
    }

    /// 运行应用
    ///
    /// 启动HTTP服务器并等待连接
    pub async fn run(self) -> std::io::Result<()> {
        self.server.await
    }
}

/// 创建HTTP服务器
fn create_server(app_state: AppState, config: &Config) -> Result<actix_web::dev::Server> {
    let app_state_for_factory = app_state.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                app_state_for_factory.user_repository.clone(),
            ))
            .app_data(web::Data::new(
                app_state_for_factory.plan_repository.clone(),
            ))
            .app_data(web::Data::new(
                app_state_for_factory.coupon_repository.clone(),
            ))
            .app_data(web::Data::new(app_state_for_factory.auth_service.clone()))
            .configure(configure_routes)
    })
    .bind((config.server_addr.as_str(), config.server_port))?
    .run();

    Ok(server)
}

/// 记录应用启动信息
fn log_startup_info(config: &Config) {
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
}
