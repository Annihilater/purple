use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Order {
    pub id: i32,
    pub invite_user_id: Option<i32>,
    pub user_id: i32,
    pub plan_id: i32,
    pub coupon_id: Option<i32>,
    pub payment_id: Option<i32>,
    pub r#type: i32,
    pub period: String,
    pub trade_no: String,
    pub callback_no: Option<String>,
    pub total_amount: i32,
    pub handling_amount: Option<i32>,
    pub discount_amount: Option<i32>,
    pub surplus_amount: Option<i32>,
    pub refund_amount: Option<i32>,
    pub balance_amount: Option<i32>,
    pub surplus_order_ids: Option<String>,
    pub status: bool,
    pub commission_status: bool,
    pub commission_balance: i32,
    pub actual_commission_balance: Option<i32>,
    pub paid_at: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub invite_user_id: Option<i32>,
    #[validate(range(min = 1))]
    pub user_id: i32,
    #[validate(range(min = 1))]
    pub plan_id: i32,
    pub coupon_id: Option<i32>,
    pub payment_id: Option<i32>,
    #[validate(range(min = 1, max = 3))]
    pub r#type: i32,
    #[validate(length(min = 1, max = 255))]
    pub period: String,
    #[validate(length(min = 1, max = 36))]
    pub trade_no: String,
    #[validate(length(max = 255))]
    pub callback_no: Option<String>,
    #[validate(range(min = 0))]
    pub total_amount: i32,
    #[validate(range(min = 0))]
    pub handling_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub discount_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub surplus_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub refund_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub balance_amount: Option<i32>,
    pub surplus_order_ids: Option<String>,
    pub status: Option<bool>,
    pub commission_status: Option<bool>,
    #[validate(range(min = 0))]
    pub commission_balance: Option<i32>,
    #[validate(range(min = 0))]
    pub actual_commission_balance: Option<i32>,
    pub paid_at: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    pub invite_user_id: Option<i32>,
    #[validate(range(min = 1))]
    pub user_id: Option<i32>,
    #[validate(range(min = 1))]
    pub plan_id: Option<i32>,
    pub coupon_id: Option<i32>,
    pub payment_id: Option<i32>,
    #[validate(range(min = 1, max = 3))]
    pub r#type: Option<i32>,
    #[validate(length(min = 1, max = 255))]
    pub period: Option<String>,
    #[validate(length(min = 1, max = 36))]
    pub trade_no: Option<String>,
    #[validate(length(max = 255))]
    pub callback_no: Option<String>,
    #[validate(range(min = 0))]
    pub total_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub handling_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub discount_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub surplus_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub refund_amount: Option<i32>,
    #[validate(range(min = 0))]
    pub balance_amount: Option<i32>,
    pub surplus_order_ids: Option<String>,
    pub status: Option<bool>,
    pub commission_status: Option<bool>,
    #[validate(range(min = 0))]
    pub commission_balance: Option<i32>,
    #[validate(range(min = 0))]
    pub actual_commission_balance: Option<i32>,
    pub paid_at: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderResponse {
    pub id: i32,
    pub invite_user_id: Option<i32>,
    pub user_id: i32,
    pub plan_id: i32,
    pub coupon_id: Option<i32>,
    pub payment_id: Option<i32>,
    pub r#type: i32,
    pub type_text: String,
    pub period: String,
    pub trade_no: String,
    pub callback_no: Option<String>,
    pub total_amount: i32,
    pub handling_amount: Option<i32>,
    pub discount_amount: Option<i32>,
    pub surplus_amount: Option<i32>,
    pub refund_amount: Option<i32>,
    pub balance_amount: Option<i32>,
    pub surplus_order_ids: Option<String>,
    pub status: bool,
    pub status_text: String,
    pub commission_status: bool,
    pub commission_balance: i32,
    pub actual_commission_balance: Option<i32>,
    pub paid_at: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        let type_text = match order.r#type {
            1 => "新购".to_string(),
            2 => "续费".to_string(),
            3 => "升级".to_string(),
            _ => "未知".to_string(),
        };

        let status_text = if order.status {
            "已完成".to_string()
        } else {
            "待支付".to_string()
        };

        Self {
            id: order.id,
            invite_user_id: order.invite_user_id,
            user_id: order.user_id,
            plan_id: order.plan_id,
            coupon_id: order.coupon_id,
            payment_id: order.payment_id,
            r#type: order.r#type,
            type_text,
            period: order.period,
            trade_no: order.trade_no,
            callback_no: order.callback_no,
            total_amount: order.total_amount,
            handling_amount: order.handling_amount,
            discount_amount: order.discount_amount,
            surplus_amount: order.surplus_amount,
            refund_amount: order.refund_amount,
            balance_amount: order.balance_amount,
            surplus_order_ids: order.surplus_order_ids,
            status: order.status,
            status_text,
            commission_status: order.commission_status,
            commission_balance: order.commission_balance,
            actual_commission_balance: order.actual_commission_balance,
            paid_at: order.paid_at,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderListResponse {
    pub orders: Vec<OrderResponse>,
    pub total: i64,
}

/// 支付订单请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PayOrderRequest {
    #[validate(range(min = 1))]
    pub payment_id: i32,
}

/// 取消订单请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CancelOrderRequest {
    #[serde(default)]
    pub reason: Option<String>,
}

/// 简化的创建订单请求（用户端）
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserOrderRequest {
    #[validate(range(min = 1))]
    pub plan_id: i32,
    pub coupon_id: Option<i32>,
    #[validate(range(min = 1, max = 3))]
    /// 订单类型：1新购 2续费 3升级
    pub r#type: i32,
    #[validate(length(min = 1, max = 255))]
    /// 订购周期：month_price, quarter_price, half_year_price, year_price, two_year_price, three_year_price, onetime_price
    pub period: String,
    /// 使用余额支付金额
    pub balance_amount: Option<i32>,
}
