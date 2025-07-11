-- 服务器管理相关数据表

-- 服务器基础表
CREATE TABLE purple_servers (
    id SERIAL PRIMARY KEY,
    protocol VARCHAR(20) NOT NULL,           -- shadowsocks, vmess, trojan, hysteria
    name VARCHAR(255) NOT NULL,
    host VARCHAR(255) NOT NULL,
    port VARCHAR(20) NOT NULL,               -- 支持端口段 "80-90"
    server_port INTEGER NOT NULL,
    rate REAL NOT NULL DEFAULT 1.0,
    show BOOLEAN NOT NULL DEFAULT false,
    sort INTEGER,
    group_ids INTEGER[] NOT NULL DEFAULT '{}', -- PostgreSQL数组，权限组ID
    route_ids INTEGER[],                     -- 路由规则ID数组
    parent_id INTEGER REFERENCES purple_servers(id), -- 父节点ID（用于中转）
    tags TEXT[],                            -- 节点标签数组
    config JSONB,                           -- 协议特定配置
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

-- 服务器组表
CREATE TABLE purple_server_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

-- 路由规则表
CREATE TABLE purple_server_routes (
    id SERIAL PRIMARY KEY,
    remarks VARCHAR(255) NOT NULL,
    match_rules JSONB NOT NULL,              -- 匹配规则数组
    action VARCHAR(20) NOT NULL,             -- block, dns
    action_value VARCHAR(255),
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

-- 服务器统计表
CREATE TABLE purple_server_stats (
    id SERIAL PRIMARY KEY,
    server_id INTEGER NOT NULL REFERENCES purple_servers(id) ON DELETE CASCADE,
    upload_bytes BIGINT NOT NULL DEFAULT 0,
    download_bytes BIGINT NOT NULL DEFAULT 0,
    record_type CHAR(1) NOT NULL,            -- d=day, m=month
    record_at BIGINT NOT NULL,               -- 记录时间戳
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    UNIQUE(server_id, record_type, record_at)
);

-- 服务器日志表（用户流量记录）
-- 注意：此表依赖 purple_users 表，如果用户表不存在则暂时注释掉外键约束
CREATE TABLE purple_server_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL, -- REFERENCES purple_users(id) ON DELETE CASCADE,
    server_id INTEGER NOT NULL REFERENCES purple_servers(id) ON DELETE CASCADE,
    upload_bytes BIGINT NOT NULL DEFAULT 0,
    download_bytes BIGINT NOT NULL DEFAULT 0,
    rate REAL NOT NULL DEFAULT 1.0, -- 流量倍率
    log_at BIGINT NOT NULL,                  -- 日志时间戳
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

-- 创建索引优化查询性能

-- 服务器表索引
CREATE INDEX idx_servers_protocol ON purple_servers(protocol);
CREATE INDEX idx_servers_show ON purple_servers(show);
CREATE INDEX idx_servers_sort ON purple_servers(sort);
CREATE INDEX idx_servers_parent_id ON purple_servers(parent_id);
CREATE INDEX idx_servers_group_ids ON purple_servers USING GIN(group_ids);
CREATE INDEX idx_servers_route_ids ON purple_servers USING GIN(route_ids);
CREATE INDEX idx_servers_tags ON purple_servers USING GIN(tags);
CREATE INDEX idx_servers_config ON purple_servers USING GIN(config);

-- 服务器组表索引
CREATE INDEX idx_server_groups_name ON purple_server_groups(name);

-- 路由规则表索引
CREATE INDEX idx_server_routes_action ON purple_server_routes(action);
CREATE INDEX idx_server_routes_match_rules ON purple_server_routes USING GIN(match_rules);

-- 服务器统计表索引
CREATE INDEX idx_server_stats_server_id ON purple_server_stats(server_id);
CREATE INDEX idx_server_stats_record_type ON purple_server_stats(record_type);
CREATE INDEX idx_server_stats_record_at ON purple_server_stats(record_at);
CREATE INDEX idx_server_stats_server_record ON purple_server_stats(server_id, record_type, record_at);

-- 服务器日志表索引
CREATE INDEX idx_server_logs_user_id ON purple_server_logs(user_id);
CREATE INDEX idx_server_logs_server_id ON purple_server_logs(server_id);
CREATE INDEX idx_server_logs_log_at ON purple_server_logs(log_at);
CREATE INDEX idx_server_logs_user_server ON purple_server_logs(user_id, server_id);

-- 插入默认数据

-- 插入默认服务器组
INSERT INTO purple_server_groups (name, created_at, updated_at) VALUES 
('默认组', EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT),
('VIP组', EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT),
('高级组', EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT);

-- 插入默认路由规则
INSERT INTO purple_server_routes (remarks, match_rules, action, action_value, created_at, updated_at) VALUES 
(
    '阻止广告域名', 
    '["doubleclick.net", "googleads.g.doubleclick.net", "googlesyndication.com"]'::JSONB,
    'block', 
    NULL,
    EXTRACT(EPOCH FROM NOW())::BIGINT, 
    EXTRACT(EPOCH FROM NOW())::BIGINT
),
(
    '中国大陆直连', 
    '["geoip:cn", "geolocation-cn"]'::JSONB,
    'direct', 
    NULL,
    EXTRACT(EPOCH FROM NOW())::BIGINT, 
    EXTRACT(EPOCH FROM NOW())::BIGINT
),
(
    '自定义DNS', 
    '["twitter.com", "facebook.com", "youtube.com"]'::JSONB,
    'dns', 
    '8.8.8.8',
    EXTRACT(EPOCH FROM NOW())::BIGINT, 
    EXTRACT(EPOCH FROM NOW())::BIGINT
);

-- 添加表注释
COMMENT ON TABLE purple_servers IS '服务器节点表';
COMMENT ON TABLE purple_server_groups IS '服务器权限组表';
COMMENT ON TABLE purple_server_routes IS '服务器路由规则表';
COMMENT ON TABLE purple_server_stats IS '服务器统计数据表';
COMMENT ON TABLE purple_server_logs IS '服务器用户流量日志表';

-- 添加列注释
COMMENT ON COLUMN purple_servers.protocol IS '服务器协议类型：shadowsocks, vmess, trojan, hysteria';
COMMENT ON COLUMN purple_servers.port IS '连接端口，支持端口段格式如"80-90"';
COMMENT ON COLUMN purple_servers.server_port IS '服务器实际监听端口';
COMMENT ON COLUMN purple_servers.rate IS '流量倍率，1.0表示1倍';
COMMENT ON COLUMN purple_servers.show IS '是否向用户显示';
COMMENT ON COLUMN purple_servers.sort IS '排序权重，数值越小越靠前';
COMMENT ON COLUMN purple_servers.group_ids IS '权限组ID数组，控制用户访问权限';
COMMENT ON COLUMN purple_servers.route_ids IS '路由规则ID数组';
COMMENT ON COLUMN purple_servers.parent_id IS '父节点ID，用于中转节点';
COMMENT ON COLUMN purple_servers.tags IS '节点标签数组';
COMMENT ON COLUMN purple_servers.config IS '协议特定配置，JSON格式';

COMMENT ON COLUMN purple_server_stats.record_type IS '记录类型：d=天统计，m=月统计';
COMMENT ON COLUMN purple_server_stats.record_at IS '统计时间点的时间戳';

COMMENT ON COLUMN purple_server_routes.match_rules IS '匹配规则数组，支持域名、IP段等';
COMMENT ON COLUMN purple_server_routes.action IS '路由动作：block=阻止，dns=DNS解析，direct=直连';
COMMENT ON COLUMN purple_server_routes.action_value IS '动作参数，如DNS服务器地址';

COMMENT ON COLUMN purple_server_logs.rate IS '该次连接使用的流量倍率';
COMMENT ON COLUMN purple_server_logs.log_at IS '流量产生的时间戳';