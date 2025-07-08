use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::{
    common::{
        response_v2::{ApiError, ApiResponse, IntoHttpResponse},
        ErrorCode,
    },
    models::subscribe::{
        ExpireInfo, PlanInfo, ResetTokenRequest, TrafficInfo, TrafficReportRequest, UserStatus,
        UserSubscribeInfo,
    },
    // repositories::{UserRepository, ServerRepository},
};

/// 获取用户订阅信息
#[utoipa::path(
    get,
    path = "/api/user/subscribe/info",
    tag = "subscribe",
    responses(
        (status = 200, description = "获取订阅信息成功"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/user/subscribe/info")]
pub async fn get_subscribe_info(// user_repo: web::Data<UserRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现获取用户订阅信息
    // 1. 从JWT获取用户ID
    // 2. 查询用户详细信息
    // 3. 查询用户套餐信息
    // 4. 构建订阅链接
    // 5. 计算流量和过期信息

    // 示例数据 - 实际应该从数据库查询
    let traffic = TrafficInfo::new(107374182400, 5368709120, 3221225472); // 100GB总量
    let expire = ExpireInfo::new(Some(1735689600)); // 2025-01-01
    let plan = Some(PlanInfo {
        id: 1,
        name: "标准套餐".to_string(),
        transfer_enable: 107374182400,
        speed_limit: Some(100),
    });
    let status = UserStatus::new(false, false);

    let subscribe_info = UserSubscribeInfo {
        subscribe_url: "https://example.com/api/subscribe/abcd1234".to_string(),
        token: "abcd1234efgh5678".to_string(),
        traffic,
        expire,
        plan,
        status,
    };

    let response = ApiResponse::success(subscribe_info);
    Ok(response.into_http_response())
}

/// 获取订阅链接和二维码
#[utoipa::path(
    get,
    path = "/api/user/subscribe/link",
    tag = "subscribe",
    responses(
        (status = 200, description = "获取订阅链接成功"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/user/subscribe/link")]
pub async fn get_subscribe_link(// user_repo: web::Data<UserRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现获取订阅链接
    // 1. 从JWT获取用户ID
    // 2. 查询用户订阅token
    // 3. 生成订阅链接和二维码
    // 4. 返回支持的客户端信息

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 重置订阅令牌
#[utoipa::path(
    post,
    path = "/api/user/subscribe/reset",
    tag = "subscribe",
    request_body = ResetTokenRequest,
    responses(
        (status = 200, description = "重置令牌成功"),
        (status = 400, description = "请求参数无效"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/user/subscribe/reset")]
pub async fn reset_subscribe_token(
    reset_request: web::Json<ResetTokenRequest>,
    // user_repo: web::Data<UserRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = reset_request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    if !reset_request.confirm {
        return Err(ApiError::with_details(
            ErrorCode::ValidationError,
            "需要确认重置操作".to_string(),
        ));
    }

    // TODO: 实现重置订阅令牌
    // 1. 从JWT获取用户ID
    // 2. 生成新的订阅token
    // 3. 更新用户记录
    // 4. 返回新的订阅信息

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 获取订阅统计
#[utoipa::path(
    get,
    path = "/api/user/subscribe/stats",
    tag = "subscribe",
    responses(
        (status = 200, description = "获取订阅统计成功"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/user/subscribe/stats")]
pub async fn get_subscribe_stats(// user_repo: web::Data<UserRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现获取订阅统计
    // 1. 统计今日和本月流量使用
    // 2. 统计登录信息
    // 3. 统计在线设备数

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 客户端获取订阅配置（通过token）
#[utoipa::path(
    get,
    path = "/api/subscribe/{token}",
    tag = "subscribe",
    params(
        ("token" = String, Path, description = "订阅令牌")
    ),
    responses(
        (status = 200, description = "获取订阅配置成功", content_type = "text/plain"),
        (status = 404, description = "订阅不存在"),
        (status = 403, description = "订阅已过期或被禁用"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/subscribe/{token}")]
pub async fn get_subscribe_config(
    token: web::Path<String>,
    query: web::Query<SubscribeQuery>,
    // user_repo: web::Data<UserRepository>,
    // server_repo: web::Data<ServerRepository>,
) -> Result<HttpResponse, ApiError> {
    let token = token.into_inner();

    // TODO: 实现获取订阅配置
    // 1. 根据token查询用户
    // 2. 验证用户状态（未封禁、未过期）
    // 3. 查询用户可用的服务器节点
    // 4. 根据客户端类型生成配置
    // 5. 更新用户最后访问时间

    // 根据客户端类型返回不同格式的配置
    match query.client.as_deref() {
        Some("clash") => {
            // 返回Clash配置
            let yaml_config = generate_clash_config(&token).await?;
            Ok(HttpResponse::Ok()
                .content_type("text/yaml")
                .body(yaml_config))
        }
        Some("v2ray") => {
            // 返回V2Ray配置
            let json_config = generate_v2ray_config(&token).await?;
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json_config))
        }
        _ => {
            // 默认返回base64编码的配置
            let base64_config = generate_base64_config(&token).await?;
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(base64_config))
        }
    }
}

/// 节点流量上报（节点端使用）
#[utoipa::path(
    post,
    path = "/api/server/traffic/report",
    tag = "subscribe",
    request_body = TrafficReportRequest,
    responses(
        (status = 200, description = "流量上报成功"),
        (status = 400, description = "请求参数无效"),
        (status = 401, description = "节点认证失败"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/server/traffic/report")]
pub async fn report_traffic(
    traffic_data: web::Json<TrafficReportRequest>,
    // user_repo: web::Data<UserRepository>,
    // TODO: 验证节点认证token
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = traffic_data.validate() {
        return Err(ApiError::from(validation_errors));
    }

    // TODO: 实现流量上报
    // 1. 验证节点认证
    // 2. 批量更新用户流量数据
    // 3. 检查用户流量是否超限
    // 4. 记录流量日志

    let response = ApiResponse::success(());
    Ok(response.into_http_response())
}

/// 订阅查询参数
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SubscribeQuery {
    /// 客户端类型：clash, v2ray, 等
    pub client: Option<String>,
    /// 是否包含流量信息头
    #[serde(default)]
    pub include_header: bool,
}

// 辅助函数：生成Clash配置
async fn generate_clash_config(token: &str) -> Result<String, ApiError> {
    // TODO: 实现Clash配置生成
    // 1. 查询用户可用节点
    // 2. 生成Clash YAML配置
    // 3. 包含代理规则和节点信息

    Ok("# Clash配置示例\nproxies: []\nproxy-groups: []\nrules: []".to_string())
}

// 辅助函数：生成V2Ray配置
async fn generate_v2ray_config(token: &str) -> Result<String, ApiError> {
    // TODO: 实现V2Ray配置生成
    // 1. 查询用户可用节点
    // 2. 生成V2Ray JSON配置
    // 3. 包含入站出站规则

    Ok(r#"{"outbounds": []}"#.to_string())
}

// 辅助函数：生成Base64配置
async fn generate_base64_config(token: &str) -> Result<String, ApiError> {
    // TODO: 实现Base64配置生成
    // 1. 查询用户可用节点
    // 2. 生成各类型节点的URI
    // 3. 用换行分隔，整体base64编码

    Ok("".to_string())
}

/// 获取节点在线状态
#[utoipa::path(
    get,
    path = "/api/user/subscribe/nodes/status",
    tag = "subscribe",
    responses(
        (status = 200, description = "获取节点状态成功"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[get("/api/user/subscribe/nodes/status")]
pub async fn get_nodes_status(// user_repo: web::Data<UserRepository>,
    // server_repo: web::Data<ServerRepository>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    // TODO: 实现获取节点状态
    // 1. 查询用户可用节点
    // 2. 检查各节点在线状态
    // 3. 返回节点状态列表

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 节点状态响应
#[derive(Debug, Serialize, ToSchema)]
pub struct NodeStatusResponse {
    pub nodes: Vec<NodeStatus>,
    pub total_nodes: i32,
    pub online_nodes: i32,
}

/// 单个节点状态
#[derive(Debug, Serialize, ToSchema)]
pub struct NodeStatus {
    pub id: i32,
    pub name: String,
    pub node_type: String,
    pub location: String,
    pub online: bool,
    pub latency: Option<i32>, // 延迟（毫秒）
    pub load: Option<f32>,    // 负载
    pub rate: f32,            // 倍率
}

/// 测试订阅链接连通性
#[utoipa::path(
    post,
    path = "/api/user/subscribe/test",
    tag = "subscribe",
    request_body = TestSubscribeRequest,
    responses(
        (status = 200, description = "测试成功"),
        (status = 400, description = "请求参数无效"),
        (status = 401, description = "未授权"),
        (status = 500, description = "内部服务器错误")
    )
)]
#[post("/api/user/subscribe/test")]
pub async fn test_subscribe_connectivity(
    test_request: web::Json<TestSubscribeRequest>,
    // TODO: 从JWT中获取用户ID
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = test_request.validate() {
        return Err(ApiError::from(validation_errors));
    }

    // TODO: 实现订阅连通性测试
    // 1. 验证用户权限
    // 2. 测试指定节点的连通性
    // 3. 返回测试结果

    Err(ApiError::new(ErrorCode::InternalError))
}

/// 测试订阅请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TestSubscribeRequest {
    /// 要测试的节点ID（可选，不指定则测试所有）
    pub node_id: Option<i32>,
    /// 测试超时时间（秒）
    #[validate(range(min = 1, max = 30))]
    #[serde(default = "default_timeout")]
    pub timeout: i32,
}

fn default_timeout() -> i32 {
    10
}

/// 测试结果响应
#[derive(Debug, Serialize, ToSchema)]
pub struct TestResultResponse {
    pub test_time: i64,
    pub results: Vec<NodeTestResult>,
    pub summary: TestSummary,
}

/// 单个节点测试结果
#[derive(Debug, Serialize, ToSchema)]
pub struct NodeTestResult {
    pub node_id: i32,
    pub node_name: String,
    pub success: bool,
    pub latency: Option<i32>,
    pub error: Option<String>,
}

/// 测试摘要
#[derive(Debug, Serialize, ToSchema)]
pub struct TestSummary {
    pub total_nodes: i32,
    pub success_nodes: i32,
    pub average_latency: Option<f32>,
    pub fastest_node: Option<String>,
}
