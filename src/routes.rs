use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api, api::openapi::ApiDoc};

/// 配置应用路由
///
/// 将所有API路由注册到应用实例中，
/// 包括OpenAPI文档、Swagger UI和各种业务API
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // 项目信息根路径端点
        .service(api::get_project_info)
        // OpenAPI JSON 端点
        .route("/api-docs/openapi.json", web::get().to(serve_openapi_spec))
        // Swagger UI
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        // 健康检查端点
        .service(api::health_check)
        // 认证相关路由
        .configure(configure_auth_routes)
        // 用户管理路由
        .configure(configure_user_routes)
        // 套餐管理路由
        .configure(configure_plan_routes)
        // 优惠券管理路由
        .configure(configure_coupon_routes)
        // 订阅管理路由
        .configure(configure_subscribe_routes);
}

/// 配置认证相关路由
fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api::register).service(api::login);
}

/// 配置用户管理路由
fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .service(api::create_user)
            .service(api::get_users)
            .service(api::get_user)
            .service(api::update_user)
            .service(api::delete_user)
            .service(api::update_user_status),
    );
}

/// 配置套餐管理路由
fn configure_plan_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/plans")
            .service(api::create_plan)
            .service(api::list_plans)
            .service(api::get_plan)
            .service(api::update_plan)
            .service(api::delete_plan)
            .service(api::get_enabled_plans)
            .service(api::get_plan_stats)
            .service(api::get_plan_pricing)
            .service(api::check_plan_availability)
            .service(api::batch_update_plan_status),
    );
}

/// 配置优惠券管理路由
fn configure_coupon_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/coupons")
            .service(api::create_coupon)
            .service(api::list_coupons)
            .service(api::get_coupon)
            .service(api::update_coupon)
            .service(api::delete_coupon)
            .service(api::verify_coupon),
    );
}

/// 配置订阅管理路由
fn configure_subscribe_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // 用户订阅接口
        .service(api::get_subscribe_info)
        .service(api::get_subscribe_link)
        .service(api::reset_subscribe_token)
        .service(api::get_subscribe_stats)
        .service(api::get_nodes_status)
        .service(api::test_subscribe_connectivity)
        // 客户端配置获取接口
        .service(api::get_subscribe_config)
        // 节点流量上报接口
        .service(api::report_traffic);
}

/// 提供OpenAPI规范
async fn serve_openapi_spec() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .json(ApiDoc::openapi())
}
