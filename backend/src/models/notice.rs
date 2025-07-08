use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

/// 公告模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Notice {
    pub id: i32,
    pub title: String,
    pub content: String,
    /// 是否显示
    pub show: bool,
    /// 图片URL
    pub img_url: Option<String>,
    /// 标签
    pub tags: Option<String>,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 创建公告请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNoticeRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 10000))]
    pub content: String,
    /// 是否显示
    #[serde(default = "default_show")]
    pub show: bool,
    /// 图片URL
    #[validate(url)]
    pub img_url: Option<String>,
    /// 标签（逗号分隔）
    #[validate(length(max = 255))]
    pub tags: Option<String>,
}

/// 更新公告请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateNoticeRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 10000))]
    pub content: Option<String>,
    /// 是否显示
    pub show: Option<bool>,
    /// 图片URL
    #[validate(url)]
    pub img_url: Option<String>,
    /// 标签（逗号分隔）
    #[validate(length(max = 255))]
    pub tags: Option<String>,
}

/// 公告响应
#[derive(Debug, Serialize, ToSchema)]
pub struct NoticeResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub show: bool,
    pub img_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 公告列表响应
#[derive(Debug, Serialize, ToSchema)]
pub struct NoticeListResponse {
    pub notices: Vec<NoticeResponse>,
    pub total: i64,
}

fn default_show() -> bool {
    true
}

impl From<Notice> for NoticeResponse {
    fn from(notice: Notice) -> Self {
        let tags = notice.tags.map(|tags_str| {
            tags_str
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect()
        });

        Self {
            id: notice.id,
            title: notice.title,
            content: notice.content,
            show: notice.show,
            img_url: notice.img_url,
            tags,
            created_at: notice.created_at,
            updated_at: notice.updated_at,
        }
    }
}
