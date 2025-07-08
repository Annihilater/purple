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
//! - 🔄 **统一响应格式**: 标准化的 RESTful API 响应规范
//! - ⚡ **微秒级监控**: 精确到微秒的响应时间监控和智能预警
//! - 🎯 **智能路由**: 避免路径冲突的路由配置系统
//! - 📄 **分页支持**: 统一的分页查询和响应格式
//! - 🔍 **请求追踪**: 每个请求的唯一标识符，便于问题排查
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use purple_backend::startup::Application;
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
//! use purple_backend::services::AuthService;
//! use purple_backend::models::auth::LoginRequest;
//!
//! async fn login_example() -> anyhow::Result<()> {
//!     use purple_backend::repositories::UserRepository;
//!     use sqlx::PgPool;
//!
//!     let pool = PgPool::connect("postgres://localhost/purple").await?;
//!     let user_repo = UserRepository::new(pool);
//!     let jwt_secret = "your-secret-key".to_string();
//!     let auth_service = AuthService::new(user_repo, jwt_secret);
//!     
//!     let login_request = LoginRequest {
//!         username: "admin@test.com".to_string(),
//!         password: "secure_admin_password_123".to_string(),
//!     };
//!     
//!     let token_response = auth_service.login(login_request).await?;
//!     println!("Access token: {}", token_response.access_token);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### 统一响应格式 API
//!
//! Purple 使用统一的响应格式，确保所有 API 返回一致的数据结构：
//!
//! ```rust
//! use purple_backend::common::response_v2::{ApiResponse, IntoHttpResponse, ApiError};
//! use purple_backend::common::ErrorCode;
//! use actix_web::HttpResponse;
//!
//! // 成功响应
//! let response = ApiResponse::success("操作成功");
//! let http_response: HttpResponse = response.into_http_response();
//!
//! // 分页响应
//! let data = vec!["item1", "item2"];
//! let page_response = ApiResponse::page(data, 1, 10, 100);
//! let http_page_response: HttpResponse = page_response.into_http_response();
//!
//! // 错误响应
//! let error_response = ApiResponse::error(ErrorCode::UserNotFound);
//! let http_error_response: HttpResponse = error_response.into_http_response();
//!
//! // 自定义错误响应
//! let custom_error = ApiError::with_details(
//!     ErrorCode::ValidationError,
//!     "参数验证失败".to_string()
//! );
//! ```
//!
//! ### 响应格式示例
//!
//! 所有 API 响应都遵循统一格式：
//!
//! ```json
//! // 成功响应
//! {
//!   "success": true,
//!   "data": { "id": 1, "name": "示例数据" },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//!
//! // 分页响应
//! {
//!   "success": true,
//!   "data": [{"id": 1}, {"id": 2}],
//!   "pagination": {
//!     "page": 1,
//!     "page_size": 10,
//!     "total": 100,
//!     "total_pages": 10,
//!     "has_next": true,
//!     "has_prev": false
//!   },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//!
//! // 错误响应
//! {
//!   "success": false,
//!   "error": {
//!     "code": "USER_NOT_FOUND",
//!     "message": "用户未找到",
//!     "details": "用户ID 123 不存在"
//!   },
//!   "meta": {
//!     "timestamp": 1751938399,
//!     "request_id": "uuid-here"
//!   }
//! }
//! ```
//!
//! ## 中间件系统
//!
//! Purple 提供了完整的中间件系统：
//!
//! ### 认证中间件
//! ```rust,no_run
//! use purple_backend::middleware::Auth;
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
//! use purple_backend::middleware::{RequestLogger, RequestTimer};
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
//! use purple_backend::common::ErrorCode;
//!
//! let error_code = ErrorCode::UserNotFound;
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
//! use purple_backend::config::{Config, DatabaseConfig};
//! use anyhow::Result;
//!
//! fn load_config() -> Result<()> {
//!     // 从环境变量加载配置
//!     let config = Config::from_env()?;
//!     Ok(())
//! }
//! ```
//!
//! 更多信息请参考：
//! - [项目文档](https://github.com/your-org/purple/docs)
//! - [API 文档](https://github.com/your-org/purple/docs/api)
//! - [开发指南](https://github.com/your-org/purple/docs/development)

// 导入共享库
pub use purple_shared;

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
