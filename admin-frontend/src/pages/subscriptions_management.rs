use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: u32,
    pub user_id: u32,
    pub user_name: String,
    pub user_email: String,
    pub plan_name: String,
    pub status: String, // active, expired, cancelled, pending
    pub price: f64,
    pub currency: String,
    pub start_date: String,
    pub expire_date: String,
    pub auto_renew: bool,
    pub traffic_used: String,
    pub traffic_total: String,
    pub created_at: String,
}

impl Subscription {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Subscription {
                id: 1,
                user_id: 1001,
                user_name: "张三".to_string(),
                user_email: "zhangsan@example.com".to_string(),
                plan_name: "高级套餐".to_string(),
                status: "active".to_string(),
                price: 99.0,
                currency: "CNY".to_string(),
                start_date: "2024-01-01".to_string(),
                expire_date: "2024-02-01".to_string(),
                auto_renew: true,
                traffic_used: "15.6 GB".to_string(),
                traffic_total: "100 GB".to_string(),
                created_at: "2024-01-01".to_string(),
            },
            Subscription {
                id: 2,
                user_id: 1002,
                user_name: "李四".to_string(),
                user_email: "lisi@example.com".to_string(),
                plan_name: "标准套餐".to_string(),
                status: "active".to_string(),
                price: 49.0,
                currency: "CNY".to_string(),
                start_date: "2024-01-05".to_string(),
                expire_date: "2024-02-05".to_string(),
                auto_renew: false,
                traffic_used: "28.3 GB".to_string(),
                traffic_total: "50 GB".to_string(),
                created_at: "2024-01-05".to_string(),
            },
            Subscription {
                id: 3,
                user_id: 1003,
                user_name: "王五".to_string(),
                user_email: "wangwu@example.com".to_string(),
                plan_name: "基础套餐".to_string(),
                status: "expired".to_string(),
                price: 19.0,
                currency: "CNY".to_string(),
                start_date: "2023-12-01".to_string(),
                expire_date: "2024-01-01".to_string(),
                auto_renew: false,
                traffic_used: "18.7 GB".to_string(),
                traffic_total: "20 GB".to_string(),
                created_at: "2023-12-01".to_string(),
            },
        ]
    }
}

#[component]
pub fn SubscriptionsManagementPage() -> impl IntoView {
    let subscriptions = create_rw_signal(Subscription::mock_data());

    let stats = create_memo(move |_| {
        let subs_data = subscriptions.get();
        let total_subs = subs_data.len();
        let active_subs = subs_data.iter().filter(|s| s.status == "active").count();
        let expired_subs = subs_data.iter().filter(|s| s.status == "expired").count();
        let total_revenue = subs_data
            .iter()
            .filter(|s| s.status == "active")
            .map(|s| s.price)
            .sum::<f64>();

        (total_subs, active_subs, expired_subs, total_revenue)
    });

    let render_subscription_row = Box::new(|sub: &Subscription| {
        let status_variant = match sub.status.as_str() {
            "active" => "success",
            "expired" => "error",
            "cancelled" => "warning",
            "pending" => "info",
            _ => "info",
        };

        let traffic_percentage =
            if sub.traffic_total.contains("GB") && sub.traffic_used.contains("GB") {
                let used: f64 = sub.traffic_used.replace(" GB", "").parse().unwrap_or(0.0);
                let total: f64 = sub.traffic_total.replace(" GB", "").parse().unwrap_or(1.0);
                (used / total * 100.0).min(100.0)
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
                <div class="user-info">
                    <div class="user-name">{sub.user_name.clone()}</div>
                    <div class="user-email">{sub.user_email.clone()}</div>
                </div>
            </td>
            <td>{sub.plan_name.clone()}</td>
            <td>
                <StatusBadge
                    status=match sub.status.as_str() {
                        "active" => "有效",
                        "expired" => "已过期",
                        "cancelled" => "已取消",
                        "pending" => "待激活",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td class="price-cell">
                <span class="price">{format!("¥{:.2}", sub.price)}</span>
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
                        {format!("{} / {}", sub.traffic_used, sub.traffic_total)}
                    </div>
                </div>
            </td>
            <td>{sub.expire_date.clone()}</td>
            <td>
                <span class=format!("auto-renew-badge {}", if sub.auto_renew { "enabled" } else { "disabled" })>
                    {if sub.auto_renew { "已开启" } else { "已关闭" }}
                </span>
            </td>
        }.into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加订阅".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑订阅".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        subscriptions.update(|subs| {
            subs.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="订阅管理".to_string() subtitle="管理用户订阅和套餐状态".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总订阅数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="📋".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="活跃订阅".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="过期订阅".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="❌".to_string()
                    color="red".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="月收入".to_string()
                    value=Signal::derive(move || format!("¥{:.2}", stats.get().3))
                    icon="💰".to_string()
                    color="purple".to_string()
                    change=Some("+12.5%".to_string())
                    change_type=Some("up".to_string())
                />
            </div>

            // 订阅列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "用户".to_string(),
                        "套餐".to_string(),
                        "状态".to_string(),
                        "价格".to_string(),
                        "流量使用".to_string(),
                        "到期时间".to_string(),
                        "自动续费".to_string(),
                    ]
                    data=subscriptions.read_only()
                    render_row=render_subscription_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
