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
│   ├── middleware.rs # 中间件
│   ├── openapi.rs    # OpenAPI文档配置
│   └── response.rs   # 响应结构体
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
│   ├── auth_service.rs # 认证服务
│   └── auth.rs       # 认证逻辑
└── utils/            # 工具函数
```

## 技术栈

- **Web框架**: Actix-web 4.x
- **数据库**: PostgreSQL + SQLx
- **认证**: JWT (jsonwebtoken)
- **日志**: tracing + tracing-subscriber
- **文档**: OpenAPI 3.0 + Swagger UI
- **序列化**: Serde
- **配置**: config + dotenv
- **异步运行时**: Tokio

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

# 日志配置
RUST_LOG=info
LOG_LEVEL=info
LOG_FILE_PATH=logs/app.log
```

### 4. 初始化数据库

```bash
# 运行数据库迁移脚本
psql -U username -d purple -f migrations/init.sql
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

## 配置说明

### 环境变量配置

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| `DATABASE_URL` | PostgreSQL数据库连接URL | 无 |
| `JWT_SECRET` | JWT签名密钥 | 无 |
| `SERVER_ADDR` | 服务器监听地址 | 127.0.0.1 |
| `SERVER_PORT` | 服务器端口 | 8080 |
| `RUST_LOG` | 日志级别 | info |
| `LOG_LEVEL` | 应用日志级别 | info |
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

### 代码结构

本项目采用模块化架构，遵循Rust最佳实践：

1. **单一职责原则**: 每个模块只负责特定功能
2. **依赖注入**: 通过应用状态管理依赖
3. **错误处理**: 使用`Result<T, E>`进行显式错误处理
4. **类型安全**: 利用Rust类型系统确保编译时安全

### 添加新功能

1. 在`models/`中定义数据模型
2. 在`repositories/`中实现数据访问层
3. 在`services/`中实现业务逻辑
4. 在`api/`中实现API处理器
5. 在`routes.rs`中注册路由

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

## 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解更多信息。

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 技术支持

如果遇到问题，请通过以下方式寻求帮助：

1. 查看项目文档
2. 搜索已有的Issues
3. 创建新的Issue详细描述问题
4. 联系项目维护者

---

**注意**: 在生产环境中使用前，请确保更改默认的JWT密钥和其他敏感配置。
