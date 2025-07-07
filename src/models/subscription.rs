use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

/// 用户订阅模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Subscription {
    pub id: i32,
    pub user_id: i32,
    pub plan_id: i32,
    /// 订阅令牌
    pub token: String,
    /// 订阅总流量(GB)
    pub transfer_enable: i64,
    /// 已使用上行流量(GB)
    pub u: i64,
    /// 已使用下行流量(GB)
    pub d: i64,
    /// 过期时间戳
    pub expired_at: i32,
    /// 最后登录时间
    pub last_login_at: Option<i32>,
    /// 最后登录IP
    pub last_login_ip: Option<String>,
    /// 最后签到时间
    pub last_checkin_at: Option<i32>,
    /// 最后重置时间
    pub last_reset_at: Option<i32>,
    /// 订阅状态：0正常 1禁用
    pub status: i32,
    /// 备注
    pub remarks: Option<String>,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 用户订阅创建请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSubscriptionRequest {
    pub user_id: i32,
    pub plan_id: i32,
    /// 订阅总流量(GB)
    #[validate(range(min = 1))]
    pub transfer_enable: i64,
    /// 过期时间戳
    #[validate(range(min = 1))]
    pub expired_at: i32,
    /// 备注
    #[validate(length(max = 255))]
    pub remarks: Option<String>,
}

/// 用户订阅更新请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSubscriptionRequest {
    pub plan_id: Option<i32>,
    /// 订阅总流量(GB)
    #[validate(range(min = 1))]
    pub transfer_enable: Option<i64>,
    /// 过期时间戳
    #[validate(range(min = 1))]
    pub expired_at: Option<i32>,
    /// 订阅状态：0正常 1禁用
    pub status: Option<i32>,
    /// 备注
    #[validate(length(max = 255))]
    pub remarks: Option<String>,
}

/// 重置订阅请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetSubscriptionRequest {
    /// 是否重置流量
    #[serde(default)]
    pub reset_traffic: bool,
    /// 是否重置过期时间
    #[serde(default)]
    pub reset_expired: bool,
    /// 新的过期时间戳（如果重置过期时间）
    pub new_expired_at: Option<i32>,
}

/// 订阅响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionResponse {
    pub id: i32,
    pub user_id: i32,
    pub plan_id: i32,
    pub plan_name: Option<String>,
    pub token: String,
    pub transfer_enable: i64,
    pub transfer_enable_gb: String,
    pub u: i64,
    pub d: i64,
    pub used_traffic: i64,
    pub used_traffic_gb: String,
    pub remaining_traffic: i64,
    pub remaining_traffic_gb: String,
    pub usage_percent: f32,
    pub expired_at: i32,
    pub expired_at_text: String,
    pub is_expired: bool,
    pub days_remaining: i32,
    pub last_login_at: Option<i32>,
    pub last_login_at_text: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_checkin_at: Option<i32>,
    pub last_checkin_at_text: Option<String>,
    pub last_reset_at: Option<i32>,
    pub last_reset_at_text: Option<String>,
    pub status: i32,
    pub status_text: String,
    pub remarks: Option<String>,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 订阅列表响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionListResponse {
    pub subscriptions: Vec<SubscriptionResponse>,
    pub total: i64,
}

/// 订阅统计响应
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionStatsResponse {
    pub total_subscriptions: i64,
    pub active_subscriptions: i64,
    pub expired_subscriptions: i64,
    pub disabled_subscriptions: i64,
    pub total_traffic_gb: i64,
    pub used_traffic_gb: i64,
    pub remaining_traffic_gb: i64,
    pub average_usage_percent: f32,
}

/// 用户订阅详情响应
#[derive(Debug, Serialize, ToSchema)]
pub struct UserSubscriptionResponse {
    pub subscription: SubscriptionResponse,
    pub subscribe_url: String,
    pub subscribe_info: SubscribeInfo,
}

/// 订阅信息
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeInfo {
    pub upload: i64,
    pub download: i64,
    pub total: i64,
    pub expire: i64,
}

impl From<Subscription> for SubscriptionResponse {
    fn from(subscription: Subscription) -> Self {
        let used_traffic = subscription.u + subscription.d;
        let remaining_traffic = subscription.transfer_enable - used_traffic;
        let usage_percent = if subscription.transfer_enable > 0 {
            (used_traffic as f32 / subscription.transfer_enable as f32) * 100.0
        } else {
            0.0
        };

        let current_time = chrono::Utc::now().timestamp() as i32;
        let is_expired = subscription.expired_at < current_time;
        let days_remaining = if is_expired {
            0
        } else {
            ((subscription.expired_at - current_time) / 86400).max(0)
        };

        let status_text = match subscription.status {
            0 => "正常".to_string(),
            1 => "禁用".to_string(),
            _ => "未知".to_string(),
        };

        let expired_at_text = if is_expired {
            "已过期".to_string()
        } else {
            format!("{}天后过期", days_remaining)
        };

        let format_traffic = |bytes: i64| -> String {
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

        let format_timestamp = |timestamp: Option<i32>| -> Option<String> {
            timestamp.map(|ts| {
                let datetime = chrono::NaiveDateTime::from_timestamp(ts as i64, 0);
                datetime.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "无效时间".to_string())
            })
        };

        Self {
            id: subscription.id,
            user_id: subscription.user_id,
            plan_id: subscription.plan_id,
            plan_name: None, // 需要从plan表查询
            token: subscription.token,
            transfer_enable: subscription.transfer_enable,
            transfer_enable_gb: format_traffic(subscription.transfer_enable),
            u: subscription.u,
            d: subscription.d,
            used_traffic,
            used_traffic_gb: format_traffic(used_traffic),
            remaining_traffic,
            remaining_traffic_gb: format_traffic(remaining_traffic),
            usage_percent,
            expired_at: subscription.expired_at,
            expired_at_text,
            is_expired,
            days_remaining,
            last_login_at: subscription.last_login_at,
            last_login_at_text: format_timestamp(subscription.last_login_at),
            last_login_ip: subscription.last_login_ip,
            last_checkin_at: subscription.last_checkin_at,
            last_checkin_at_text: format_timestamp(subscription.last_checkin_at),
            last_reset_at: subscription.last_reset_at,
            last_reset_at_text: format_timestamp(subscription.last_reset_at),
            status: subscription.status,
            status_text,
            remarks: subscription.remarks,
            created_at: subscription.created_at,
            updated_at: subscription.updated_at,
        }
    }
}