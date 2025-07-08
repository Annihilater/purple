use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

/// 工单模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Ticket {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    /// 优先级：false普通 true紧急
    pub level: bool,
    /// 状态：false已开启 true已关闭
    pub status: bool,
    /// 回复状态：false待回复 true已回复
    pub reply_status: bool,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 工单消息模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TicketMessage {
    pub id: i32,
    pub user_id: i32,
    pub ticket_id: i32,
    pub message: String,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 创建工单请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTicketRequest {
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    #[validate(length(min = 1, max = 10000))]
    pub message: String,
    /// 优先级：false普通 true紧急
    #[serde(default)]
    pub level: bool,
}

/// 回复工单请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReplyTicketRequest {
    #[validate(length(min = 1, max = 10000))]
    pub message: String,
}

/// 更新工单状态请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTicketStatusRequest {
    /// 状态：false已开启 true已关闭
    pub status: bool,
}

/// 工单响应
#[derive(Debug, Serialize, ToSchema)]
pub struct TicketResponse {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    pub level: bool,
    pub level_text: String,
    pub status: bool,
    pub status_text: String,
    pub reply_status: bool,
    pub reply_status_text: String,
    pub created_at: i32,
    pub updated_at: i32,
    /// 最新消息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_message: Option<TicketMessageResponse>,
    /// 消息数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
}

/// 工单消息响应
#[derive(Debug, Serialize, ToSchema)]
pub struct TicketMessageResponse {
    pub id: i32,
    pub user_id: i32,
    pub ticket_id: i32,
    pub message: String,
    pub created_at: i32,
    pub updated_at: i32,
    /// 是否为管理员回复
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
}

/// 工单列表响应
#[derive(Debug, Serialize, ToSchema)]
pub struct TicketListResponse {
    pub tickets: Vec<TicketResponse>,
    pub total: i64,
}

/// 工单详情响应（包含所有消息）
#[derive(Debug, Serialize, ToSchema)]
pub struct TicketDetailResponse {
    pub ticket: TicketResponse,
    pub messages: Vec<TicketMessageResponse>,
}

impl From<Ticket> for TicketResponse {
    fn from(ticket: Ticket) -> Self {
        let level_text = if ticket.level {
            "紧急".to_string()
        } else {
            "普通".to_string()
        };

        let status_text = if ticket.status {
            "已关闭".to_string()
        } else {
            "已开启".to_string()
        };

        let reply_status_text = if ticket.reply_status {
            "已回复".to_string()
        } else {
            "待回复".to_string()
        };

        Self {
            id: ticket.id,
            user_id: ticket.user_id,
            subject: ticket.subject,
            level: ticket.level,
            level_text,
            status: ticket.status,
            status_text,
            reply_status: ticket.reply_status,
            reply_status_text,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
            latest_message: None,
            message_count: None,
        }
    }
}

impl From<TicketMessage> for TicketMessageResponse {
    fn from(message: TicketMessage) -> Self {
        Self {
            id: message.id,
            user_id: message.user_id,
            ticket_id: message.ticket_id,
            message: message.message,
            created_at: message.created_at,
            updated_at: message.updated_at,
            is_admin: None,
        }
    }
}
