# Purple 项目

Purple 是一个现代化全栈 SaaS 应用系统，采用 Rust 语言构建，包含后端 API 服务、用户前端和管理员前端。

## 项目架构

```
purple/
├── backend/            # 后端 API 服务 (Actix-web)
│   ├── src/           # 后端源代码
│   │   ├── api/       # API 控制器
│   │   ├── models/    # 数据模型
│   │   ├── services/  # 业务逻辑
│   │   ├── repositories/ # 数据访问层
│   │   └── ...        # 其他模块
│   ├── migrations/    # 数据库迁移文件
│   └── tests/         # 单元测试
├── admin-frontend/     # 管理员前端 (Leptos + WASM)
│   ├── src/           # 前端源代码
│   │   ├── components/ # UI 组件
│   │   ├── pages/     # 页面
│   │   └── services/  # API 客户端
│   └── dist/          # 编译产物 (已忽略)
├── user-frontend/      # 用户前端 (Leptos + WASM)
│   ├── src/           # 前端源代码
│   │   ├── components/ # UI 组件
│   │   ├── pages/     # 页面
│   │   └── services/  # API 客户端
│   └── dist/          # 编译产物 (已忽略)
└── shared/            # 前后端共享代码
    └── src/           # 共享数据类型和工具
```

## 技术栈

- **后端**: Rust + Actix-web + PostgreSQL + SQLx
- **前端**: Rust + Leptos + WebAssembly
- **认证**: JWT Bearer Token
- **共享**: 类型安全的 API 契约

## 功能特性

- 用户认证与授权
- 管理员系统：用户管理、套餐管理、优惠券管理等
- 用户系统：用户信息查看、服务订阅等
- 前后端共享的类型系统，保证类型安全
- 统一的 API 响应格式

## 开发指南

### 环境准备

- Rust 1.70+
- PostgreSQL 13+
- Trunk (WASM 构建工具)

### 开发环境启动

```bash
# 启动所有服务（后端、管理员前端、用户前端）
./dev.sh

# 或分别启动
cd backend && cargo run
cd admin-frontend && trunk serve --port 8081
cd user-frontend && trunk serve --port 8082
```

访问地址:

- 后端 API: <http://localhost:8080>
- 管理员前端: <http://localhost:8081>
- 用户前端: <http://localhost:8082>

### 构建部署

```bash
# 后端构建
cd backend && cargo build --release

# 管理员前端构建
cd admin-frontend && trunk build --release

# 用户前端构建
cd user-frontend && trunk build --release
```

## 目录说明

### 后端

- `api/`: API 控制器，处理 HTTP 请求
- `models/`: 数据库模型定义
- `services/`: 业务逻辑层
- `repositories/`: 数据库访问层
- `middleware/`: 中间件（认证、日志等）
- `common/`: 通用工具和类型

### 前端

- `components/`: UI 组件
- `pages/`: 页面组件
- `services/`: API 客户端和服务
- `utils/`: 工具函数

### 共享库

- 定义前后端共享的数据类型
- 提供验证规则
- 错误码和错误处理

## 文档

更多详细文档请参考：

- 开发指南: `docs/development/`
- API 文档: `docs/api/`
- 部署指南: `docs/deployment/`

## 许可证

MIT
