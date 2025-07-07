# Purple

一个使用Rust和Actix-web框架构建的现代化Web API应用，采用模块化架构设计。

## 功能特性

- 🚀 基于Actix-web的高性能Web服务器
- 📊 完整的用户、套餐、优惠券管理API
- 🔐 JWT认证和授权系统
- 📝 OpenAPI 3.0规范和Swagger UI文档
- 🗃️ PostgreSQL数据库支持
- 📋 结构化日志系统（终端+文件双输出）
- 🏗️ 模块化架构设计
- ⚙️ 灵活的环境配置管理
- 🔍 健康检查接口
- 🛡️ 完善的中间件系统（认证、CORS、日志）
- 📦 统一的响应格式和错误处理

## 项目架构

```
src/
├── main.rs           # 应用入口点
├── startup.rs        # 应用启动器
├── app_state.rs      # 应用状态管理
├── routes.rs         # 路由配置
├── logging.rs        # 日志系统
├── api/              # API路由和处理器
│   ├── auth.rs       # 认证相关API
│   ├── user.rs       # 用户管理API
│   ├── plan.rs       # 套餐管理API
│   ├── coupon.rs     # 优惠券管理API
│   ├── health.rs     # 健康检查API
│   ├── openapi.rs    # OpenAPI文档配置
│   └── response.rs   # 响应结构体（已弃用）
├── middleware/       # 中间件模块
│   ├── auth.rs       # 认证中间件
│   ├── cors.rs       # CORS中间件
│   ├── logging.rs    # 请求日志中间件
│   └── mod.rs        # 中间件模块声明
├── common/           # 通用组件
│   ├── error.rs      # 错误代码定义
│   ├── response.rs   # 通用响应结构
│   ├── status.rs     # 状态码映射
│   └── mod.rs        # 通用模块声明
├── config/           # 配置管理
│   ├── database.rs   # 数据库配置
│   └── mod.rs        # 应用配置
├── models/           # 数据模型
│   ├── user.rs       # 用户模型
│   ├── plan.rs       # 套餐模型
│   ├── coupon.rs     # 优惠券模型
│   └── auth.rs       # 认证模型
├── repositories/     # 数据访问层
│   ├── user_repository.rs    # 用户数据访问
│   ├── plan_repository.rs    # 套餐数据访问
│   └── coupon_repository.rs  # 优惠券数据访问
├── services/         # 业务逻辑服务
│   └── auth.rs       # 认证服务
└── utils/            # 工具函数
```

## 架构设计原则

### 分层架构

- **API层**: 处理HTTP请求和响应
- **服务层**: 业务逻辑处理
- **仓库层**: 数据访问抽象
- **模型层**: 数据结构定义

### 中间件系统

- **认证中间件**: JWT token验证和用户身份确认
- **CORS中间件**: 跨域资源共享配置
- **日志中间件**: 请求/响应日志记录

### 通用响应系统

- **统一错误代码**: 业务相关的标准化错误代码
- **标准响应格式**: 包含状态码、消息、数据和时间戳
- **分页响应**: 统一的分页数据结构
- **响应构建器**: 便捷的响应构建工具

## 技术栈

- **Web框架**: Actix-web 4.x
- **数据库**: PostgreSQL + SQLx
- **认证**: JWT (jsonwebtoken)
- **日志**: tracing + tracing-subscriber + tracing-appender
- **文档**: OpenAPI 3.0 + Swagger UI (utoipa)
- **序列化**: Serde
- **配置**: config + dotenv
- **异步运行时**: Tokio
- **CORS**: actix-cors

## 开发环境要求

- Rust 1.70.0 或更高版本
- Cargo包管理器
- PostgreSQL 12+ 数据库

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/yourusername/purple.git
cd purple
```

### 2. 创建环境配置文件

```bash
cp .env.example .env
```

### 3. 配置环境变量

编辑 `.env` 文件设置以下必要配置：

```env
# 数据库连接
DATABASE_URL=postgresql://username:password@localhost:5432/purple

# JWT秘钥
JWT_SECRET=your-secret-key-here-please-change-in-production

# 服务器配置
SERVER_ADDR=127.0.0.1
SERVER_PORT=8080

# 管理员账户配置（首次启动自动创建）
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=admin123456

# 日志配置
RUST_LOG=info
LOG_LEVEL=info
LOG_WITH_THREAD_IDS=true
LOG_WITH_LINE_NUMBER=true
LOG_WITH_FILE=true
LOG_WITH_TARGET=false
LOG_FILE_PATH=logs/app.log
```

**重要提示**:

- `ADMIN_EMAIL` 和 `ADMIN_PASSWORD` 用于在应用启动时自动创建管理员账户
- 如果管理员账户已存在但密码不匹配，系统会自动更新为配置的密码
- 生产环境中请务必修改默认的管理员邮箱和密码

### 4. 初始化数据库

```bash
# 运行数据库迁移脚本
psql -U username -d purple -f migrations/init.sql

# 修复字段长度限制（如果遇到管理员创建失败）
psql -U username -d purple -f migrations/fix_user_fields.sql
```

### 5. 运行项目

```bash
cargo run
```

### 6. 验证服务

```bash
# 健康检查
curl http://localhost:8080/health

# 查看API文档
open http://localhost:8080/swagger-ui/
```

## API文档

启动应用后，可以通过以下地址访问完整的API文档：

- **Swagger UI**: `http://localhost:8080/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

### 主要API端点

- `POST /register` - 用户注册
- `POST /login` - 用户登录
- `GET /health` - 健康检查
- `GET /api/users` - 获取用户列表
- `GET /api/plans` - 获取套餐列表
- `GET /api/coupons` - 获取优惠券列表

### 响应格式

所有API响应都遵循统一的格式：

```json
{
  "code": 1000,
  "status": "Success",
  "message": "操作成功",
  "data": { ... },
  "timestamp": 1640995200
}
```

### 错误代码系统

应用使用标准化的错误代码系统：

- **1000-1999**: 通用错误（成功、内部错误、参数错误等）
- **2000-2999**: 认证相关错误
- **3000-3999**: 用户相关错误
- **4000-4999**: 套餐相关错误
- **5000-5999**: 优惠券相关错误
- **6000-6999**: 订单相关错误

## 配置说明

### 环境变量配置

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| `DATABASE_URL` | PostgreSQL数据库连接URL | 无 |
| `JWT_SECRET` | JWT签名密钥 | 无 |
| `SERVER_ADDR` | 服务器监听地址 | 127.0.0.1 |
| `SERVER_PORT` | 服务器端口 | 8080 |
| `ADMIN_EMAIL` | 管理员邮箱地址 | <admin@example.com> |
| `ADMIN_PASSWORD` | 管理员密码 | admin123 |
| `RUST_LOG` | 日志级别 | info |
| `LOG_LEVEL` | 应用日志级别 | info |
| `LOG_WITH_THREAD_IDS` | 是否显示线程ID | true |
| `LOG_WITH_LINE_NUMBER` | 是否显示行号 | true |
| `LOG_WITH_FILE` | 是否显示文件名 | true |
| `LOG_WITH_TARGET` | 是否显示目标模块 | false |
| `LOG_FILE_PATH` | 日志文件路径 | logs/app.log |

### 日志配置

应用支持双输出日志系统：

- **终端输出**: 彩色格式，便于开发调试
- **文件输出**: 纯文本格式，支持按日期轮转

日志文件格式：`logs/app.log.YYYY-MM-DD`

### 数据库配置

应用使用PostgreSQL作为主数据库，主要数据表：

- `purple_user` - 用户表
- `purple_plan` - 套餐表
- `purple_coupon` - 优惠券表
- `purple_order` - 订单表
- 以及其他业务相关表

## 开发指南

### 代码结构原则

本项目采用模块化架构，遵循Rust最佳实践：

1. **单一职责原则**: 每个模块只负责特定功能
2. **依赖注入**: 通过应用状态管理依赖
3. **错误处理**: 使用`Result<T, E>`进行显式错误处理
4. **类型安全**: 利用Rust类型系统确保编译时安全
5. **异步处理**: 全面使用async/await进行异步编程

### 添加新功能

1. 在`models/`中定义数据模型
2. 在`repositories/`中实现数据访问层
3. 在`services/`中实现业务逻辑
4. 在`api/`中实现API处理器
5. 在`routes.rs`中注册路由
6. 使用`common/`模块的响应系统

### 中间件开发

在`src/middleware/`目录下添加新的中间件：

```rust
// src/middleware/my_middleware.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

pub struct MyMiddleware;

impl<S, B> Transform<S, ServiceRequest> for MyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // 实现中间件逻辑
}
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test test_name
```

### 代码检查

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 构建检查
cargo check
```

## 数据库修复

如果遇到管理员账户创建失败的错误（如 "value too long for type character varying(64)"），需要运行数据库修复脚本：

```bash
# 运行修复脚本
psql -U username -d purple -f migrations/fix_user_fields.sql
```

该脚本会：

- 将 `email` 字段长度从 64 字符扩展到 255 字符
- 将 `password` 字段长度从 64 字符扩展到 255 字符（支持 Argon2 哈希）
- 添加缺失的 `token`、`created_at`、`updated_at` 字段

修复完成后重新启动应用即可。

## 管理员账户配置

### 自动初始化

应用支持在启动时自动创建和管理管理员账户：

1. **首次启动**: 如果配置的管理员邮箱不存在，系统会自动创建管理员账户
2. **密码同步**: 如果管理员账户已存在但密码与配置不符，系统会自动更新密码
3. **安全特性**: 使用 Argon2 算法进行密码哈希，确保安全性

### 配置示例

在 `.env` 文件中配置管理员账户：

```env
# 管理员账户配置
ADMIN_EMAIL=admin@yourdomain.com
ADMIN_PASSWORD=secure_admin_password_123
```

### 注意事项

- **生产环境**: 务必使用强密码，建议包含大小写字母、数字和特殊字符
- **密码长度**: 建议密码长度至少 12 位
- **定期更换**: 生产环境建议定期更换管理员密码
- **邮箱格式**: 必须是有效的邮箱格式
- **启动日志**: 管理员账户的创建/更新过程会在启动日志中记录

### 启动日志示例

```
INFO src/config/admin.rs:16: 开始初始化管理员账户...
INFO src/config/admin.rs:40: 未发现管理员账户，正在创建新的管理员账户...
INFO src/config/admin.rs:42: 管理员账户创建成功: admin@example.com
```

## 部署

### 生产环境构建

```bash
cargo build --release
```

### Docker部署

```dockerfile
# Dockerfile示例
FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/purple .
EXPOSE 8080
CMD ["./purple"]
```

### 环境变量配置

生产环境需要设置的关键环境变量：

```bash
# 必须修改的配置
export JWT_SECRET="your-production-secret-key"
export DATABASE_URL="postgresql://user:pass@host:5432/db"
export ADMIN_EMAIL="admin@yourdomain.com"
export ADMIN_PASSWORD="your-secure-admin-password"
export RUST_LOG="warn"
export LOG_LEVEL="warn"
```

**管理员账户说明**:

- 应用启动时会自动检查并创建管理员账户
- 如果配置的管理员邮箱已存在但密码不匹配，会自动更新密码
- 生产环境中务必使用强密码并定期更换

## 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解更多信息。

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码贡献规范

- 遵循项目的代码风格和架构原则
- 添加适当的单元测试
- 更新相关文档
- 确保所有测试通过
- 提供清晰的提交信息

## 技术支持

如果遇到问题，请通过以下方式寻求帮助：

1. 查看项目文档
2. 搜索已有的Issues
3. 创建新的Issue详细描述问题
4. 联系项目维护者

## 更新日志

### v0.1.0 (Current)

- 完成基础架构设计
- 实现用户、套餐、优惠券管理API
- 添加JWT认证系统
- 实现OpenAPI文档
- 完成中间件系统重构
- 添加通用响应系统
- 实现双输出日志系统

---

**注意**: 在生产环境中使用前，请确保更改默认的JWT密钥和其他敏感配置。
