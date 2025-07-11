use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 服务器协议枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ServerProtocol {
    Shadowsocks,
    Vmess,
    Trojan,
    Hysteria,
}

impl std::fmt::Display for ServerProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerProtocol::Shadowsocks => write!(f, "shadowsocks"),
            ServerProtocol::Vmess => write!(f, "vmess"),
            ServerProtocol::Trojan => write!(f, "trojan"),
            ServerProtocol::Hysteria => write!(f, "hysteria"),
        }
    }
}

/// 服务器状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Online,      // 在线
    Offline,     // 离线
    Warning,     // 异常
    Maintenance, // 维护
}

/// 服务器数据模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Server {
    pub id: u32,
    pub protocol: ServerProtocol,
    pub name: String,
    pub host: String,
    pub port: String, // 支持端口段 "80-90"
    pub server_port: u16,
    pub rate: f32,                   // 流量倍率
    pub show: bool,                  // 是否显示给用户
    pub sort: Option<u32>,           // 排序权重
    pub group_ids: Vec<u32>,         // 权限组ID数组
    pub route_ids: Option<Vec<u32>>, // 路由规则ID数组
    pub parent_id: Option<u32>,      // 父节点ID（用于中转）
    pub tags: Option<Vec<String>>,   // 节点标签
    pub config: serde_json::Value,   // 协议特定配置
    pub created_at: i64,
    pub updated_at: i64,
}

/// 服务器创建请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateServerRequest {
    pub protocol: ServerProtocol,
    pub name: String,
    pub host: String,
    pub port: String,
    pub server_port: u16,
    pub rate: f32,
    pub show: bool,
    pub sort: Option<u32>,
    pub group_ids: Vec<u32>,
    pub route_ids: Option<Vec<u32>>,
    pub parent_id: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub config: serde_json::Value, // 协议特定配置
}

/// 服务器更新请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub server_port: Option<u16>,
    pub rate: Option<f32>,
    pub show: Option<bool>,
    pub sort: Option<u32>,
    pub group_ids: Option<Vec<u32>>,
    pub route_ids: Option<Vec<u32>>,
    pub parent_id: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub config: Option<serde_json::Value>,
}

/// 服务器响应（包含运行时状态）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServerResponse {
    pub id: u32,
    pub protocol: ServerProtocol,
    pub name: String,
    pub host: String,
    pub port: String,
    pub server_port: u16,
    pub rate: f32,
    pub show: bool,
    pub sort: Option<u32>,
    pub group_ids: Vec<u32>,
    pub route_ids: Option<Vec<u32>>,
    pub parent_id: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub config: serde_json::Value,
    pub status: ServerStatus,          // 运行时状态
    pub load: f32,                     // 负载率
    pub online_users: u32,             // 在线用户数
    pub traffic_today: String,         // 今日流量
    pub last_check_at: Option<String>, // 最后检查时间
    pub created_at: String,
    pub updated_at: String,
}

/// 服务器组
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServerGroup {
    pub id: u32,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 服务器组创建请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateServerGroupRequest {
    pub name: String,
}

/// 服务器组更新请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateServerGroupRequest {
    pub name: String,
}

/// 路由规则
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServerRoute {
    pub id: u32,
    pub remarks: String,
    pub match_rules: Vec<String>, // 匹配规则
    pub action: String,           // block, dns
    pub action_value: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 路由规则创建请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateServerRouteRequest {
    pub remarks: String,
    pub match_rules: Vec<String>,
    pub action: String,
    pub action_value: Option<String>,
}

/// 路由规则更新请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateServerRouteRequest {
    pub remarks: Option<String>,
    pub match_rules: Option<Vec<String>>,
    pub action: Option<String>,
    pub action_value: Option<String>,
}

/// 服务器统计
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServerStats {
    pub id: u32,
    pub server_id: u32,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub record_type: String, // d=day, m=month
    pub record_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 服务器日志
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServerLog {
    pub id: u64,
    pub user_id: u32,
    pub server_id: u32,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub rate: f32,
    pub log_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 服务器运行时状态（缓存数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRuntimeStatus {
    pub server_id: u32,
    pub online_users: u32,
    pub last_check_at: Option<i64>,
    pub last_push_at: Option<i64>,
    pub load: f32,            // CPU/内存负载
    pub available_status: u8, // 0=离线, 1=异常, 2=正常
}

/// 批量排序请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServerSortRequest {
    pub servers: Vec<ServerSortItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServerSortItem {
    pub id: u32,
    pub sort: u32,
}

/// 复制服务器请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CopyServerRequest {
    pub name: String,             // 新服务器名称
    pub host: Option<String>,     // 可选：新主机地址
    pub server_port: Option<u16>, // 可选：新服务端口
}

/// 用户可见的服务器信息（隐藏敏感配置）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserServerResponse {
    pub id: u32,
    pub name: String,
    pub host: String,
    pub port: String,
    pub server_port: u16,
    pub rate: f32,
    pub tags: Option<Vec<String>>,
    pub status: ServerStatus,
    pub load: f32,
    pub config: serde_json::Value, // 客户端配置（已过滤敏感信息）
}

// 协议特定配置结构体

/// Shadowsocks 配置
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ShadowsocksConfig {
    pub cipher: String,            // 加密方式
    pub password: String,          // 密码
    pub obfs: Option<String>,      // 混淆方式
    pub obfs_host: Option<String>, // 混淆主机
}

/// VMess 配置
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VmessConfig {
    pub uuid: String,                                // UUID
    pub alter_id: u16,                               // 额外ID
    pub security: String,                            // 加密方式
    pub network: String,                             // 传输协议
    pub network_settings: Option<serde_json::Value>, // 传输设置
    pub tls: bool,                                   // TLS开关
    pub tls_settings: Option<serde_json::Value>,     // TLS设置
}

/// Trojan 配置
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TrojanConfig {
    pub password: String,            // 密码
    pub allow_insecure: bool,        // 允许不安全连接
    pub server_name: Option<String>, // 服务器名称（SNI）
}

/// Hysteria 配置
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HysteriaConfig {
    pub password: String,            // 密码
    pub up_mbps: u32,                // 上传带宽限制
    pub down_mbps: u32,              // 下载带宽限制
    pub server_name: Option<String>, // 服务器名称
    pub insecure: bool,              // 不安全连接
}

impl From<Server> for ServerResponse {
    fn from(server: Server) -> Self {
        Self {
            id: server.id,
            protocol: server.protocol,
            name: server.name,
            host: server.host,
            port: server.port,
            server_port: server.server_port,
            rate: server.rate,
            show: server.show,
            sort: server.sort,
            group_ids: server.group_ids,
            route_ids: server.route_ids,
            parent_id: server.parent_id,
            tags: server.tags,
            config: server.config,
            status: ServerStatus::Offline, // 默认离线，需要从缓存更新
            load: 0.0,
            online_users: 0,
            traffic_today: "0 B".to_string(),
            last_check_at: None,
            created_at: chrono::DateTime::from_timestamp(server.created_at, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            updated_at: chrono::DateTime::from_timestamp(server.updated_at, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}
