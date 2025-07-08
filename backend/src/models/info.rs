use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 项目信息模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectInfo {
    /// 项目名称
    pub name: String,
    /// 项目版本
    pub version: String,
    /// 项目描述
    pub description: String,
    /// 项目作者
    pub author: String,
    /// 构建框架
    pub framework: String,
    /// 数据库类型
    pub database: String,
    /// 服务器状态
    pub status: String,
    /// 启动时间
    pub uptime: String,
    /// API 文档地址
    pub api_docs: String,
    /// 健康检查地址
    pub health_check: String,
}

impl ProjectInfo {
    /// 创建项目信息实例
    pub fn new() -> Self {
        Self {
            name: "Purple".to_string(),
            version: "0.1.0".to_string(),
            description: "基于 Rust 和 Actix-web 构建的 Web API 项目".to_string(),
            author: "Purple Team".to_string(),
            framework: "Actix-web".to_string(),
            database: "PostgreSQL".to_string(),
            status: "运行中".to_string(),
            uptime: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
            api_docs: "/swagger-ui/".to_string(),
            health_check: "/health".to_string(),
        }
    }
}

impl Default for ProjectInfo {
    fn default() -> Self {
        Self::new()
    }
}
