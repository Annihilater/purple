mod auth;
mod coupon;
mod health;
mod info;
mod notice;
pub mod openapi;
mod order;
mod plan;
mod subscribe;
mod ticket;
pub mod user;

pub use auth::{login, register};
pub use coupon::{
    create_coupon, delete_coupon, get_coupon, list_coupons, update_coupon, verify_coupon,
};
pub use health::health_check;
pub use info::get_project_info;
pub use notice::{
    create_notice, delete_notice, get_latest_notices, get_notice, get_notices, update_notice,
};
pub use openapi::*;
pub use order::{
    cancel_order, create_order, delete_order, get_order, get_order_stats, get_orders, pay_order,
};
pub use plan::{
    batch_update_plan_status, check_plan_availability, create_plan, delete_plan, get_enabled_plans,
    get_plan, get_plan_pricing, get_plan_stats, list_plans, update_plan,
};
pub use subscribe::{
    get_nodes_status, get_subscribe_config, get_subscribe_info, get_subscribe_link,
    get_subscribe_stats, report_traffic, reset_subscribe_token, test_subscribe_connectivity,
};
pub use ticket::{
    create_ticket, delete_ticket, get_ticket, get_ticket_stats, get_tickets, reply_ticket,
    update_ticket_status,
};
pub use user::*;
