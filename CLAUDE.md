# CLAUDE.md

此文件为 Claude Code (claude.ai/code) 在此代码仓库中工作时提供指导。

## 重要工作规则

**中文回复规则**: 在此代码仓库中工作时，Claude 必须使用中文回复用户的所有问题和请求。所有交流都应使用中文进行。

**Rust 最佳实践规则**: 总是从 Rust 最佳项目实践的方向思考和解决问题，保持整个项目代码风格一致，逻辑自洽和闭合。遵循 Rust 社区的惯用法和设计模式，确保代码质量和可维护性。

## 路由配置重要规则

### 路由路径配置原则

在配置 API 路由时，必须遵循以下原则避免路径重复：

#### ❌ 错误的路由配置
```rust
// 在 routes.rs 中已经定义了 scope
cfg.service(
    web::scope("/api/coupons")
        .service(api::list_coupons)
);

// 在 api/coupon.rs 中又重复定义了完整路径
#[get("/api/coupons")]  // 错误：会导致 /api/coupons/api/coupons
pub async fn list_coupons(...) { ... }
```

#### ✅ 正确的路由配置
```rust
// 在 routes.rs 中定义 scope
cfg.service(
    web::scope("/api/coupons")
        .wrap(Auth::new())
        .service(api::create_coupon)    // POST ""
        .service(api::list_coupons)     // GET ""
        .service(api::get_coupon)       // GET "/{id}"
        .service(api::update_coupon)    // PUT "/{id}"
        .service(api::delete_coupon)    // DELETE "/{id}"
);

// 在 api/coupon.rs 中使用相对路径
#[post("")]           // 对应 POST /api/coupons
#[get("")]            // 对应 GET /api/coupons
#[get("/{id}")]       // 对应 GET /api/coupons/{id}
#[put("/{id}")]       // 对应 PUT /api/coupons/{id}
#[delete("/{id}")]    // 对应 DELETE /api/coupons/{id}
```

### 路由检查清单

在添加新的 API 端点时，请检查：

1. ✅ **Scope 配置**: 在 `src/routes.rs` 中正确配置 scope
2. ✅ **相对路径**: 在处理函数中使用相对路径注解
3. ✅ **导出函数**: 在 `src/api/mod.rs` 中正确导出函数
4. ✅ **认证中间件**: 为需要认证的路由添加 `Auth::new()`
5. ✅ **OpenAPI 注解**: 正确配置 `#[utoipa::path]` 注解

### 常见路由问题排查

如果遇到 404 错误，按以下步骤排查：

1. **检查路由注册**: 确认函数在 `routes.rs` 中正确注册
2. **检查路径配置**: 确认没有路径重复（scope + 注解路径）
3. **检查函数导出**: 确认函数在 `api/mod.rs` 中正确导出
4. **检查编译**: 确认代码编译通过且服务器重启

## 统一响应格式规范

本项目采用标准化的 RESTful API 响应格式，确保所有接口返回数据的一致性和可维护性。

### 核心设计原则

1. **语义明确**: `success` 字段明确表示操作是否成功
2. **类型安全**: 错误代码使用字符串枚举，便于维护和理解
3. **RESTful 兼容**: 配合 HTTP 状态码使用
4. **扩展性**: `meta` 字段可包含时间戳、请求ID等元数据
5. **调试友好**: 可选的 `request_id` 便于问题追踪

### 响应格式标准

#### 成功响应
```json
{
  "success": true,
  "data": { /* 实际数据 */ },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

#### 错误响应
```json
{
  "success": false,
  "error": {
    "code": "USER_NOT_FOUND",
    "message": "用户未找到",
    "details": "详细错误信息（可选）",
    "field": "user_id（字段级验证错误，可选）"
  },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

#### 分页响应
```json
{
  "success": true,
  "data": [ /* 数据数组 */ ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 100,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

### 实现指南

#### 1. 使用新的响应系统
```rust
use crate::common::response_v2::{ApiResponse, IntoHttpResponse};
use crate::common::ErrorCode;

// 成功响应
let response = ApiResponse::success(data);
Ok(response.into_http_response())

// 错误响应 - 使用 with_details 方法
let response = ApiError::with_details(ErrorCode::UserNotFound, "用户不存在".to_string());
Err(response)

// 分页响应 - 直接返回数组和分页信息
let response = ApiResponse::page(items, page as u64, page_size as u64, total as u64);
Ok(response.into_http_response())
```

#### 2. 推荐的响应模式
```rust
// 成功响应
match repo.create(&request).await {
    Ok(item) => {
        let response = ApiResponse::success(ItemResponse::from(item));
        Ok(response.into_http_response())
    }
    Err(e) => {
        tracing::error!("创建失败: {}", e);
        Err(ApiError::with_details(
            ErrorCode::DatabaseError,
            "数据库操作失败".to_string(),
        ))
    }
}

// 分页响应
match repo.list(page, page_size, filters).await {
    Ok((items, total)) => {
        let items = items.into_iter().map(ItemResponse::from).collect();
        let response = ApiResponse::page(items, page as u64, page_size as u64, total as u64);
        Ok(response.into_http_response())
    }
    Err(e) => {
        tracing::error!("查询失败: {}", e);
        Err(ApiError::with_details(
            ErrorCode::DatabaseError,
            "数据库操作失败".to_string(),
        ))
    }
}
```

#### 3. 错误处理最佳实践
```rust
// 标准错误处理模式
pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    service: web::Data<AuthService>,
) -> Result<HttpResponse, ApiError> {
    // 输入验证
    if let Err(validation_errors) = user.validate() {
        return Err(ApiError::from(validation_errors));
    }

    // 业务逻辑处理
    match service.create(&user.into_inner()).await {
        Ok(user) => {
            let response = ApiResponse::success(UserResponse::from(user));
            Ok(response.into_http_response())
        }
        Err(e) => {
            tracing::error!("创建用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::InternalError,
                "注册失败，请稍后重试".to_string(),
            ))
        }
    }
}

// 查找资源错误处理
pub async fn get_user(
    id: web::Path<i32>,
    repo: web::Data<UserRepository>,
) -> Result<HttpResponse, ApiError> {
    let user_id = id.into_inner();

    match repo.find_by_id(user_id).await {
        Ok(Some(user)) => {
            let response = ApiResponse::success(UserResponse::from(user));
            Ok(response.into_http_response())
        }
        Ok(None) => Err(ApiError::new(ErrorCode::UserNotFound)),
        Err(e) => {
            tracing::error!("查找用户失败: {}", e);
            Err(ApiError::with_details(
                ErrorCode::DatabaseError,
                "数据库操作失败".to_string(),
            ))
        }
    }
}
```

### 错误代码规范

错误代码采用分类管理，便于维护和扩展：

- **1000-1999**: 通用错误
- **2000-2999**: 认证相关错误  
- **3000-3999**: 用户相关错误
- **4000-4999**: 套餐相关错误
- **5000-5999**: 优惠券相关错误
- **6000-6999**: 订单相关错误

### HTTP 状态码映射

响应系统会自动根据错误类型映射相应的 HTTP 状态码：

- 成功响应: `200 OK`
- 认证错误: `401 Unauthorized` 或 `403 Forbidden`
- 客户端错误: `400 Bad Request`
- 资源不存在: `404 Not Found`
- 服务器错误: `500 Internal Server Error`

### 响应格式验证示例

#### 认证失败场景 (401)
```bash
curl 'http://127.0.0.1:8080/api/coupons'
```
返回：
```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "未授权访问",
    "details": "缺少授权令牌"
  },
  "meta": {
    "timestamp": 1751937981,
    "request_id": "uuid-here"
  }
}
```

#### 认证成功但无数据场景 (200)
```bash
curl 'http://127.0.0.1:8080/api/coupons?page=1&page_size=10' \
  -H 'Authorization: Bearer valid-token'
```
返回：
```json
{
  "success": true,
  "data": [],
  "pagination": {
    "page": 1,
    "page_size": 10,
    "total": 0,
    "total_pages": 0,
    "has_next": false,
    "has_prev": false
  },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

#### 认证成功且有数据场景 (200)
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "code": "WELCOME10",
      "name": "欢迎优惠券",
      "type": false,
      "value": 1000
    }
  ],
  "pagination": {
    "page": 1,
    "page_size": 10,
    "total": 1,
    "total_pages": 1,
    "has_next": false,
    "has_prev": false
  },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

### 迁移指南

对于现有接口，建议渐进式迁移：

1. 新接口直接使用新的响应格式
2. 现有接口可以保持兼容性，逐步迁移
3. 重要接口优先迁移
4. 统一更新 OpenAPI 文档

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

# 管理员账户配置（启动时自动创建/更新）
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=admin123

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
