use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: String, // active, banned, inactive
    pub role: String,   // admin, user, vip
    pub balance: f64,
    pub referral_code: String,
    pub referred_by: Option<String>,
    pub total_traffic: String,
    pub used_traffic: String,
    pub last_login: Option<String>,
    pub created_at: String,
    pub subscription_status: Option<String>,
}

impl User {
    pub fn mock_data() -> Vec<Self> {
        vec![
            User {
                id: 1001,
                username: "张三".to_string(),
                email: "zhangsan@example.com".to_string(),
                phone: Some("13800138000".to_string()),
                avatar: Some("/avatars/zhangsan.png".to_string()),
                status: "active".to_string(),
                role: "vip".to_string(),
                balance: 156.50,
                referral_code: "ZS001".to_string(),
                referred_by: None,
                total_traffic: "100 GB".to_string(),
                used_traffic: "15.6 GB".to_string(),
                last_login: Some("2024-01-15 10:30:00".to_string()),
                created_at: "2024-01-01 09:00:00".to_string(),
                subscription_status: Some("active".to_string()),
            },
            User {
                id: 1002,
                username: "李四".to_string(),
                email: "lisi@example.com".to_string(),
                phone: Some("13800138001".to_string()),
                avatar: None,
                status: "active".to_string(),
                role: "user".to_string(),
                balance: 0.0,
                referral_code: "LS002".to_string(),
                referred_by: Some("ZS001".to_string()),
                total_traffic: "50 GB".to_string(),
                used_traffic: "28.3 GB".to_string(),
                last_login: Some("2024-01-14 20:15:00".to_string()),
                created_at: "2024-01-05 14:30:00".to_string(),
                subscription_status: Some("active".to_string()),
            },
            User {
                id: 1003,
                username: "王五".to_string(),
                email: "wangwu@example.com".to_string(),
                phone: None,
                avatar: None,
                status: "banned".to_string(),
                role: "user".to_string(),
                balance: 25.00,
                referral_code: "WW003".to_string(),
                referred_by: None,
                total_traffic: "20 GB".to_string(),
                used_traffic: "18.7 GB".to_string(),
                last_login: Some("2024-01-10 16:45:00".to_string()),
                created_at: "2023-12-01 11:20:00".to_string(),
                subscription_status: Some("expired".to_string()),
            },
            User {
                id: 1004,
                username: "赵六".to_string(),
                email: "zhaoliu@example.com".to_string(),
                phone: Some("13800138003".to_string()),
                avatar: None,
                status: "inactive".to_string(),
                role: "user".to_string(),
                balance: 0.0,
                referral_code: "ZL004".to_string(),
                referred_by: None,
                total_traffic: "0 GB".to_string(),
                used_traffic: "0 GB".to_string(),
                last_login: None,
                created_at: "2024-01-16 13:10:00".to_string(),
                subscription_status: None,
            },
        ]
    }
}

#[component]
pub fn UsersManagementPage() -> impl IntoView {
    let users = create_rw_signal(User::mock_data());

    let stats = create_memo(move |_| {
        let users_data = users.get();
        let total_users = users_data.len();
        let active_users = users_data.iter().filter(|u| u.status == "active").count();
        let banned_users = users_data.iter().filter(|u| u.status == "banned").count();
        let vip_users = users_data.iter().filter(|u| u.role == "vip").count();
        let total_balance = users_data.iter().map(|u| u.balance).sum::<f64>();

        (
            total_users,
            active_users,
            banned_users,
            vip_users,
            total_balance,
        )
    });

    let render_user_row = Box::new(|user: &User| {
        let status_variant = match user.status.as_str() {
            "active" => "success",
            "banned" => "error",
            "inactive" => "warning",
            _ => "info",
        };

        let role_variant = match user.role.as_str() {
            "admin" => "error",
            "vip" => "warning",
            "user" => "info",
            _ => "info",
        };

        let traffic_percentage =
            if user.total_traffic.contains("GB") && user.used_traffic.contains("GB") {
                let used: f64 = user.used_traffic.replace(" GB", "").parse().unwrap_or(0.0);
                let total: f64 = user.total_traffic.replace(" GB", "").parse().unwrap_or(1.0);
                if total > 0.0 {
                    (used / total * 100.0).min(100.0)
                } else {
                    0.0
                }
            } else {
                0.0
            };

        let traffic_color = if traffic_percentage < 50.0 {
            "success"
        } else if traffic_percentage < 80.0 {
            "warning"
        } else {
            "error"
        };

        view! {
            <td>
                <div class="user-profile">
                    <div class="user-avatar">
                        {user.avatar.clone().unwrap_or_else(|| "👤".to_string())}
                    </div>
                    <div class="user-details">
                        <div class="user-name">{user.username.clone()}</div>
                        <div class="user-email">{user.email.clone()}</div>
                        <div class="user-phone">{user.phone.clone().unwrap_or_else(|| "-".to_string())}</div>
                    </div>
                </div>
            </td>
            <td>
                <StatusBadge
                    status=match user.status.as_str() {
                        "active" => "正常",
                        "banned" => "已封禁",
                        "inactive" => "未激活",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                <StatusBadge
                    status=match user.role.as_str() {
                        "admin" => "管理员",
                        "vip" => "VIP用户",
                        "user" => "普通用户",
                        _ => "未知"
                    }.to_string()
                    variant=role_variant.to_string()
                />
            </td>
            <td class="balance-cell">
                <span class="balance">{format!("¥{:.2}", user.balance)}</span>
            </td>
            <td>
                <div class="traffic-info">
                    <div class="traffic-bar">
                        <div
                            class=format!("traffic-fill traffic-{}", traffic_color)
                            style=format!("width: {}%", traffic_percentage as i32)
                        ></div>
                    </div>
                    <div class="traffic-text">
                        {format!("{} / {}", user.used_traffic, user.total_traffic)}
                    </div>
                </div>
            </td>
            <td>
                <div class="subscription-info">
                    {user.subscription_status.clone().map(|status| {
                        let variant = match status.as_str() {
                            "active" => "success",
                            "expired" => "error",
                            _ => "info",
                        };
                        view! {
                            <StatusBadge
                                status=match status.as_str() {
                                    "active" => "有效",
                                    "expired" => "已过期",
                                    _ => "无"
                                }.to_string()
                                variant=variant.to_string()
                            />
                        }
                    }).unwrap_or_else(|| {
                        view! {
                            <span class="no-subscription">"-"</span>
                        }.into_view()
                    })}
                </div>
            </td>
            <td>
                <div class="referral-info">
                    <div class="referral-code">{user.referral_code.clone()}</div>
                    {user.referred_by.clone().map(|referred| {
                        view! {
                            <div class="referred-by">
                                {format!("由 {referred} 推荐")}
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! {}.into_view())}
                </div>
            </td>
            <td>
                <div class="login-info">
                    {user.last_login.clone().unwrap_or_else(|| "从未登录".to_string())}
                </div>
            </td>
        }.into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加用户".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑用户".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        users.update(|users| {
            users.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="用户管理".to_string() subtitle="管理用户账户和权限".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总用户数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="👥".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="活跃用户".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="被封禁用户".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="🚫".to_string()
                    color="red".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="VIP用户".to_string()
                    value=Signal::derive(move || stats.get().3.to_string())
                    icon="👑".to_string()
                    color="purple".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 用户列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "用户信息".to_string(),
                        "状态".to_string(),
                        "角色".to_string(),
                        "余额".to_string(),
                        "流量使用".to_string(),
                        "订阅状态".to_string(),
                        "推荐信息".to_string(),
                        "最后登录".to_string(),
                    ]
                    data=users.read_only()
                    render_row=render_user_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
