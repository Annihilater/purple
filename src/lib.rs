//! # Purple - 现代化 Web API 项目
//!
//! Purple 是一个基于 Rust 和 Actix-web 构建的现代化 Web API 项目，
//! 提供用户管理、套餐管理、优惠券系统等完整功能。
//!
//! ## 特性
//!
//! - 🚀 **高性能**: 基于 Rust 和 Actix-web，提供极致性能
//! - 🛡️ **安全可靠**: JWT 认证、参数验证、SQL 注入防护
//! - 📊 **完整监控**: 请求日志、性能监控、错误追踪
//! - 📚 **自动文档**: OpenAPI/Swagger 自动生成 API 文档
//! - 🔧 **易于维护**: 分层架构、统一错误处理、类型安全
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use purple::startup::Application;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let application = Application::build().await?;
//!     application.run().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## 架构概览
//!
//! Purple 采用分层架构设计：
//!
//! - **API 层**: HTTP 请求处理器和 OpenAPI 文档
//! - **服务层**: 业务逻辑处理和 JWT 认证
//! - **仓库层**: PostgreSQL 数据访问抽象
//! - **模型层**: 数据结构和领域模型
//! - **中间件系统**: 认证、CORS、请求日志、性能监控
//!
//! ## 主要模块
//!
//! - [`api`] - HTTP API 端点和 OpenAPI 文档
//! - [`services`] - 业务逻辑服务
//! - [`repositories`] - 数据访问层
//! - [`models`] - 数据模型和结构体
//! - [`middleware`] - 中间件系统
//! - [`common`] - 通用组件和响应系统
//! - [`config`] - 配置管理
//!
//! ## 示例
//!
//! ### 用户认证
//!
//! ```rust,no_run
//! use purple::services::AuthService;
//! use purple::models::auth::LoginRequest;
//!
//! async fn login_example() -> anyhow::Result<()> {
//!     let auth_service = AuthService::new(/* dependencies */);
//!     
//!     let login_request = LoginRequest {
//!         username: "admin".to_string(),
//!         password: "password123".to_string(),
//!     };
//!     
//!     let token_response = auth_service.login(login_request).await?;
//!     println!("Access token: {}", token_response.access_token);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### API 响应处理
//!
//! ```rust
//! use purple::common::response_v2::{ApiResponse, IntoHttpResponse};
//! use purple::common::ErrorCode;
//!
//! // 成功响应
//! let response = ApiResponse::success("操作成功");
//! let http_response = response.into_http_response();
//!
//! // 错误响应
//! let error_response = ApiResponse::error(ErrorCode::UserNotFound);
//! let http_error_response = error_response.into_http_response();
//! ```
//!
//! ## 中间件系统
//!
//! Purple 提供了完整的中间件系统：
//!
//! ### 认证中间件
//! ```rust,no_run
//! use purple::middleware::Auth;
//! use actix_web::{web, App};
//!
//! let app = App::new()
//!     .service(
//!         web::scope("/api/users")
//!             .wrap(Auth::new())
//!             // 添加需要认证的路由
//!     );
//! ```
//!
//! ### 请求监控中间件
//! ```rust,no_run
//! use purple::middleware::{RequestLogger, RequestTimer};
//! use actix_web::App;
//!
//! let app = App::new()
//!     .wrap(RequestTimer::new())   // 请求耗时统计
//!     .wrap(RequestLogger::new()); // 请求日志记录
//! ```
//!
//! ## 错误处理
//!
//! Purple 使用统一的错误代码系统：
//!
//! ```rust
//! use purple::common::ErrorCode;
//!
//! match error_code {
//!     ErrorCode::Success => println!("操作成功"),
//!     ErrorCode::UserNotFound => println!("用户不存在"),
//!     ErrorCode::InvalidCredentials => println!("用户名或密码错误"),
//!     _ => println!("其他错误"),
//! }
//! ```
//!
//! ## 配置管理
//!
//! ```rust,no_run
//! use purple::config::{Config, DatabaseConfig};
//!
//! // 从环境变量加载配置
//! let config = Config::from_env()?;
//! let db_config = DatabaseConfig::from_env()?;
//! ```
//!
//! 更多信息请参考：
//! - [项目文档](https://github.com/your-org/purple/docs)
//! - [API 文档](https://github.com/your-org/purple/docs/api)
//! - [开发指南](https://github.com/your-org/purple/docs/development)

pub mod api;
pub mod app_state;
pub mod common;
pub mod config;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod startup;
pub mod utils;

// 重新导出常用类型和函数
pub use app_state::AppState;
pub use startup::Application;

/// Purple 项目的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Purple 项目的名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Purple 项目的描述
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
