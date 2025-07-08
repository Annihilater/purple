# Purple - 全栈 Rust Web 应用

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Actix-web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs)
[![Leptos](https://img.shields.io/badge/leptos-0.6-purple.svg)](https://leptos.dev)
[![PostgreSQL](https://img.shields.io/badge/postgresql-12+-green.svg)](https://www.postgresql.org)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](LICENSE)

Purple 是一个基于 Rust 构建的现代化全栈 Web 应用，采用 Workspace 架构，包含后端 API、前端 Web 应用和共享库。

## 🏗️ 架构概览

```
purple/
├── Cargo.toml          # Workspace 配置
├── backend/            # Actix-web API 后端
│   ├── src/           # 后端源代码
│   ├── migrations/    # 数据库迁移
│   └── tests/         # 后端测试
├── frontend/           # Leptos WASM 前端
│   ├── src/           # 前端源代码
│   └── index.html     # 入口页面
├── shared/             # 前后端共享类型库
│   └── src/           # 共享类型定义
├── build.sh           # 统一构建脚本
├── dev.sh             # 开发环境脚本
└── docs/              # 项目文档
```

### 🛠️ 技术栈

#### 后端 (Backend)
- **框架**: Rust + Actix-web
- **数据库**: PostgreSQL + SQLx
- **认证**: JWT Bearer Token
- **文档**: OpenAPI/Swagger
- **日志**: Tracing + 结构化日志

#### 前端 (Frontend)  
- **框架**: Rust + Leptos
- **编译**: WebAssembly (WASM)
- **样式**: Tailwind CSS
- **状态管理**: Leptos Signals
- **路由**: Leptos Router

#### 共享库 (Shared)
- **类型安全**: 统一的 API 契约
- **序列化**: Serde
- **验证**: Validator
- **错误处理**: 标准化错误码

### ✨ 特性

- 🚀 **高性能**: Rust + WebAssembly 极致性能
- 🔒 **类型安全**: 前后端共享类型定义
- 🛡️ **安全可靠**: JWT 认证、参数验证、SQL 注入防护
- 📊 **完整监控**: 请求日志、性能监控、错误追踪
- 📚 **自动文档**: OpenAPI/Swagger 自动生成 API 文档
- 🐳 **容器化**: Docker 支持，一键部署
- 🔄 **统一响应**: 标准化的 RESTful API 响应格式
- ⚡ **实时编译**: 前端热重载开发体验

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 13+
- Node.js 18+ (可选，用于其他工具)

### 安装开发依赖

```bash
# 安装 Rust 前端工具
./dev.sh deps

# 或手动安装
cargo install trunk wasm-pack cargo-watch
```

### 配置环境

```bash
# 复制环境配置
cp backend/.env.example backend/.env

# 编辑数据库配置
vim backend/.env
```

示例配置 (`backend/.env`)：
```env
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

### 数据库设置

```bash
# 创建数据库
createdb purple

# 运行迁移
./dev.sh db
```

### 启动开发环境

```bash
# 启动完整开发环境（前端+后端）
./dev.sh both

# 或分别启动
./dev.sh backend    # 后端 (http://localhost:8080)
./dev.sh frontend   # 前端 (http://localhost:8000)
```

访问应用：
- 🔧 **后端 API**: http://localhost:8080
- 🎨 **前端应用**: http://localhost:8000  
- 📚 **API 文档**: http://localhost:8080/swagger-ui/
- ❤️ **健康检查**: http://localhost:8080/health

## 📦 构建和部署

### 开发构建

```bash
# 检查编译
./build.sh check

# 完整构建
./build.sh all

# 单独构建
./build.sh backend   # 构建后端
./build.sh frontend  # 构建前端  
./build.sh shared    # 构建共享库
```

### 生产构建

```bash
# 生产环境构建
cargo build --release --workspace

# 前端生产构建
cd frontend && trunk build --release
```

### Docker 部署

```bash
# 构建镜像
docker build -t purple-app .

# 运行容器
docker-compose up -d
```

## 🔧 开发指南

### Workspace 结构详解

Purple 采用 Cargo Workspace 管理三个相关的 crate：

#### Backend (`/backend`) - API 服务
```
backend/
├── src/
│   ├── api/           # HTTP 端点处理器
│   ├── services/      # 业务逻辑层
│   ├── repositories/  # 数据访问层
│   ├── models/        # 数据模型
│   ├── middleware/    # 中间件
│   ├── common/        # 通用组件
│   ├── config/        # 配置管理
│   ├── lib.rs         # 库入口
│   └── main.rs        # 应用入口
├── migrations/        # 数据库迁移
└── tests/            # 测试文件
```

#### Frontend (`/frontend`) - Web 应用
```
frontend/
├── src/
│   ├── components/    # UI 组件
│   ├── pages/         # 页面组件
│   ├── services/      # API 客户端
│   ├── utils/         # 工具函数
│   └── lib.rs         # 应用入口
├── index.html         # HTML 模板
└── dist/             # 构建输出
```

#### Shared (`/shared`) - 共享库
```
shared/
└── src/
    └── lib.rs         # 类型定义和工具
```

### API 响应格式

所有 API 遵循统一的响应格式：

```json
{
  "success": true,
  "data": { "id": 1, "name": "示例" },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

**分页响应**：
```json
{
  "success": true,
  "data": [{"id": 1}, {"id": 2}],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 100,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

**错误响应**：
```json
{
  "success": false,
  "error": {
    "code": "USER_NOT_FOUND",
    "message": "用户未找到",
    "details": "用户ID 123 不存在"
  },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

### 添加新功能

1. **定义共享类型** (`shared/src/lib.rs`)
```rust
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, message = "任务名称不能为空"))]
    pub name: String,
    pub description: Option<String>,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
}
```

2. **实现后端 API** (`backend/src/api/task.rs`)
```rust
use purple_shared::{CreateTaskRequest, Task};
use crate::common::response_v2::{ApiResponse, IntoHttpResponse, ApiError};

#[utoipa::path(
    post,
    path = "",
    tag = "tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 200, description = "创建任务成功", body = TaskApiResponse)
    )
)]
#[post("")]
pub async fn create_task(
    request: web::Json<CreateTaskRequest>,
    // 注入仓库依赖...
) -> Result<HttpResponse, ApiError> {
    if let Err(validation_errors) = request.validate() {
        return Err(ApiError::from(validation_errors));
    }
    
    // 业务逻辑实现...
    let task = task_service.create(request.into_inner()).await?;
    
    let response = ApiResponse::success(task);
    Ok(response.into_http_response())
}
```

3. **创建前端页面** (`frontend/src/pages/tasks.rs`)
```rust
use leptos::*;
use purple_shared::{CreateTaskRequest, Task};
use crate::services::api::ApiClient;

#[component]
pub fn TasksPage() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(Vec::<Task>::new());
    
    // 获取任务列表
    let load_tasks = create_action(|_| async move {
        match ApiClient::get::<Vec<Task>>("/api/tasks").await {
            Ok(response) if response.success => {
                if let Some(data) = response.data {
                    set_tasks.set(data);
                }
            }
            _ => {
                // 错误处理
            }
        }
    });
    
    view! {
        <div class="container mx-auto p-6">
            <h1 class="text-2xl font-bold mb-6">"任务管理"</h1>
            
            <div class="grid gap-4">
                <For
                    each=tasks
                    key=|task| task.id
                    children=move |task| {
                        view! {
                            <div class="bg-white p-4 rounded-lg shadow">
                                <h3 class="font-semibold">{&task.name}</h3>
                                <p class="text-gray-600">{&task.description}</p>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
```

## 📚 API 文档

### 🔓 公开接口（无需认证）
```bash
GET  /                           # 项目信息
GET  /health                     # 健康检查  
POST /api/auth/register          # 用户注册
POST /api/auth/login             # 用户登录
GET  /swagger-ui/                # API 文档
GET  /api-docs/openapi.json      # OpenAPI 规范
```

### 🔒 认证接口（需要 JWT）
```bash
# 用户管理
GET    /api/users                # 获取用户列表
POST   /api/users                # 创建用户
GET    /api/users/{id}           # 获取用户详情
PUT    /api/users/{id}           # 更新用户
DELETE /api/users/{id}           # 删除用户

# 套餐管理  
GET    /api/plans                # 获取套餐列表
POST   /api/plans                # 创建套餐
GET    /api/plans/enabled        # 获取启用套餐

# 优惠券管理
GET    /api/coupons              # 获取优惠券列表
POST   /api/coupons              # 创建优惠券
PUT    /api/coupons/{id}         # 更新优惠券
DELETE /api/coupons/{id}         # 删除优惠券
```

### 🔑 认证方式

```bash
# 1. 登录获取令牌
curl -X POST 'http://localhost:8080/api/auth/login' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "admin@example.com",
    "password": "admin123"
  }'

# 2. 使用令牌访问 API
curl 'http://localhost:8080/api/users' \
  -H 'Authorization: Bearer YOUR_JWT_TOKEN'
```

详细 API 文档: http://localhost:8080/swagger-ui/

## 🧪 测试

```bash
# 运行所有测试
./build.sh test

# 后端测试
cd backend && cargo test

# 前端测试（如果有）
cd frontend && wasm-pack test --node

# 代码覆盖率
cargo tarpaulin --workspace
```

## 📊 监控和日志

### 🎯 性能监控
- ⚡ **微秒级精度**: 请求响应时间精确到微秒
- 📈 **智能预警**: 根据响应时间自动调整日志级别
- 🔍 **详细追踪**: 每个请求都有唯一的 request_id
- 📊 **性能分级**: 
  - 🚀 0-100ms: 快速响应
  - ⚡ 101-500ms: 正常响应  
  - 🐢 501-1000ms: 较慢响应 (警告)
  - 🦕 1001-5000ms: 慢响应 (警告)
  - 🐌 >5000ms: 非常慢 (错误)

### 📝 日志系统
- **控制台**: 彩色结构化日志输出
- **文件**: `backend/logs/app.log.*` (按日轮转)
- **格式**: JSON 结构化日志
- **安全**: 自动过滤敏感信息

示例日志：
```
📥 请求: GET /health - IP: 127.0.0.1 - User-Agent: curl/8.7.1
⏱️ GET /health 200 - ✅ - IP: 127.0.0.1 - 耗时: 193μs 🚀
📤 响应: GET /health - 状态码: 200 ✅
```

## 🔒 安全性

- **认证**: JWT Bearer Token
- **授权**: 基于角色的访问控制
- **输入验证**: Serde + Validator
- **SQL 注入防护**: SQLx 参数化查询
- **CORS**: 可配置的跨域策略
- **密码安全**: Argon2 哈希加密

## 🐳 Docker 支持

```yaml
# docker-compose.yml
version: '3.8'
services:
  purple-backend:
    build: ./backend
    ports:
      - "8080:8080"
    depends_on:
      - postgres
    environment:
      - DATABASE_URL=postgresql://purple:purple@postgres:5432/purple
      
  purple-frontend:
    build: ./frontend
    ports:
      - "8000:8000"
      
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: purple
      POSTGRES_USER: purple
      POSTGRES_PASSWORD: purple
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## 🤝 贡献指南

我们欢迎所有形式的贡献！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 开发规范

- 遵循 Rust 官方编码规范
- 运行 `cargo fmt` 和 `cargo clippy` 确保代码质量
- 为新功能添加测试和文档
- 保持向后兼容性

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🆘 获取帮助

- **GitHub Issues**: 报告 Bug 和功能请求
- **Discussions**: 社区讨论和问答
- **Wiki**: 详细文档和教程
- **Email**: support@purple-project.com

## 🎯 路线图

- [x] 基础 Workspace 架构
- [x] 后端 API 框架 (Actix-web)
- [x] 前端 UI 框架 (Leptos)
- [x] 共享类型系统
- [x] 统一响应格式
- [x] JWT 认证系统
- [ ] 实时通信 (WebSocket)
- [ ] 性能监控面板
- [ ] 多语言支持 (i18n)
- [ ] 移动端适配 (PWA)
- [ ] GraphQL 支持
- [ ] 微服务架构支持

## 🙏 致谢

感谢以下开源项目：

- [Actix-web](https://actix.rs/) - 高性能的 Rust Web 框架
- [Leptos](https://leptos.dev/) - 现代 Rust 前端框架
- [SQLx](https://github.com/launchbadge/sqlx) - 异步 SQL 工具包
- [Tokio](https://tokio.rs/) - 异步运行时
- [Serde](https://serde.rs/) - 序列化和反序列化库
- [Utoipa](https://github.com/juhaku/utoipa) - OpenAPI 文档生成

---

<div align="center">
  <p>⭐ 如果这个项目对您有帮助，请给我们一个星标！</p>
  <p>🦀 Built with ❤️ using Rust + WebAssembly</p>
  <p><strong>Purple</strong> - 让 Rust 全栈开发更简单！ ✨</p>
</div>

# Purple 项目

Purple 是一个完整的SaaS系统，包含后端API服务、用户前端和管理员前端。

## 项目结构

- **backend/**: 后端API服务，基于Actix-Web和PostgreSQL
- **user-frontend/**: 用户界面，基于Leptos框架的WebAssembly前端
- **admin-frontend/**: 管理员界面，基于Leptos框架的WebAssembly前端
- **shared/**: 前后端共享代码和类型定义
- **docs/**: 项目文档

## 开发环境设置

### 后端

```bash
cd backend
cargo run
```

后端服务默认运行在 `http://127.0.0.1:8080`

### 用户前端

```bash
cd user-frontend
trunk serve --open
```

### 管理员前端

```bash
cd admin-frontend
trunk serve --open
```

## 部署

请参考 `docs/deployment/` 目录下的文档了解部署详情。

## 文档

- API文档: `docs/api/`
- 开发指南: `docs/development/`
- 部署指南: `docs/deployment/`
- 示例代码: `docs/examples/`

## 许可证

MIT