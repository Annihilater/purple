use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 用户订阅信息响应
#[derive(Debug, Serialize, ToSchema)]
pub struct UserSubscribeInfo {
    /// 订阅链接
    pub subscribe_url: String,
    /// 订阅令牌
    pub token: String,
    /// 流量信息
    pub traffic: TrafficInfo,
    /// 过期信息
    pub expire: ExpireInfo,
    /// 套餐信息
    pub plan: Option<PlanInfo>,
    /// 用户状态
    pub status: UserStatus,
}

/// 流量信息
#[derive(Debug, Serialize, ToSchema)]
pub struct TrafficInfo {
    /// 总流量(字节)
    pub total: i64,
    /// 已用上传流量(字节)
    pub upload: i64,
    /// 已用下载流量(字节)
    pub download: i64,
    /// 已用总流量(字节)
    pub used: i64,
    /// 剩余流量(字节)
    pub remaining: i64,
    /// 使用百分比
    pub usage_percent: f32,
    /// 格式化的流量信息
    pub total_gb: String,
    pub upload_gb: String,
    pub download_gb: String,
    pub used_gb: String,
    pub remaining_gb: String,
}

/// 过期信息
#[derive(Debug, Serialize, ToSchema)]
pub struct ExpireInfo {
    /// 过期时间戳
    pub expired_at: Option<i64>,
    /// 是否已过期
    pub is_expired: bool,
    /// 剩余天数
    pub remaining_days: i32,
    /// 过期时间文本
    pub expired_at_text: String,
}

/// 套餐信息
#[derive(Debug, Serialize, ToSchema)]
pub struct PlanInfo {
    pub id: i32,
    pub name: String,
    pub transfer_enable: i64,
    pub speed_limit: Option<i32>,
}

/// 用户状态
#[derive(Debug, Serialize, ToSchema)]
pub struct UserStatus {
    /// 是否被封禁
    pub banned: bool,
    /// 是否过期
    pub expired: bool,
    /// 状态文本
    pub status_text: String,
}

/// 重置订阅令牌请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetTokenRequest {
    /// 确认重置（防误操作）
    #[serde(default)]
    pub confirm: bool,
}

/// 客户端订阅配置响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeConfig {
    /// 节点列表
    pub servers: Vec<ServerNode>,
    /// 订阅信息
    pub subscribe_info: SubscribeInfoHeader,
}

/// 订阅信息头（用于客户端显示）
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeInfoHeader {
    /// 已用上传流量
    pub upload: i64,
    /// 已用下载流量
    pub download: i64,
    /// 总流量
    pub total: i64,
    /// 过期时间戳
    pub expire: i64,
}

/// 服务器节点
#[derive(Debug, Serialize, ToSchema)]
pub struct ServerNode {
    /// 节点名称
    pub name: String,
    /// 节点类型 (shadowsocks, vmess, trojan, hysteria)
    pub node_type: String,
    /// 节点配置
    pub config: serde_json::Value,
    /// 节点地址
    pub server: String,
    /// 端口
    pub port: i32,
    /// 是否可用
    pub available: bool,
    /// 倍率
    pub rate: f32,
}

/// 流量上报请求（节点使用）
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TrafficReportRequest {
    /// 节点ID
    pub node_id: i32,
    /// 用户流量数据
    pub data: Vec<UserTrafficData>,
}

/// 用户流量数据
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserTrafficData {
    /// 用户UUID
    pub uuid: String,
    /// 上传流量(字节)
    pub upload: i64,
    /// 下载流量(字节)
    pub download: i64,
}

/// 订阅链接响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeLinkResponse {
    /// 订阅链接
    pub subscribe_url: String,
    /// QR码（base64编码的图片）
    pub qr_code: Option<String>,
    /// 支持的客户端列表
    pub supported_clients: Vec<ClientInfo>,
}

/// 客户端信息
#[derive(Debug, Serialize, ToSchema)]
pub struct ClientInfo {
    /// 客户端名称
    pub name: String,
    /// 下载链接
    pub download_url: String,
    /// 支持的平台
    pub platforms: Vec<String>,
}

/// 订阅统计响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeStatsResponse {
    /// 今日流量使用
    pub today_traffic: i64,
    /// 本月流量使用
    pub month_traffic: i64,
    /// 登录统计
    pub login_stats: LoginStats,
    /// 在线设备数
    pub online_devices: i32,
}

/// 登录统计
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginStats {
    /// 最后登录时间
    pub last_login_at: Option<i64>,
    /// 最后登录IP
    pub last_login_ip: Option<String>,
    /// 登录次数（今日）
    pub today_logins: i32,
}

/// 重置订阅响应
#[derive(Debug, Serialize, ToSchema)]
pub struct ResetTokenResponse {
    /// 新的订阅令牌
    pub new_token: String,
    /// 新的订阅链接
    pub new_subscribe_url: String,
    /// 提示信息
    pub message: String,
}

impl TrafficInfo {
    pub fn new(total: i64, upload: i64, download: i64) -> Self {
        let used = upload + download;
        let remaining = (total - used).max(0);
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        let format_bytes = |bytes: i64| -> String {
            if bytes < 1024 {
                format!("{} B", bytes)
            } else if bytes < 1024 * 1024 {
                format!("{:.2} KB", bytes as f64 / 1024.0)
            } else if bytes < 1024 * 1024 * 1024 {
                format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
            }
        };

        Self {
            total,
            upload,
            download,
            used,
            remaining,
            usage_percent,
            total_gb: format_bytes(total),
            upload_gb: format_bytes(upload),
            download_gb: format_bytes(download),
            used_gb: format_bytes(used),
            remaining_gb: format_bytes(remaining),
        }
    }
}

impl ExpireInfo {
    pub fn new(expired_at: Option<i64>) -> Self {
        let current_time = chrono::Utc::now().timestamp();

        match expired_at {
            Some(expire_time) => {
                let is_expired = expire_time < current_time;
                let remaining_days = if is_expired {
                    0
                } else {
                    ((expire_time - current_time) / 86400).max(0) as i32
                };

                let expired_at_text = if is_expired {
                    "已过期".to_string()
                } else if remaining_days == 0 {
                    "今日过期".to_string()
                } else if remaining_days == 1 {
                    "明日过期".to_string()
                } else {
                    format!("{}天后过期", remaining_days)
                };

                Self {
                    expired_at,
                    is_expired,
                    remaining_days,
                    expired_at_text,
                }
            }
            None => Self {
                expired_at: None,
                is_expired: false,
                remaining_days: -1,
                expired_at_text: "永不过期".to_string(),
            },
        }
    }
}

impl UserStatus {
    pub fn new(banned: bool, expired: bool) -> Self {
        let status_text = if banned {
            "账户已封禁".to_string()
        } else if expired {
            "订阅已过期".to_string()
        } else {
            "正常".to_string()
        };

        Self {
            banned,
            expired,
            status_text,
        }
    }
}
