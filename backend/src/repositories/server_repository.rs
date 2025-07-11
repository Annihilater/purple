use anyhow::Result;
use sqlx::{PgPool, Row};

use crate::models::server::{
    CreateServerGroupRequest, CreateServerRequest, CreateServerRouteRequest, Server, ServerGroup,
    ServerRoute, ServerSortItem, ServerStats, UpdateServerGroupRequest, UpdateServerRequest,
    UpdateServerRouteRequest,
};

#[derive(Clone)]
pub struct ServerRepository {
    pool: PgPool,
}

impl ServerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========== 服务器管理 ==========

    /// 创建服务器
    pub async fn create_server(&self, request: &CreateServerRequest) -> Result<u32> {
        let now = chrono::Utc::now().timestamp();

        let route_ids_ref = request
            .route_ids
            .as_ref()
            .map(|ids| ids.iter().map(|&id| id as i32).collect::<Vec<_>>());

        let row = sqlx::query!(
            r#"
            INSERT INTO purple_servers (
                protocol, name, host, port, server_port, rate, show, sort,
                group_ids, route_ids, parent_id, tags, config, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id
            "#,
            request.protocol.to_string(),
            request.name,
            request.host,
            request.port,
            request.server_port as i32,
            request.rate,
            request.show,
            request.sort.map(|s| s as i32),
            &request
                .group_ids
                .iter()
                .map(|&id| id as i32)
                .collect::<Vec<_>>(),
            route_ids_ref.as_deref(),
            request.parent_id.map(|id| id as i32),
            request.tags.as_ref().map(|tags| tags.as_slice()),
            request.config,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id as u32)
    }

    /// 获取所有服务器
    pub async fn get_all_servers(&self) -> Result<Vec<Server>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, protocol, name, host, port, server_port, rate, show, sort,
                group_ids, route_ids, parent_id, tags, config, created_at, updated_at
            FROM purple_servers 
            ORDER BY sort ASC, id ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let servers = rows
            .into_iter()
            .map(|row| {
                let protocol = match row.protocol.as_str() {
                    "shadowsocks" => crate::models::server::ServerProtocol::Shadowsocks,
                    "vmess" => crate::models::server::ServerProtocol::Vmess,
                    "trojan" => crate::models::server::ServerProtocol::Trojan,
                    "hysteria" => crate::models::server::ServerProtocol::Hysteria,
                    _ => crate::models::server::ServerProtocol::Shadowsocks, // 默认值
                };

                Server {
                    id: row.id as u32,
                    protocol,
                    name: row.name,
                    host: row.host,
                    port: row.port,
                    server_port: row.server_port as u16,
                    rate: row.rate as f32,
                    show: row.show,
                    sort: row.sort.map(|s| s as u32),
                    group_ids: row.group_ids.into_iter().map(|id| id as u32).collect(),
                    route_ids: row
                        .route_ids
                        .map(|ids| ids.into_iter().map(|id| id as u32).collect()),
                    parent_id: row.parent_id.map(|id| id as u32),
                    tags: row.tags,
                    config: row.config.unwrap_or_default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .collect();

        Ok(servers)
    }

    /// 根据ID获取服务器
    pub async fn get_server_by_id(&self, id: u32) -> Result<Option<Server>> {
        let row = sqlx::query!(
            r#"
            SELECT 
                id, protocol, name, host, port, server_port, rate, show, sort,
                group_ids, route_ids, parent_id, tags, config, created_at, updated_at
            FROM purple_servers 
            WHERE id = $1
            "#,
            id as i32
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let protocol = match row.protocol.as_str() {
                "shadowsocks" => crate::models::server::ServerProtocol::Shadowsocks,
                "vmess" => crate::models::server::ServerProtocol::Vmess,
                "trojan" => crate::models::server::ServerProtocol::Trojan,
                "hysteria" => crate::models::server::ServerProtocol::Hysteria,
                _ => crate::models::server::ServerProtocol::Shadowsocks,
            };

            let server = Server {
                id: row.id as u32,
                protocol,
                name: row.name,
                host: row.host,
                port: row.port,
                server_port: row.server_port as u16,
                rate: row.rate as f32,
                show: row.show,
                sort: row.sort.map(|s| s as u32),
                group_ids: row.group_ids.into_iter().map(|id| id as u32).collect(),
                route_ids: row
                    .route_ids
                    .map(|ids| ids.into_iter().map(|id| id as u32).collect()),
                parent_id: row.parent_id.map(|id| id as u32),
                tags: row.tags,
                config: row.config.unwrap_or_default(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            Ok(Some(server))
        } else {
            Ok(None)
        }
    }

    /// 更新服务器（简化版本）
    pub async fn update_server(&self, id: u32, request: &UpdateServerRequest) -> Result<bool> {
        let now = chrono::Utc::now().timestamp();

        // 使用简单的 UPDATE 语句而不是动态查询
        let result = sqlx::query!(
            r#"
            UPDATE purple_servers 
            SET name = COALESCE($1, name),
                host = COALESCE($2, host),
                port = COALESCE($3, port),
                server_port = COALESCE($4, server_port),
                rate = COALESCE($5, rate),
                show = COALESCE($6, show),
                sort = COALESCE($7, sort),
                updated_at = $8
            WHERE id = $9
            "#,
            request.name,
            request.host,
            request.port,
            request.server_port.map(|p| p as i32),
            request.rate,
            request.show,
            request.sort.map(|s| s as i32),
            now,
            id as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 删除服务器
    pub async fn delete_server(&self, id: u32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM purple_servers WHERE id = $1", id as i32)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 复制服务器
    pub async fn copy_server(
        &self,
        id: u32,
        new_name: &str,
        new_host: Option<&str>,
        new_server_port: Option<u16>,
    ) -> Result<u32> {
        let server = self
            .get_server_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("服务器不存在"))?;

        let now = chrono::Utc::now().timestamp();
        let host = new_host.unwrap_or(&server.host);
        let server_port = new_server_port.unwrap_or(server.server_port);

        // 准备路由ID数组（避免临时值问题）
        let route_ids_ref = server
            .route_ids
            .as_ref()
            .map(|ids| ids.iter().map(|&id| id as i32).collect::<Vec<_>>());

        let row = sqlx::query!(
            r#"
            INSERT INTO purple_servers (
                protocol, name, host, port, server_port, rate, show, sort,
                group_ids, route_ids, parent_id, tags, config, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id
            "#,
            server.protocol.to_string(),
            new_name,
            host,
            server.port,
            server_port as i32,
            server.rate as f64,
            false, // 复制的服务器默认不显示
            server.sort.map(|s| s as i32),
            &server
                .group_ids
                .iter()
                .map(|&id| id as i32)
                .collect::<Vec<_>>(),
            route_ids_ref.as_deref(),
            server.parent_id.map(|id| id as i32),
            server.tags.as_ref().map(|tags| tags.as_slice()),
            server.config,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id as u32)
    }

    /// 批量更新服务器排序
    pub async fn update_servers_sort(&self, items: &[ServerSortItem]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for item in items {
            sqlx::query!(
                "UPDATE purple_servers SET sort = $1, updated_at = $2 WHERE id = $3",
                item.sort as i32,
                chrono::Utc::now().timestamp(),
                item.id as i32
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// 获取用户可见的服务器（根据权限组过滤）
    pub async fn get_servers_for_user(&self, user_group_ids: &[u32]) -> Result<Vec<Server>> {
        if user_group_ids.is_empty() {
            return Ok(vec![]);
        }

        let group_ids: Vec<i32> = user_group_ids.iter().map(|&id| id as i32).collect();

        let rows = sqlx::query!(
            r#"
            SELECT 
                id, protocol, name, host, port, server_port, rate, show, sort,
                group_ids, route_ids, parent_id, tags, config, created_at, updated_at
            FROM purple_servers 
            WHERE show = true 
            AND group_ids && $1
            ORDER BY sort ASC, id ASC
            "#,
            &group_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let servers = rows
            .into_iter()
            .map(|row| {
                let protocol = match row.protocol.as_str() {
                    "shadowsocks" => crate::models::server::ServerProtocol::Shadowsocks,
                    "vmess" => crate::models::server::ServerProtocol::Vmess,
                    "trojan" => crate::models::server::ServerProtocol::Trojan,
                    "hysteria" => crate::models::server::ServerProtocol::Hysteria,
                    _ => crate::models::server::ServerProtocol::Shadowsocks,
                };

                Server {
                    id: row.id as u32,
                    protocol,
                    name: row.name,
                    host: row.host,
                    port: row.port,
                    server_port: row.server_port as u16,
                    rate: row.rate as f32,
                    show: row.show,
                    sort: row.sort.map(|s| s as u32),
                    group_ids: row.group_ids.into_iter().map(|id| id as u32).collect(),
                    route_ids: row
                        .route_ids
                        .map(|ids| ids.into_iter().map(|id| id as u32).collect()),
                    parent_id: row.parent_id.map(|id| id as u32),
                    tags: row.tags,
                    config: row.config.unwrap_or_default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .collect();

        Ok(servers)
    }

    // ========== 服务器组管理 ==========

    /// 创建服务器组
    pub async fn create_server_group(&self, request: &CreateServerGroupRequest) -> Result<u32> {
        let now = chrono::Utc::now().timestamp();

        let row = sqlx::query!(
            "INSERT INTO purple_server_groups (name, created_at, updated_at) VALUES ($1, $2, $3) RETURNING id",
            request.name,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id as u32)
    }

    /// 获取所有服务器组
    pub async fn get_all_server_groups(&self) -> Result<Vec<ServerGroup>> {
        let rows = sqlx::query!(
            "SELECT id, name, created_at, updated_at FROM purple_server_groups ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;

        let groups = rows
            .into_iter()
            .map(|row| ServerGroup {
                id: row.id as u32,
                name: row.name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(groups)
    }

    /// 更新服务器组
    pub async fn update_server_group(
        &self,
        id: u32,
        request: &UpdateServerGroupRequest,
    ) -> Result<bool> {
        let result = sqlx::query!(
            "UPDATE purple_server_groups SET name = $1, updated_at = $2 WHERE id = $3",
            request.name,
            chrono::Utc::now().timestamp(),
            id as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 删除服务器组
    pub async fn delete_server_group(&self, id: u32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM purple_server_groups WHERE id = $1", id as i32)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ========== 路由规则管理 ==========

    /// 创建路由规则
    pub async fn create_server_route(&self, request: &CreateServerRouteRequest) -> Result<u32> {
        let now = chrono::Utc::now().timestamp();

        let row = sqlx::query!(
            r#"
            INSERT INTO purple_server_routes (remarks, match_rules, action, action_value, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id
            "#,
            request.remarks,
            serde_json::to_value(&request.match_rules)?,
            request.action,
            request.action_value,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id as u32)
    }

    /// 获取所有路由规则
    pub async fn get_all_server_routes(&self) -> Result<Vec<ServerRoute>> {
        let rows = sqlx::query!(
            "SELECT id, remarks, match_rules, action, action_value, created_at, updated_at FROM purple_server_routes ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;

        let routes = rows
            .into_iter()
            .map(|row| {
                let match_rules: Vec<String> =
                    serde_json::from_value(row.match_rules).unwrap_or_default();

                ServerRoute {
                    id: row.id as u32,
                    remarks: row.remarks,
                    match_rules,
                    action: row.action,
                    action_value: row.action_value,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .collect();

        Ok(routes)
    }

    /// 更新路由规则
    pub async fn update_server_route(
        &self,
        id: u32,
        request: &UpdateServerRouteRequest,
    ) -> Result<bool> {
        let now = chrono::Utc::now().timestamp();

        let result = sqlx::query!(
            r#"
            UPDATE purple_server_routes 
            SET remarks = COALESCE($1, remarks),
                match_rules = COALESCE($2, match_rules),
                action = COALESCE($3, action),
                action_value = COALESCE($4, action_value),
                updated_at = $5
            WHERE id = $6
            "#,
            request.remarks,
            request
                .match_rules
                .as_ref()
                .map(|rules| serde_json::to_value(rules).unwrap_or_default()),
            request.action,
            request.action_value,
            now,
            id as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 删除路由规则
    pub async fn delete_server_route(&self, id: u32) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM purple_server_routes WHERE id = $1", id as i32)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ========== 统计数据 ==========

    /// 获取服务器统计数据
    pub async fn get_server_stats(
        &self,
        server_id: u32,
        record_type: &str,
        limit: u32,
    ) -> Result<Vec<ServerStats>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, server_id, upload_bytes, download_bytes, record_type, record_at, created_at, updated_at
            FROM purple_server_stats 
            WHERE server_id = $1 AND record_type = $2
            ORDER BY record_at DESC
            LIMIT $3
            "#,
            server_id as i32,
            record_type,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let stats = rows
            .into_iter()
            .map(|row| ServerStats {
                id: row.id as u32,
                server_id: row.server_id as u32,
                upload_bytes: row.upload_bytes as u64,
                download_bytes: row.download_bytes as u64,
                record_type: row.record_type,
                record_at: row.record_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(stats)
    }
}
