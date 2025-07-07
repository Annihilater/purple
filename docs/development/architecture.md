# 项目架构说明

Purple 采用现代化的分层架构设计，遵循 Rust 生态系统的最佳实践，确保代码的可维护性、可扩展性和高性能。

## 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    HTTP 客户端请求                            │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                  中间件层                                     │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ 请求计时器   │ │ 请求日志    │ │ JWT认证     │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                   API 层                                    │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ 用户API     │ │ 套餐API     │ │ 认证API     │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                  服务层                                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ 认证服务    │ │ 业务逻辑    │ │ 验证服务    │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                 仓库层                                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ 用户仓库    │ │ 套餐仓库    │ │ 优惠券仓库   │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                PostgreSQL 数据库                            │
└─────────────────────────────────────────────────────────────┘
```

## 目录结构

```
src/
├── main.rs                 # 应用程序入口点
├── startup.rs              # 应用启动和配置
├── app_state.rs           # 应用状态管理
├── routes.rs              # 路由配置
├── logging.rs             # 日志系统配置
│
├── api/                   # API 层 - HTTP 请求处理
│   ├── mod.rs            # API 模块声明
│   ├── auth.rs           # 认证相关 API
│   ├── user.rs           # 用户管理 API
│   ├── plan.rs           # 套餐管理 API
│   ├── coupon.rs         # 优惠券管理 API
│   ├── subscribe.rs      # 订阅管理 API
│   ├── health.rs         # 健康检查 API
│   ├── openapi.rs        # OpenAPI 文档配置
│   └── response.rs       # 响应结构定义（已弃用）
│
├── services/              # 服务层 - 业务逻辑
│   ├── mod.rs            # 服务模块声明
│   └── auth.rs           # 认证业务服务
│
├── repositories/          # 仓库层 - 数据访问
│   ├── mod.rs            # 仓库模块声明
│   ├── user_repository.rs    # 用户数据访问
│   ├── plan_repository.rs    # 套餐数据访问
│   └── coupon_repository.rs  # 优惠券数据访问
│
├── models/                # 数据模型层
│   ├── mod.rs            # 模型模块声明
│   ├── user.rs           # 用户相关模型
│   ├── plan.rs           # 套餐相关模型
│   ├── coupon.rs         # 优惠券相关模型
│   ├── auth.rs           # 认证相关模型
│   └── subscribe.rs      # 订阅相关模型
│
├── middleware/            # 中间件系统
│   ├── mod.rs            # 中间件模块声明
│   ├── auth.rs           # JWT 认证中间件
│   ├── cors.rs           # CORS 中间件
│   ├── logging.rs        # 请求日志中间件
│   ├── request_logger.rs # 请求/响应打印中间件
│   └── request_timer.rs  # 请求耗时记录中间件
│
├── common/                # 通用组件
│   ├── mod.rs            # 通用模块声明
│   ├── error.rs          # 错误代码定义
│   ├── response.rs       # 响应结构体
│   ├── response_v2.rs    # 新版响应系统
│   └── status.rs         # 状态码映射
│
├── config/                # 配置管理
│   ├── mod.rs            # 配置模块声明
│   ├── admin.rs          # 管理员账户配置
│   └── database.rs       # 数据库配置
│
└── utils/                 # 工具函数
    └── mod.rs            # 工具模块声明
```

## 分层架构详解

### 1. API 层 (src/api/)

**职责**: 处理 HTTP 请求和响应，参数验证，OpenAPI 文档生成

**特点**:
- 使用 Actix-web 的处理器函数
- 集成 utoipa 自动生成 OpenAPI 文档
- 统一的错误处理和响应格式
- 参数验证和类型安全

**示例**:
```rust
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功"),
        (status = 401, description = "用户名或密码错误")
    )
)]
#[post("/api/auth/login")]
pub async fn login(
    request: web::Json<LoginRequest>,
    service: web::Data<AuthService>,
) -> Result<HttpResponse, ApiError> {
    // 处理登录逻辑
}
```

### 2. 服务层 (src/services/)

**职责**: 业务逻辑处理，协调仓库层操作，事务管理

**特点**:
- 无状态设计
- 依赖注入
- 错误处理和转换
- 业务规则验证

**示例**:
```rust
impl AuthService {
    pub async fn login(&self, request: LoginRequest) -> Result<TokenResponse, anyhow::Error> {
        // 1. 验证用户存在
        let user = self.user_repo.find_by_username(&request.username).await?;
        
        // 2. 验证密码
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }
        
        // 3. 生成 JWT token
        let token = self.generate_token(user.id)?;
        
        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 604800, // 7 days
        })
    }
}
```

### 3. 仓库层 (src/repositories/)

**职责**: 数据访问抽象，SQL 查询，数据库事务

**特点**:
- 使用 SQLx 进行类型安全的 SQL 操作
- 异步数据库操作
- 连接池管理
- 数据库无关的接口设计

**示例**:
```rust
impl UserRepository {
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM purple_user WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await
    }
    
    pub async fn create(&self, user: &CreateUser) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO purple_user (username, email, password) VALUES ($1, $2, $3) RETURNING id",
            user.username,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.id)
    }
}
```

### 4. 模型层 (src/models/)

**职责**: 数据结构定义，序列化/反序列化，验证规则

**特点**:
- 使用 Serde 进行 JSON 序列化
- 使用 Validator 进行数据验证
- utoipa 集成生成 OpenAPI schema
- 类型安全和编译时检查

**示例**:
```rust
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

## 中间件系统

### 认证中间件 (auth.rs)
- JWT token 验证
- 用户状态检查
- 权限控制

### 请求日志中间件 (request_logger.rs)
- 记录所有请求和响应
- 根据状态码使用不同日志级别
- 隐藏敏感信息

### 请求计时中间件 (request_timer.rs)
- 精确计算请求耗时
- 性能预警和分级
- 自动性能优化建议

### CORS 中间件 (cors.rs)
- 跨域资源共享配置
- 开发和生产环境适配

## 通用响应系统

### 统一响应格式
```rust
pub struct ApiResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorInfo>,
    pub meta: ResponseMeta,
}
```

### 错误代码系统
- **1000-1999**: 通用错误
- **2000-2999**: 认证错误
- **3000-3999**: 用户错误
- **4000-4999**: 套餐错误
- **5000-5999**: 优惠券错误
- **6000-6999**: 订单错误

## 配置管理

### 应用状态 (app_state.rs)
- 依赖注入容器
- 服务和仓库实例管理
- 数据库连接池

### 环境配置
- 支持 .env 文件
- 环境变量覆盖
- 配置验证和默认值

## 数据库设计

### 表命名规范
- 统一使用 `purple_` 前缀
- 主要表: `purple_user`, `purple_plan`, `purple_coupon`, `purple_order`

### 连接池配置
- 最大连接数: 5
- 连接超时: 30 秒
- 空闲超时: 10 分钟

## 日志系统

### 双输出设计
- **控制台输出**: 开发环境彩色格式
- **文件输出**: 生产环境结构化日志

### 日志级别
- **ERROR**: 系统错误和异常
- **WARN**: 警告信息和性能问题
- **INFO**: 正常操作日志
- **DEBUG**: 详细调试信息

## 性能优化

### 异步设计
- 全面使用 async/await
- 非阻塞 I/O 操作
- 高并发处理能力

### 内存管理
- 零拷贝优化
- 智能指针使用
- 栈上分配优先

### 数据库优化
- 连接池复用
- 预编译语句
- 索引优化

## 安全设计

### 认证和授权
- JWT 无状态认证
- 细粒度权限控制
- Token 过期机制

### 数据安全
- SQL 注入防护
- XSS 攻击防护
- 密码哈希存储

### 请求安全
- 参数验证
- 速率限制
- CORS 配置

## 可扩展性

### 模块化设计
- 松耦合架构
- 接口抽象
- 依赖注入

### 水平扩展
- 无状态服务设计
- 数据库读写分离支持
- 缓存系统集成预留

### 监控和观测
- 请求追踪
- 性能指标
- 错误监控

## 最佳实践

### 代码组织
- 单一职责原则
- 依赖倒置原则
- 开闭原则

### 错误处理
- 显式错误处理
- 错误传播和转换
- 用户友好的错误信息

### 测试策略
- 单元测试
- 集成测试
- API 测试

这种架构设计确保了 Purple 项目的高性能、可维护性和可扩展性，同时遵循 Rust 生态系统的最佳实践。