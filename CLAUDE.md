# CLAUDE.md

此文件为 Claude Code (claude.ai/code) 在此代码仓库中工作时提供指导。

## 重要工作规则

**中文回复规则**: 在此代码仓库中工作时，Claude 必须使用中文回复用户的所有问题和请求。所有交流都应使用中文进行。

**Rust 最佳实践规则**: 总是从 Rust 最佳项目实践的方向思考和解决问题，保持整个项目代码风格一致，逻辑自洽和闭合。遵循 Rust 社区的惯用法和设计模式，确保代码质量和可维护性。

## 常用开发命令

### 构建和测试

```bash
# 构建项目
cargo build

# 生产环境构建
cargo build --release

# 运行测试
cargo test

# 运行特定测试
cargo test --test test_name

# 检查代码不构建
cargo check
```

### 代码质量

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行数据库迁移
psql -U username -d purple -f migrations/init.sql
```

### 运行应用

```bash
# 启动开发服务器
cargo run

# 使用自定义环境启动
RUST_LOG=debug cargo run
```

### 环境配置

创建 `.env` 文件，包含以下内容：

```
# 数据库连接配置
DATABASE_URL=postgresql://purple:purple@localhost:5432/purple

# JWT 秘钥
JWT_SECRET=your-secret-key-here-please-change-in-production

# 服务器配置
SERVER_ADDR=127.0.0.1
SERVER_PORT=8080

# 日志配置
RUST_LOG=info
LOG_LEVEL=info
LOG_WITH_THREAD_IDS=true
LOG_WITH_LINE_NUMBER=true
LOG_WITH_FILE=true
LOG_WITH_TARGET=false
LOG_FILE_PATH=logs/app.log
```

## 架构概览

这是一个使用 Actix-web 构建的 Rust Web API，采用分层架构模式：

### 核心架构层次

- **API 层** (`src/api/`): HTTP 请求处理器和 OpenAPI 文档
- **服务层** (`src/services/`): 业务逻辑和 JWT 认证
- **仓库层** (`src/repositories/`): PostgreSQL 数据访问抽象
- **模型层** (`src/models/`): 数据结构和领域模型

### 关键组件

- **应用状态** (`src/app_state.rs`): 依赖注入容器，包含仓库和服务实例
- **启动器** (`src/startup.rs`): 应用引导过程，包括配置、日志和服务器设置
- **路由** (`src/routes.rs`): 集中的路由配置，集成 OpenAPI/Swagger
- **中间件系统** (`src/middleware/`): 认证、CORS 和请求日志中间件
- **通用响应系统** (`src/common/`): 统一的错误处理和响应格式化

### 响应架构

所有 API 返回标准化响应，包含：

- `code`: 业务错误代码（1000-6999 范围）
- `status`: 状态字符串表示
- `message`: 人类可读消息（默认中文）
- `data`: 响应数据（可选）
- `timestamp`: Unix 时间戳

错误代码范围：

- 1000-1999: 通用错误
- 2000-2999: 认证错误
- 3000-3999: 用户错误
- 4000-4999: 套餐错误
- 5000-5999: 优惠券错误
- 6000-6999: 订单错误

### 数据库架构

- PostgreSQL 配合 SQLx 进行异步数据库操作
- 连接池（最大 5 个连接）
- 表名前缀为 `purple_`（user, plan, coupon, order）
- 迁移脚本位于 `migrations/` 目录

### 日志系统

双输出日志：

- 控制台：开发时的彩色输出
- 文件：`logs/` 目录中的每日轮转日志

## 添加新功能

1. **定义模型**: 在 `src/models/` 中添加数据结构
2. **创建仓库**: 在 `src/repositories/` 中实现数据访问
3. **添加服务**: 在 `src/services/` 中实现业务逻辑
4. **创建 API 处理器**: 在 `src/api/` 中添加 HTTP 处理器
5. **注册路由**: 在 `src/routes.rs` 中更新新端点
6. **使用响应系统**: 利用 `common/response.rs` 实现一致的响应

## 测试

测试位于 `tests/` 目录：

- 仓库测试使用模拟依赖
- 使用 `tokio-test` 进行异步测试
- 使用 `mockall` 模拟外部依赖

## API 文档

- Swagger UI 地址: `/swagger-ui/`
- OpenAPI 规范: `/api-docs/openapi.json`
- 健康检查: `/health`
- 所有业务 API 在 `/api/` 前缀下

## OpenAPI/Swagger 文档规则

### OpenAPI Schema 引用的关键规则

在使用 OpenAPI 注解（`#[utoipa::path]`）时，**绝对不要**在 `body =` 参数中使用以下问题类型：

#### ❌ 禁止使用的类型

- **泛型类型**: `ApiResponse<T>`、`PageResponse<T>` 等
  - 问题：`T` 参数会创建 "/components/schemas/T does not exist" 错误
- **类型别名**: `Response`、`Response<T>`
  - 问题：别名不是具体的 schema 类型
- **Trait 类型**: `ResponseError`
  - 问题：Trait 无法序列化为 schema

#### ✅ 必须使用的类型

始终在 OpenAPI 注解中使用具体的响应类型：

- `EmptyApiResponse` - 用于空响应/错误响应
- `UserApiResponse` - 用于单用户响应  
- `UserPageApiResponse` - 用于分页用户响应
- `TokenApiResponse` - 用于认证令牌响应
- `UserIdApiResponse` - 用于用户 ID 响应
- `HealthApiResponse` - 用于健康检查响应

### OpenAPI Schema 注册

1. **所有响应类型**在 `body =` 中使用的必须在 `src/api/openapi.rs` 中注册：

   ```rust
   #[derive(OpenApi)]
   #[openapi(
       components(schemas(
           // 在此注册所有响应类型
           EmptyApiResponse,
           UserApiResponse,
           UserPageApiResponse,
           // ... 等等
       ))
   )]
   ```

2. **移除泛型类型的 `#[derive(ToSchema)]`**：

   ```rust
   // ❌ 错误 - 会造成冲突
   #[derive(Debug, Serialize, Deserialize, ToSchema)]
   pub struct ApiResponse<T> { ... }
   
   // ✅ 正确 - 泛型不使用 ToSchema
   #[derive(Debug, Serialize, Deserialize)]
   pub struct ApiResponse<T> { ... }
   ```

### 创建新响应类型

添加新 API 时，创建具体的响应类型而不是使用泛型：

```rust
// 在 src/common/response.rs 中
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourNewApiResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<YourDataType>,
    pub timestamp: i64,
}
```

### 分页响应模式

对于分页数据，创建具体的数据类型：

```rust
// 创建具体的分页数据类型
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourPageData {
    pub items: Vec<YourType>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

// 在具体响应类型中使用
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourPageApiResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
    pub data: Option<YourPageData>,
    pub timestamp: i64,
}
```

## 关键依赖

- **actix-web**: Web 框架
- **sqlx**: 数据库工具包
- **utoipa**: OpenAPI 文档生成
- **jsonwebtoken**: JWT 认证
- **tracing**: 结构化日志
- **validator**: 输入验证
- **serde**: 序列化/反序列化
