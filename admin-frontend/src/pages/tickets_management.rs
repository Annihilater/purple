use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub user_id: u32,
    pub user_name: String,
    pub user_email: String,
    pub category: String, // technical, billing, general
    pub priority: String, // low, medium, high, urgent
    pub status: String,   // open, in_progress, resolved, closed
    pub assigned_to: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub resolved_at: Option<String>,
    pub replies_count: u32,
}

impl Ticket {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Ticket {
                id: 1,
                title: "无法连接到香港节点".to_string(),
                description: "我尝试连接香港节点，但总是显示连接超时，请帮助解决。".to_string(),
                user_id: 1001,
                user_name: "张三".to_string(),
                user_email: "zhangsan@example.com".to_string(),
                category: "technical".to_string(),
                priority: "high".to_string(),
                status: "in_progress".to_string(),
                assigned_to: Some("技术支持A".to_string()),
                created_at: "2024-01-15 10:30:00".to_string(),
                updated_at: "2024-01-15 14:20:00".to_string(),
                resolved_at: None,
                replies_count: 3,
            },
            Ticket {
                id: 2,
                title: "账单问题咨询".to_string(),
                description: "我的账单显示有重复扣费，希望能够退款。".to_string(),
                user_id: 1002,
                user_name: "李四".to_string(),
                user_email: "lisi@example.com".to_string(),
                category: "billing".to_string(),
                priority: "medium".to_string(),
                status: "open".to_string(),
                assigned_to: None,
                created_at: "2024-01-14 16:45:00".to_string(),
                updated_at: "2024-01-14 16:45:00".to_string(),
                resolved_at: None,
                replies_count: 0,
            },
            Ticket {
                id: 3,
                title: "如何更改密码".to_string(),
                description: "我忘记了密码，需要重置，但没有收到重置邮件。".to_string(),
                user_id: 1003,
                user_name: "王五".to_string(),
                user_email: "wangwu@example.com".to_string(),
                category: "general".to_string(),
                priority: "low".to_string(),
                status: "resolved".to_string(),
                assigned_to: Some("客服B".to_string()),
                created_at: "2024-01-13 09:15:00".to_string(),
                updated_at: "2024-01-13 15:30:00".to_string(),
                resolved_at: Some("2024-01-13 15:30:00".to_string()),
                replies_count: 2,
            },
            Ticket {
                id: 4,
                title: "服务器速度慢".to_string(),
                description: "最近几天服务器速度很慢，影响使用体验。".to_string(),
                user_id: 1004,
                user_name: "赵六".to_string(),
                user_email: "zhaoliu@example.com".to_string(),
                category: "technical".to_string(),
                priority: "urgent".to_string(),
                status: "open".to_string(),
                assigned_to: None,
                created_at: "2024-01-16 11:00:00".to_string(),
                updated_at: "2024-01-16 11:00:00".to_string(),
                resolved_at: None,
                replies_count: 0,
            },
        ]
    }
}

#[component]
pub fn TicketsManagementPage() -> impl IntoView {
    let tickets = create_rw_signal(Ticket::mock_data());

    let stats = create_memo(move |_| {
        let tickets_data = tickets.get();
        let total_tickets = tickets_data.len();
        let open_tickets = tickets_data.iter().filter(|t| t.status == "open").count();
        let in_progress_tickets = tickets_data
            .iter()
            .filter(|t| t.status == "in_progress")
            .count();
        let resolved_tickets = tickets_data
            .iter()
            .filter(|t| t.status == "resolved")
            .count();
        let urgent_tickets = tickets_data
            .iter()
            .filter(|t| t.priority == "urgent")
            .count();

        (
            total_tickets,
            open_tickets,
            in_progress_tickets,
            resolved_tickets,
            urgent_tickets,
        )
    });

    let render_ticket_row = Box::new(|ticket: &Ticket| {
        let category_variant = match ticket.category.as_str() {
            "technical" => "error",
            "billing" => "warning",
            "general" => "info",
            _ => "info",
        };

        let priority_variant = match ticket.priority.as_str() {
            "urgent" => "error",
            "high" => "warning",
            "medium" => "info",
            "low" => "success",
            _ => "info",
        };

        let status_variant = match ticket.status.as_str() {
            "open" => "warning",
            "in_progress" => "info",
            "resolved" => "success",
            "closed" => "info",
            _ => "info",
        };

        view! {
            <td>
                <div class="ticket-info">
                    <div class="ticket-title">{ticket.title.clone()}</div>
                    <div class="ticket-description">
                        {ticket.description.chars().take(60).collect::<String>()}
                        {if ticket.description.len() > 60 { "..." } else { "" }}
                    </div>
                </div>
            </td>
            <td>
                <div class="user-info">
                    <div class="user-name">{ticket.user_name.clone()}</div>
                    <div class="user-email">{ticket.user_email.clone()}</div>
                </div>
            </td>
            <td>
                <StatusBadge
                    status=match ticket.category.as_str() {
                        "technical" => "技术",
                        "billing" => "账单",
                        "general" => "一般",
                        _ => "未知"
                    }.to_string()
                    variant=category_variant.to_string()
                />
            </td>
            <td>
                <StatusBadge
                    status=match ticket.priority.as_str() {
                        "urgent" => "紧急",
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
                    status=match ticket.status.as_str() {
                        "open" => "待处理",
                        "in_progress" => "处理中",
                        "resolved" => "已解决",
                        "closed" => "已关闭",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                <div class="assigned-to">
                    {ticket.assigned_to.clone().unwrap_or_else(|| "未分配".to_string())}
                </div>
            </td>
            <td class="replies-cell">
                <span class="replies-count">{ticket.replies_count}</span>
            </td>
            <td>
                <div class="date-info">
                    <div class="created-date">{ticket.created_at.clone()}</div>
                    {ticket.resolved_at.clone().map(|resolved| {
                        view! {
                            <div class="resolved-date">
                                {format!("已解决: {resolved}")}
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                </div>
            </td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加工单".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑工单".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        tickets.update(|tickets| {
            tickets.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="工单管理".to_string() subtitle="管理用户工单和客服支持".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总工单数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="🎫".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="待处理".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="⏳".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="处理中".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="🔄".to_string()
                    color="info".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="已解决".to_string()
                    value=Signal::derive(move || stats.get().3.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 工单列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "工单内容".to_string(),
                        "用户".to_string(),
                        "类别".to_string(),
                        "优先级".to_string(),
                        "状态".to_string(),
                        "分配给".to_string(),
                        "回复数".to_string(),
                        "时间".to_string(),
                    ]
                    data=tickets.read_only()
                    render_row=render_ticket_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
