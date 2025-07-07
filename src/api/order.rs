use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::order::{
        CancelOrderRequest, CreateUserOrderRequest, Order, OrderListResponse, OrderResponse,
        PayOrderRequest,
    },
    // repositories::OrderRepository,
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetOrdersQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    /// 订单状态：0待支付 1已完成
    pub status: Option<bool>,
    /// 订单类型：1新购 2续费 3升级
    pub r#type: Option<i32>,
    /// 用户ID（管理员查询用）
    pub user_id: Option<i32>,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

/// 创建订单（用户端）
#[utoipa::path(
    post,
    path = "/api/orders",
    tag = "orders",
    request_body = CreateUserOrderRequest,
    responses(
        (status = 200, description = "创建订单成功"),
        (status = 400, description = "请求参数无效"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/orders")]
pub async fn create_order(
    order: web::Json<CreateUserOrderRequest>,
    // repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = order.validate() {
        return Err(ApiError::from(validation_errors));
    }

    // TODO: 实现订单创建逻辑
    // 1. 验证套餐存在
    // 2. 验证优惠券（如果有）
    // 3. 计算价格
    // 4. 生成交易号
    // 5. 创建订单

    // 临时返回错误，等实现仓库方法
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 获取订单列表
#[utoipa::path(
    get,
    path = "/api/orders",
    tag = "orders",
    params(
        GetOrdersQuery
    ),
    responses(
        (status = 200, description = "获取订单列表成功"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/orders")]
pub async fn get_orders(
    query: web::Query<GetOrdersQuery>,
    // repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 根据用户角色决定是否过滤用户ID
    let user_id = query.user_id; // 管理员可以查询指定用户的订单

    // match repo.find_all(
    //     query.page as i32,
    //     query.page_size as i32,
    //     query.status,
    //     query.r#type,
    //     user_id,
    // ).await {
    //     Ok((orders, total)) => {
    //         let orders = orders.into_iter().map(OrderResponse::from).collect();
    //         let response = ApiResponse::success(OrderListResponse { orders, total });
    //         Ok(response.into_http_response())
    //     }
    //     Err(e) => {
    //         tracing::error!("获取订单列表失败: {}", e);
    //         Err(ApiError::with_details(
    //             ErrorCode::DatabaseError,
    //             "数据库操作失败".to_string(),
    //         ))
    //     }
    // }
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 根据ID获取订单
#[utoipa::path(
    get,
    path = "/api/orders/{id}",
    tag = "orders",
    params(
        ("id" = i32, Path, description = "订单ID")
    ),
    responses(
        (status = 200, description = "获取订单成功"),
        (status = 404, description = "订单不存在"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/orders/{id}")]
pub async fn get_order(
    id: web::Path<i32>,
    // repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID，验证权限
) -> Result<HttpResponse, ApiError> {
    let order_id = id.into_inner();

    // match repo.find_by_id(order_id).await {
    //     Ok(Some(order)) => {
    //         // TODO: 验证用户权限（只能查看自己的订单，除非是管理员）
    //         let response = ApiResponse::success(OrderResponse::from(order));
    //         Ok(response.into_http_response())
    //     }
    //     Ok(None) => Err(ApiError::new(ErrorCode::OrderNotFound)),
    //     Err(e) => {
    //         tracing::error!("获取订单失败: {}", e);
    //         Err(ApiError::with_details(
    //             ErrorCode::DatabaseError,
    //             "数据库操作失败".to_string(),
    //         ))
    //     }
    // }
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 支付订单
#[utoipa::path(
    post,
    path = "/api/orders/{id}/pay",
    tag = "orders",
    params(
        ("id" = i32, Path, description = "订单ID")
    ),
    request_body = PayOrderRequest,
    responses(
        (status = 200, description = "支付订单成功"),
        (status = 400, description = "请求参数无效"),
        (status = 404, description = "订单不存在"),
        (status = 409, description = "订单状态不允许支付"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/orders/{id}/pay")]
pub async fn pay_order(
    id: web::Path<i32>,
    pay_request: web::Json<PayOrderRequest>,
    // repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = pay_request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    let order_id = id.into_inner();

    // TODO: 实现支付逻辑
    // 1. 验证订单存在且属于当前用户
    // 2. 验证订单状态（未支付）
    // 3. 验证支付方式
    // 4. 调用支付接口
    // 5. 更新订单状态

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 取消订单
#[utoipa::path(
    post,
    path = "/api/orders/{id}/cancel",
    tag = "orders",
    params(
        ("id" = i32, Path, description = "订单ID")
    ),
    request_body = CancelOrderRequest,
    responses(
        (status = 200, description = "取消订单成功"),
        (status = 404, description = "订单不存在"),
        (status = 409, description = "订单状态不允许取消"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/orders/{id}/cancel")]
pub async fn cancel_order(
    id: web::Path<i32>,
    cancel_request: web::Json<CancelOrderRequest>,
    // repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    let order_id = id.into_inner();

    // TODO: 实现取消订单逻辑
    // 1. 验证订单存在且属于当前用户
    // 2. 验证订单状态（待支付）
    // 3. 更新订单状态为已取消
    // 4. 记录取消原因

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 删除订单（管理员）
#[utoipa::path(
    delete,
    path = "/api/orders/{id}",
    tag = "orders",
    params(
        ("id" = i32, Path, description = "订单ID")
    ),
    responses(
        (status = 200, description = "删除订单成功"),
        (status = 404, description = "订单不存在"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[delete("/api/orders/{id}")]
pub async fn delete_order(
    id: web::Path<i32>,
    // repo: web::Data<OrderRepository>,
    // TODO: 验证管理员权限
) -> Result<HttpResponse, ApiError> {
    let order_id = id.into_inner();

    // match repo.delete(order_id).await {
    //     Ok(true) => {
    //         let response = ApiResponse::success(());
    //         Ok(response.into_http_response())
    //     }
    //     Ok(false) => Err(ApiError::new(ErrorCode::OrderNotFound)),
    //     Err(e) => {
    //         tracing::error!("删除订单失败: {}", e);
    //         Err(ApiError::with_details(
    //             ErrorCode::DatabaseError,
    //             "数据库操作失败".to_string(),
    //         ))
    //     }
    // }
    Err(ApiError::new(ErrorCode::InternalError))
}

/// 获取用户订单统计
#[utoipa::path(
    get,
    path = "/api/orders/stats",
    tag = "orders",
    responses(
        (status = 200, description = "获取订单统计成功"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/orders/stats")]
pub async fn get_order_stats(// repo: web::Data<OrderRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现订单统计
    // 1. 统计不同状态的订单数量
    // 2. 统计总金额
    // 3. 统计最近订单趋势

    Err(ApiError::new(ErrorCode::InternalError))
}
