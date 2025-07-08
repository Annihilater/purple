# Purple

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Actix-web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs)
[![PostgreSQL](https://img.shields.io/badge/postgresql-12+-green.svg)](https://www.postgresql.org)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](LICENSE)

基于 Rust 和 Actix-web 构建的现代化 Web API 项目，提供用户管理、套餐管理、优惠券系统等完整功能。

## ✨ 特性

- 🚀 **高性能**: 基于 Rust 和 Actix-web，提供极致性能
- 🛡️ **安全可靠**: JWT 认证、参数验证、SQL 注入防护
- 📊 **完整监控**: 请求日志、性能监控、错误追踪
- 📚 **自动文档**: OpenAPI/Swagger 自动生成 API 文档
- 🔧 **易于维护**: 分层架构、统一错误处理、类型安全
- 🐳 **容器化**: Docker 支持，一键部署
- 🔄 **统一响应格式**: 标准化的 RESTful API 响应规范
- ⏱️ **智能监控**: 自动请求耗时统计和性能预警
- 🎯 **路由管理**: 智能路由配置，避免路径冲突
- 📄 **分页支持**: 统一的分页查询和响应格式
- 🔍 **请求追踪**: 每个请求的唯一标识符，便于问题排查
- ⚡ **微秒级监控**: 精确到微秒的响应时间监控

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 12+
- 操作系统: Linux, macOS, Windows

### 安装和运行

1. **克隆项目**
```bash
git clone https://github.com/your-org/purple.git
cd purple
```

2. **配置环境变量**
```bash
cp .env.example .env
# 编辑 .env 文件，配置数据库连接等信息
```

3. **初始化数据库**
```bash
# 创建数据库
createdb purple

# 运行迁移脚本
psql -U username -d purple -f migrations/init.sql
```

4. **启动应用**
```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/purple
```

5. **访问应用**
- API 服务: http://127.0.0.1:8080
- Swagger 文档: http://127.0.0.1:8080/swagger-ui/
- 健康检查: http://127.0.0.1:8080/health

## 📖 文档

### 📚 完整文档目录

```
docs/
├── api/                    # API 接口文档
│   ├── README.md          # API 总览和快速入门
│   └── authentication.md  # 认证接口详解
├── development/           # 开发文档
│   ├── setup.md          # 开发环境搭建
│   ├── architecture.md   # 项目架构说明
│   ├── coding-style.md   # 编码规范
│   └── testing.md        # 测试指南
├── deployment/            # 部署文档
│   ├── docker.md         # Docker 部署
│   ├── production.md     # 生产环境部署
│   └── configuration.md  # 配置说明
└── examples/              # 使用示例
    ├── client-examples.js # 客户端调用示例
    └── postman.json       # Postman 导入文件
```

### 🔗 快速链接

- 📋 [API 接口总览](docs/api/README.md)
- 🔐 [认证接口详解](docs/api/authentication.md)
- 🌐 [在线 Swagger 文档](http://127.0.0.1:8080/swagger-ui/)
- 🛠️ [开发环境搭建](docs/development/setup.md)
- 🏗️ [项目架构说明](docs/development/architecture.md)
- 🐳 [Docker 部署](docs/deployment/docker.md)

## 🏗️ 项目架构

```
src/
├── api/           # HTTP 请求处理器和 OpenAPI 文档
├── services/      # 业务逻辑层
├── repositories/  # 数据访问层
├── models/        # 数据模型和结构体
├── middleware/    # 中间件系统
├── common/        # 通用组件和工具
├── config/        # 配置管理
└── main.rs        # 应用入口
```

### 核心组件

- **API 层**: RESTful API 端点，OpenAPI 文档生成
- **服务层**: 业务逻辑处理，JWT 认证服务
- **仓库层**: PostgreSQL 数据访问，连接池管理
- **中间件系统**: 认证、CORS、请求日志、性能监控
- **通用响应系统**: 统一的错误处理和响应格式

## 🛠️ 主要功能

### 🔐 认证系统
- JWT Token 认证
- 用户注册和登录
- 权限控制和状态管理

### 👥 用户管理
- 用户 CRUD 操作
- 用户状态管理
- 批量操作支持

### 📦 套餐管理
- 套餐配置和定价
- 套餐统计和分析
- 可用性检查

### 🎫 优惠券系统
- 优惠券创建和管理
- 优惠券验证和使用
- 使用限制和有效期控制

### 📡 订阅管理
- 用户订阅信息
- 客户端配置生成
- 流量统计和监控

## 🔧 开发命令

```bash
# 代码检查
cargo check

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码静态分析
cargo clippy

# 生成文档
cargo doc --open

# 数据库迁移
psql -U username -d purple -f migrations/init.sql
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

### 📝 智能日志
- 📥 **请求日志**: 详细记录所有入站请求
- 📤 **响应日志**: 根据状态码使用不同emoji标识
- 🎨 **彩色输出**: 开发环境友好的彩色终端输出
- 📁 **文件归档**: 生产环境的结构化日志文件
- 🔒 **安全过滤**: 自动隐藏敏感信息如Authorization头

### 日志示例
```
📥 请求: GET /health - IP: 127.0.0.1 - User-Agent: curl/8.7.1
⏱️ GET /health 200 - ✅ - IP: 127.0.0.1 - 耗时: 193μs 🚀
📤 响应: GET /health - 状态码: 200 ✅
```

## 🛡️ 安全特性

- 🔐 **JWT 认证**: 无状态的安全认证机制
- ✅ **参数验证**: 自动参数验证和类型检查
- 🛡️ **SQL 注入防护**: 使用 SQLx 参数化查询
- 🔒 **密码安全**: BCrypt 哈希加密存储
- 🚫 **权限控制**: 基于角色的访问控制
- 🔍 **请求追踪**: 每个请求的唯一标识符

## 🌍 环境配置

### 开发环境 (.env)
```env
# 数据库配置
DATABASE_URL=postgresql://purple:purple@localhost:5432/purple

# JWT 配置
JWT_SECRET=your-development-secret-key

# 服务器配置
SERVER_ADDR=127.0.0.1
SERVER_PORT=8080

# 管理员账户
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

### 生产环境
生产环境需要更强的安全配置，请参考 [部署文档](docs/deployment/production.md)。

## 📋 API 快速参考

### 🔓 公开接口（无需认证）
```bash
GET  /                           # 项目信息
GET  /health                     # 健康检查
POST /api/auth/register          # 用户注册
POST /api/auth/login             # 用户登录
GET  /swagger-ui/                # API 文档
GET  /api-docs/openapi.json      # OpenAPI 规范
GET  /coupons/verify/{code}      # 验证优惠码（公开接口）
```

### 🔒 认证接口（需要 JWT）
```bash
# 用户管理
GET    /api/users                # 获取用户列表（分页）
POST   /api/users                # 创建用户
GET    /api/users/{id}           # 获取用户详情
PUT    /api/users/{id}           # 更新用户
DELETE /api/users/{id}           # 删除用户

# 套餐管理
GET    /api/plans                # 获取套餐列表（分页）
POST   /api/plans                # 创建套餐
GET    /api/plans/enabled        # 获取启用套餐
GET    /api/plans/{id}/pricing   # 获取套餐价格

# 优惠券管理
GET    /api/coupons              # 获取优惠券列表（分页）
POST   /api/coupons              # 创建优惠券
GET    /api/coupons/{id}         # 获取优惠券详情
PUT    /api/coupons/{id}         # 更新优惠券
DELETE /api/coupons/{id}         # 删除优惠券

# 订阅管理
GET    /api/subscribe/info       # 获取订阅信息
GET    /api/subscribe/link       # 获取订阅链接
POST   /api/subscribe/reset      # 重置订阅令牌
```

### 🔄 统一响应格式

所有 API 返回统一的响应格式，确保客户端处理的一致性：

#### ✅ 成功响应示例
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "示例数据"
  },
  "meta": {
    "timestamp": 1751938399,
    "request_id": "uuid-here"
  }
}
```

#### ❌ 错误响应示例
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

#### 📄 分页响应示例
```json
{
  "success": true,
  "data": [
    {"id": 1, "name": "项目1"},
    {"id": 2, "name": "项目2"}
  ],
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

### 🔑 认证方式

使用 JWT Bearer Token 进行认证：

```bash
# 1. 登录获取令牌
curl -X POST 'http://127.0.0.1:8080/api/auth/login' \
  -H 'Content-Type: application/json' \
  -d '{
    "username": "admin@test.com",
    "password": "secure_admin_password_123"
  }'

# 响应示例
{
  "success": true,
  "data": {
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
    "token_type": "Bearer",
    "expires_in": 604800
  },
  "meta": {
    "timestamp": 1751938088,
    "request_id": "uuid-here"
  }
}

# 2. 使用令牌访问受保护的接口
curl 'http://127.0.0.1:8080/api/coupons?page=1&page_size=10' \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...'
```

详细 API 文档请查看：[docs/api/README.md](docs/api/README.md)

## 🤝 贡献指南

我们欢迎所有形式的贡献！

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 开发规范

- 遵循 Rust 官方编码规范
- 运行 `cargo fmt` 和 `cargo clippy` 确保代码质量
- 为新功能添加测试
- 更新相关文档

## 📄 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 📞 技术支持

- 📧 **邮箱**: support@purple-project.com
- 🐛 **问题反馈**: [GitHub Issues](https://github.com/your-org/purple/issues)
- 💬 **讨论**: [GitHub Discussions](https://github.com/your-org/purple/discussions)
- 📚 **文档**: [项目文档](docs/)

## 🙏 致谢

感谢以下开源项目：

- [Actix-web](https://actix.rs/) - 高性能的 Rust Web 框架
- [SQLx](https://github.com/launchbadge/sqlx) - 异步 SQL 工具包
- [Tokio](https://tokio.rs/) - 异步运行时
- [Serde](https://serde.rs/) - 序列化和反序列化库
- [Utoipa](https://github.com/juhaku/utoipa) - OpenAPI 文档生成

---

<div align="center">
  <p>⭐ 如果这个项目对您有帮助，请给我们一个星标！</p>
  <p>Built with ❤️ using Rust</p>
</div>