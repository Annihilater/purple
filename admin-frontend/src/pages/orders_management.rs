use crate::components::common::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u32,
    pub order_no: String,
    pub user_id: u32,
    pub user_name: String,
    pub user_email: String,
    pub plan_name: String,
    pub amount: f64,
    pub currency: String,
    pub status: String, // pending, paid, cancelled, refunded
    pub payment_method: String,
    pub created_at: String,
    pub paid_at: Option<String>,
    pub discount_amount: f64,
    pub coupon_code: Option<String>,
}

impl Order {
    pub fn mock_data() -> Vec<Self> {
        vec![
            Order {
                id: 1,
                order_no: "ORD20240101001".to_string(),
                user_id: 1001,
                user_name: "张三".to_string(),
                user_email: "zhangsan@example.com".to_string(),
                plan_name: "高级套餐".to_string(),
                amount: 99.0,
                currency: "CNY".to_string(),
                status: "paid".to_string(),
                payment_method: "支付宝".to_string(),
                created_at: "2024-01-01 10:30:00".to_string(),
                paid_at: Some("2024-01-01 10:32:15".to_string()),
                discount_amount: 0.0,
                coupon_code: None,
            },
            Order {
                id: 2,
                order_no: "ORD20240102001".to_string(),
                user_id: 1002,
                user_name: "李四".to_string(),
                user_email: "lisi@example.com".to_string(),
                plan_name: "标准套餐".to_string(),
                amount: 49.0,
                currency: "CNY".to_string(),
                status: "paid".to_string(),
                payment_method: "微信支付".to_string(),
                created_at: "2024-01-02 14:15:00".to_string(),
                paid_at: Some("2024-01-02 14:16:30".to_string()),
                discount_amount: 10.0,
                coupon_code: Some("WELCOME10".to_string()),
            },
            Order {
                id: 3,
                order_no: "ORD20240103001".to_string(),
                user_id: 1003,
                user_name: "王五".to_string(),
                user_email: "wangwu@example.com".to_string(),
                plan_name: "基础套餐".to_string(),
                amount: 19.0,
                currency: "CNY".to_string(),
                status: "pending".to_string(),
                payment_method: "支付宝".to_string(),
                created_at: "2024-01-03 09:20:00".to_string(),
                paid_at: None,
                discount_amount: 0.0,
                coupon_code: None,
            },
            Order {
                id: 4,
                order_no: "ORD20240104001".to_string(),
                user_id: 1004,
                user_name: "赵六".to_string(),
                user_email: "zhaoliu@example.com".to_string(),
                plan_name: "高级套餐".to_string(),
                amount: 99.0,
                currency: "CNY".to_string(),
                status: "cancelled".to_string(),
                payment_method: "Stripe".to_string(),
                created_at: "2024-01-04 16:45:00".to_string(),
                paid_at: None,
                discount_amount: 0.0,
                coupon_code: None,
            },
        ]
    }
}

#[component]
pub fn OrdersManagementPage() -> impl IntoView {
    let orders = create_rw_signal(Order::mock_data());

    let stats = create_memo(move |_| {
        let orders_data = orders.get();
        let total_orders = orders_data.len();
        let paid_orders = orders_data.iter().filter(|o| o.status == "paid").count();
        let pending_orders = orders_data.iter().filter(|o| o.status == "pending").count();
        let total_revenue = orders_data
            .iter()
            .filter(|o| o.status == "paid")
            .map(|o| o.amount)
            .sum::<f64>();

        (total_orders, paid_orders, pending_orders, total_revenue)
    });

    let render_order_row = Box::new(|order: &Order| {
        let status_variant = match order.status.as_str() {
            "paid" => "success",
            "pending" => "warning",
            "cancelled" => "error",
            "refunded" => "info",
            _ => "info",
        };

        let payment_method_icon = match order.payment_method.as_str() {
            "支付宝" => "💳",
            "微信支付" => "💚",
            "Stripe" => "🔵",
            _ => "💰",
        };

        view! {
            <td>
                <div class="order-info">
                    <div class="order-no">{order.order_no.clone()}</div>
                    <div class="order-time">{order.created_at.clone()}</div>
                </div>
            </td>
            <td>
                <div class="user-info">
                    <div class="user-name">{order.user_name.clone()}</div>
                    <div class="user-email">{order.user_email.clone()}</div>
                </div>
            </td>
            <td>{order.plan_name.clone()}</td>
            <td>
                <div class="amount-info">
                    <div class="amount">{format!("¥{:.2}", order.amount)}</div>
                    {if order.discount_amount > 0.0 {
                        view! {
                            <div class="discount">
                                {format!("-¥{:.2}", order.discount_amount)}
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </td>
            <td>
                <StatusBadge
                    status=match order.status.as_str() {
                        "paid" => "已支付",
                        "pending" => "待支付",
                        "cancelled" => "已取消",
                        "refunded" => "已退款",
                        _ => "未知"
                    }.to_string()
                    variant=status_variant.to_string()
                />
            </td>
            <td>
                <div class="payment-method">
                    <span class="payment-icon">{payment_method_icon}</span>
                    <span class="payment-text">{order.payment_method.clone()}</span>
                </div>
            </td>
            <td>
                {order.paid_at.clone().unwrap_or_else(|| "-".to_string())}
            </td>
        }
        .into_view()
    });

    let on_add = Some(Rc::new(|| {
        web_sys::console::log_1(&"添加订单".into());
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(|index: usize| {
        web_sys::console::log_2(&"编辑订单".into(), &index.to_string().into());
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        orders.update(|orders| {
            orders.remove(index);
        });
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="订单管理".to_string() subtitle="管理用户订单和支付状态".to_string()>
            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总订单数".to_string()
                    value=Signal::derive(move || stats.get().0.to_string())
                    icon="🧾".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="已支付".to_string()
                    value=Signal::derive(move || stats.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="待支付".to_string()
                    value=Signal::derive(move || stats.get().2.to_string())
                    icon="⏳".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总收入".to_string()
                    value=Signal::derive(move || format!("¥{:.2}", stats.get().3))
                    icon="💰".to_string()
                    color="purple".to_string()
                    change=Some("+18.3%".to_string())
                    change_type=Some("up".to_string())
                />
            </div>

            // 订单列表
            <div class="content-card">
                <DataTable
                    headers=vec![
                        "订单号".to_string(),
                        "用户".to_string(),
                        "套餐".to_string(),
                        "金额".to_string(),
                        "状态".to_string(),
                        "支付方式".to_string(),
                        "支付时间".to_string(),
                    ]
                    data=orders.read_only()
                    render_row=render_order_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>
        </PageTemplate>
    }
}
