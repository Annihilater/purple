use actix_web::{delete, get, post, put, web, HttpResponse};
use std::sync::Arc;

use crate::{
    app_state::AppState,
    common::response_v2::{ApiError, ApiResponse, IntoHttpResponse},
    common::ErrorCode,
    models::server::{
        CopyServerRequest, CreateServerGroupRequest, CreateServerRequest, CreateServerRouteRequest,
        ServerResponse, ServerSortRequest, UpdateServerGroupRequest, UpdateServerRequest,
        UpdateServerRouteRequest, UserServerResponse,
    },
    repositories::ServerRepository,
};

// ========== 管理员接口 ==========

/// 创建服务器
#[post("")]
pub async fn create_server(
    state: web::Data<AppState>,
    request: web::Json<CreateServerRequest>,
) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.create_server(&request).await {
        Ok(id) => {
            let response = ApiResponse::success(id);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建服务器失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "创建服务器失败".to_string(),
            ))
        }
    }
}

/// 获取所有服务器
#[get("")]
pub async fn get_servers(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.get_all_servers().await {
        Ok(servers) => {
            // 转换为响应格式并合并运行时状态
            let server_responses: Vec<ServerResponse> = servers
                .into_iter()
                .map(|server| {
                    let mut response = ServerResponse::from(server.clone());
                    // TODO: 从缓存获取运行时状态并更新 response
                    response
                })
                .collect();

            let response = ApiResponse::success(server_responses);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取服务器列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "获取服务器列表失败".to_string(),
            ))
        }
    }
}

/// 获取服务器详情
#[get("/{id}")]
pub async fn get_server(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.get_server_by_id(id).await {
        Ok(Some(server)) => {
            let mut response = ServerResponse::from(server);
            // TODO: 从缓存获取运行时状态并更新 response

            let api_response = ApiResponse::success(response);
            Ok(api_response.into_http_response())
        }
        Ok(None) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "服务器不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("获取服务器详情失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "获取服务器详情失败".to_string(),
            ))
        }
    }
}

/// 更新服务器
#[put("/{id}")]
pub async fn update_server(
    state: web::Data<AppState>,
    path: web::Path<u32>,
    request: web::Json<UpdateServerRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.update_server(id, &request).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "服务器不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("更新服务器失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "更新服务器失败".to_string(),
            ))
        }
    }
}

/// 删除服务器
#[delete("/{id}")]
pub async fn delete_server(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.delete_server(id).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "服务器不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("删除服务器失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "删除服务器失败".to_string(),
            ))
        }
    }
}

/// 复制服务器
#[post("/{id}/copy")]
pub async fn copy_server(
    state: web::Data<AppState>,
    path: web::Path<u32>,
    request: web::Json<CopyServerRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo
        .copy_server(
            id,
            &request.name,
            request.host.as_deref(),
            request.server_port,
        )
        .await
    {
        Ok(new_id) => {
            let response = ApiResponse::success(new_id);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("复制服务器失败: {}", e);
            if e.to_string().contains("服务器不存在") {
                Err(ApiError::with_details(
                    ErrorCode::NotFound,
                    "要复制的服务器不存在".to_string(),
                ))
            } else {
                Err(ApiError::with_details(
                    ErrorCode::DatabaseError,
                    "复制服务器失败".to_string(),
                ))
            }
        }
    }
}

/// 批量更新服务器排序
#[put("/sort")]
pub async fn sort_servers(
    state: web::Data<AppState>,
    request: web::Json<ServerSortRequest>,
) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.update_servers_sort(&request.servers).await {
        Ok(_) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("批量排序失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "批量排序失败".to_string(),
            ))
        }
    }
}

// ========== 服务器组管理 ==========

/// 创建服务器组
#[post("/groups")]
pub async fn create_server_group(
    state: web::Data<AppState>,
    request: web::Json<CreateServerGroupRequest>,
) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.create_server_group(&request).await {
        Ok(id) => {
            let response = ApiResponse::success(id);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建服务器组失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "创建服务器组失败".to_string(),
            ))
        }
    }
}

/// 获取服务器组列表
#[get("/groups")]
pub async fn get_server_groups(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.get_all_server_groups().await {
        Ok(groups) => {
            let response = ApiResponse::success(groups);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取服务器组列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "获取服务器组列表失败".to_string(),
            ))
        }
    }
}

/// 更新服务器组
#[put("/groups/{id}")]
pub async fn update_server_group(
    state: web::Data<AppState>,
    path: web::Path<u32>,
    request: web::Json<UpdateServerGroupRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.update_server_group(id, &request).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "服务器组不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("更新服务器组失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "更新服务器组失败".to_string(),
            ))
        }
    }
}

/// 删除服务器组
#[delete("/groups/{id}")]
pub async fn delete_server_group(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.delete_server_group(id).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "服务器组不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("删除服务器组失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "删除服务器组失败".to_string(),
            ))
        }
    }
}

// ========== 路由规则管理 ==========

/// 创建路由规则
#[post("/routes")]
pub async fn create_server_route(
    state: web::Data<AppState>,
    request: web::Json<CreateServerRouteRequest>,
) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.create_server_route(&request).await {
        Ok(id) => {
            let response = ApiResponse::success(id);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建路由规则失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "创建路由规则失败".to_string(),
            ))
        }
    }
}

/// 获取路由规则列表
#[get("/routes")]
pub async fn get_server_routes(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.get_all_server_routes().await {
        Ok(routes) => {
            let response = ApiResponse::success(routes);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取路由规则列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "获取路由规则列表失败".to_string(),
            ))
        }
    }
}

/// 更新路由规则
#[put("/routes/{id}")]
pub async fn update_server_route(
    state: web::Data<AppState>,
    path: web::Path<u32>,
    request: web::Json<UpdateServerRouteRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.update_server_route(id, &request).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "路由规则不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("更新路由规则失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "更新路由规则失败".to_string(),
            ))
        }
    }
}

/// 删除路由规则
#[delete("/routes/{id}")]
pub async fn delete_server_route(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let server_repo = ServerRepository::new(state.db.clone());

    match server_repo.delete_server_route(id).await {
        Ok(true) => {
            let response = ApiResponse::success(true);
            Ok(response.into_http_response())
        }
        Ok(false) => Err(ApiError::with_details(
            ErrorCode::NotFound,
            "路由规则不存在".to_string(),
        )),
        Err(e) => {
            tracing::error!("删除路由规则失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "删除路由规则失败".to_string(),
            ))
        }
    }
}

// ========== 用户接口 ==========

/// 获取用户可用服务器
#[get("/user/servers")]
pub async fn get_user_servers(
    state: web::Data<AppState>,
    // TODO: 从 JWT token 中提取用户信息
) -> Result<HttpResponse, ApiError> {
    let server_repo = ServerRepository::new(state.db.clone());

    // TODO: 从认证中间件获取用户权限组
    let user_group_ids = vec![1u32]; // 临时硬编码

    match server_repo.get_servers_for_user(&user_group_ids).await {
        Ok(servers) => {
            // 转换为用户可见格式（过滤敏感信息）
            let user_servers: Vec<UserServerResponse> = servers
                .into_iter()
                .map(|server| {
                    // 过滤配置中的敏感信息
                    let filtered_config = filter_sensitive_config(&server.config, &server.protocol);

                    UserServerResponse {
                        id: server.id,
                        name: server.name,
                        host: server.host,
                        port: server.port, // TODO: 如果是端口段，需要随机化
                        server_port: server.server_port,
                        rate: server.rate,
                        tags: server.tags,
                        status: crate::models::server::ServerStatus::Online, // TODO: 从缓存获取
                        load: 0.0,                                           // TODO: 从缓存获取
                        config: filtered_config,
                    }
                })
                .collect();

            let response = ApiResponse::success(user_servers);
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("获取用户服务器列表失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "获取服务器列表失败".to_string(),
            ))
        }
    }
}

// ========== 辅助函数 ==========

/// 过滤配置中的敏感信息
fn filter_sensitive_config(
    config: &serde_json::Value,
    protocol: &crate::models::server::ServerProtocol,
) -> serde_json::Value {
    match protocol {
        crate::models::server::ServerProtocol::Shadowsocks => {
            // 保留除密码外的所有配置
            if let Some(obj) = config.as_object() {
                let mut filtered = obj.clone();
                filtered.remove("password");
                serde_json::Value::Object(filtered)
            } else {
                config.clone()
            }
        }
        crate::models::server::ServerProtocol::Vmess => {
            // 保留客户端需要的配置
            if let Some(obj) = config.as_object() {
                let mut filtered = serde_json::Map::new();
                if let Some(uuid) = obj.get("uuid") {
                    filtered.insert("uuid".to_string(), uuid.clone());
                }
                if let Some(alter_id) = obj.get("alter_id") {
                    filtered.insert("alter_id".to_string(), alter_id.clone());
                }
                if let Some(security) = obj.get("security") {
                    filtered.insert("security".to_string(), security.clone());
                }
                if let Some(network) = obj.get("network") {
                    filtered.insert("network".to_string(), network.clone());
                }
                if let Some(tls) = obj.get("tls") {
                    filtered.insert("tls".to_string(), tls.clone());
                }
                serde_json::Value::Object(filtered)
            } else {
                config.clone()
            }
        }
        crate::models::server::ServerProtocol::Trojan => {
            // 移除密码
            if let Some(obj) = config.as_object() {
                let mut filtered = obj.clone();
                filtered.remove("password");
                serde_json::Value::Object(filtered)
            } else {
                config.clone()
            }
        }
        crate::models::server::ServerProtocol::Hysteria => {
            // 移除密码
            if let Some(obj) = config.as_object() {
                let mut filtered = obj.clone();
                filtered.remove("password");
                serde_json::Value::Object(filtered)
            } else {
                config.clone()
            }
        }
    }
}
