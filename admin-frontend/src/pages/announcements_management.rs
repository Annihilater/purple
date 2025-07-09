use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub type_: String,    // info, warning, error, success
    pub priority: String, // low, medium, high
    pub status: String,   // draft, published, archived
    pub author: String,
    pub target_audience: String, // all, vip, new_users
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub views: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl Announcement {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Announcement {
                id: 1,
                title: "系统维护通知".to_string(),
                content: "系统将于明天凌晨2点进行维护，预计维护时间为2小时，期间服务可能会中断。"
                    .to_string(),
                type_: "warning".to_string(),
                priority: "high".to_string(),
                status: "published".to_string(),
                author: "系统管理员".to_string(),
                target_audience: "all".to_string(),
                start_date: Some("2024-01-15 00:00:00".to_string()),
                end_date: Some("2024-01-20 23:59:59".to_string()),
                views: 1256,
                created_at: "2024-01-14 15:30:00".to_string(),
                updated_at: "2024-01-14 15:30:00".to_string(),
            },
            Announcement {
                id: 2,
                title: "新增节点通知".to_string(),
                content: "我们新增了3个高速节点，分别位于香港、新加坡和美国西部，欢迎大家使用。"
                    .to_string(),
                type_: "success".to_string(),
                priority: "medium".to_string(),
                status: "published".to_string(),
                author: "运维团队".to_string(),
                target_audience: "all".to_string(),
                start_date: Some("2024-01-10 00:00:00".to_string()),
                end_date: None,
                views: 2341,
                created_at: "2024-01-10 10:00:00".to_string(),
                updated_at: "2024-01-10 10:00:00".to_string(),
            },
            Announcement {
                id: 3,
                title: "VIP用户专享优惠".to_string(),
                content: "VIP用户可享受全场8折优惠，活动仅限本月，机会难得，快来抢购吧！"
                    .to_string(),
                type_: "info".to_string(),
                priority: "medium".to_string(),
                status: "published".to_string(),
                author: "市场部".to_string(),
                target_audience: "vip".to_string(),
                start_date: Some("2024-01-01 00:00:00".to_string()),
                end_date: Some("2024-01-31 23:59:59".to_string()),
                views: 567,
                created_at: "2024-01-01 08:00:00".to_string(),
                updated_at: "2024-01-01 08:00:00".to_string(),
            },
            Announcement {
                id: 4,
                title: "新用户注册指南".to_string(),
                content: "欢迎新用户加入我们的平台，这里有详细的注册和使用指南。".to_string(),
                type_: "info".to_string(),
                priority: "low".to_string(),
                status: "draft".to_string(),
                author: "客服团队".to_string(),
                target_audience: "new_users".to_string(),
                start_date: None,
                end_date: None,
                views: 0,
                created_at: "2024-01-16 14:20:00".to_string(),
                updated_at: "2024-01-16 14:20:00".to_string(),
            },
        ]
    }
}

#[component]
pub fn AnnouncementsManagementPage() -> impl IntoView {
    let announcements = create_rw_signal(Announcement::mock_data());

    let stats = create_memo(move |_| {
        let announcements_data = announcements.get();
        let total_announcements = announcements_data.len();
        let published_announcements = announcements_data
            .iter()
            .filter(|a| a.status == "published")
            .count();
        let draft_announcements = announcements_data
            .iter()
            .filter(|a| a.status == "draft")
            .count();
        let total_views = announcements_data.iter().map(|a| a.views).sum::<u32>();

        (
            total_announcements,
            published_announcements,
            draft_announcements,
            total_views,
        )
    });

    let render_announcement_row = Box::new(|announcement: &Announcement| {
        let type_variant = match announcement.type_.as_str() {
            "info" => "info",
            "warning" => "warning",
            "error" => "error",
            "success" => "success",
            _ => "info",
        };

        let priority_variant = match announcement.priority.as_str() {
            "high" => "error",
            "medium" => "warning",
            "low" => "info",
            _ => "info",
        };

        let status_variant = match announcement.status.as_str() {
            "published" => "success",
            "draft" => "warning",
            "archived" => "info",
            _ => "info",
        };

        let target_text = match announcement.target_audience.as_str() {
            "all" => "所有用户",
            "vip" => "VIP用户",
            "new_users" => "新用户",
            _ => "未知",
        };

        view! {
            <td>
                <div class="announcement-info">
                    <div class="announcement-title">{announcement.title.clone()}</div>
                    <div class="announcement-content-preview">
                        {announcement.content.chars().take(50).collect::<String>()}
                        {if announcement.content.len() > 50 { "..." } else { "" }}
                    </div>
                </div>
            </td>
            <td>
                <StatusBadge
                    status=match announcement.type_.as_str() {
                        "info" => "信息",
                        "warning" => "警告",
                        "error" => "错误",
                        "success" => "成功",
                        _ => "未知"
                    }.to_string()
                    variant=type_variant.to_string()
                />
            </td>
            <td>
                <StatusBadge
                    status=match announcement.priority.as_str() {
                        "high" => "高",
                        "medium" => "中",
                        "low" => "低",
                        _ => "未知"
                    }.to_string()
                    variant=priority_variant.to_string()
                />
            </td>
            <td>
                <StatusBadge
                    status=match announcement.status.as_str() {
                        "published" => "已发布",
                        "draft" => "草稿",
                        "archived" => "已归档",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>{target_text}</td>
            <td>{announcement.author.clone()}</td>
            <td>
                <div class="date-range">
                    <div class="start-date">
                        {announcement.start_date.clone().unwrap_or_else(|| "-".to_string())}
                    </div>
                    {announcement.end_date.clone().map(|end| {
                        view! {
                            <div class="end-date">
                                {format!("至 {end}")}
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                </div>
            </td>
            <td class="views-cell">
                <span class="views-count">{announcement.views}</span>
            </td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加公告".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑公告".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        announcements.update(|announcements| {
            announcements.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="公告管理".to_string() subtitle="管理系统公告和通知".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总公告数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="📢".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="已发布".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="草稿".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="📝".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总浏览量".to_string()
                    value=Signal::derive(move || stats.get().3.to_string())
                    icon="👁️".to_string()
                    color="purple".to_string()
                    change=Some("+25.6%".to_string())
                    change_type=Some("up".to_string())
                />
            </div>

            // 公告列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "公告内容".to_string(),
                        "类型".to_string(),
                        "优先级".to_string(),
                        "状态".to_string(),
                        "目标用户".to_string(),
                        "作者".to_string(),
                        "有效期".to_string(),
                        "浏览量".to_string(),
                    ]
                    data=announcements.read_only()
                    render_row=render_announcement_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
