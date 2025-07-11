mod admin;
mod auth;
mod coupon;
mod health;
mod info;
// mod notice;    // 未实现仓库层，暂时注释
// mod order;     // 未实现仓库层，暂时注释
pub mod openapi;
mod plan;
pub mod server;
mod subscribe;
// mod ticket;    // 未实现仓库层，暂时注释
pub mod user;

pub use admin::{get_admin_dashboard, get_admin_stats};
pub use auth::{login, register};
pub use coupon::{
    create_coupon, delete_coupon, get_coupon, list_coupons, update_coupon, verify_coupon,
};
pub use health::health_check;
pub use info::get_project_info;
// 未实现的模块暂时注释导出
// pub use notice::{
//     create_notice, delete_notice, get_latest_notices, get_notice, get_notices, update_notice,
// };
// pub use order::{
//     cancel_order, create_order, delete_order, get_order, get_order_stats, get_orders, pay_order,
// };
pub use plan::{
    batch_update_plan_status, check_plan_availability, create_plan, delete_plan, get_enabled_plans,
    get_plan, get_plan_pricing, get_plan_stats, list_plans, update_plan,
};
pub use server::{
    copy_server, create_server, create_server_group, create_server_route, delete_server,
    delete_server_group, delete_server_route, get_server, get_server_groups, get_server_routes,
    get_servers, get_user_servers, sort_servers, update_server, update_server_group,
    update_server_route,
};
pub use subscribe::{
    get_nodes_status, get_subscribe_config, get_subscribe_info, get_subscribe_link,
    get_subscribe_stats, report_traffic, reset_subscribe_token, test_subscribe_connectivity,
};
// pub use ticket::{
//     create_ticket, delete_ticket, get_ticket, get_ticket_stats, get_tickets, reply_ticket,
//     update_ticket_status,
// };
pub use user::*;
