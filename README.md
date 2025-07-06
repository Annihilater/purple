# Purple

一个使用Rust和Actix-web框架构建的现代Web应用。

## 功能特性

- 基于Actix-web的高性能Web服务器
- 结构化的项目布局
- 环境配置管理
- 日志系统集成
- 健康检查接口

## 项目结构

```
src/
  ├── api/        # API路由和处理器
  ├── config/     # 配置管理
  ├── models/     # 数据模型
  ├── services/   # 业务逻辑服务
  └── utils/      # 工具函数
```

## 开发环境要求

- Rust 1.70.0 或更高版本
- Cargo包管理器

## 快速开始

1. 克隆项目
```bash
git clone https://github.com/yourusername/purple.git
cd purple
```

2. 创建环境配置文件
```bash
cp .env.example .env
```

3. 运行项目
```bash
cargo run
```

4. 测试健康检查接口
```bash
curl http://localhost:8080/health
```

## 配置

项目使用环境变量进行配置，主要配置项包括：

- `SERVER_ADDR`: 服务器监听地址（默认：127.0.0.1）
- `SERVER_PORT`: 服务器端口（默认：8080）
- `RUST_LOG`: 日志级别（默认：info）

## 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解更多信息。 