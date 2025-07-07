# 开发环境搭建

本文档将指导您设置 Purple 项目的开发环境。

## 系统要求

### 必需软件

- **Rust**: 1.70.0 或更高版本
- **PostgreSQL**: 12+ 版本
- **Git**: 用于版本控制

### 推荐工具

- **IDE**: VS Code + rust-analyzer 插件
- **数据库工具**: pgAdmin, DBeaver 或 psql 命令行
- **API 测试**: Postman, Insomnia 或 curl
- **Docker**: 可选，用于容器化开发

## 环境安装

### 1. 安装 Rust

#### macOS

```bash
# 使用 rustup 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 或使用 Homebrew
brew install rust
```

#### Ubuntu/Debian

```bash
# 使用 rustup 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 或使用包管理器
sudo apt update
sudo apt install rustc cargo
```

#### Windows

1. 下载并运行 [rustup-init.exe](https://rustup.rs/)
2. 按照安装向导完成安装

### 2. 安装 PostgreSQL

#### macOS

```bash
# 使用 Homebrew
brew install postgresql
brew services start postgresql

# 创建数据库用户
createuser -s purple
```

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib

# 启动服务
sudo systemctl start postgresql
sudo systemctl enable postgresql

# 创建用户和数据库
sudo -u postgres createuser -s purple
sudo -u postgres createdb purple
```

#### Windows

1. 从 [PostgreSQL 官网](https://www.postgresql.org/download/windows/) 下载安装包
2. 运行安装程序，记住设置的密码
3. 将 PostgreSQL 的 bin 目录添加到 PATH

### 3. 安装开发工具

#### VS Code 插件

```bash
# 安装 rust-analyzer
code --install-extension rust-lang.rust-analyzer

# 其他有用的插件
code --install-extension ms-vscode.vscode-json
code --install-extension bradlc.vscode-tailwindcss
code --install-extension humao.rest-client
```

## 项目设置

### 1. 克隆项目

```bash
git clone https://github.com/your-org/purple.git
cd purple
```

### 2. 配置环境变量

```bash
# 复制环境变量模板
cp .env.example .env

# 编辑环境变量
nano .env  # 或使用您喜欢的编辑器
```

### 3. 配置 .env 文件

```env
# 数据库配置
DATABASE_URL=postgresql://purple:purple@localhost:5432/purple

# JWT 配置（开发环境用）
JWT_SECRET=dev-secret-key-change-in-production

# 服务器配置
SERVER_ADDR=127.0.0.1
SERVER_PORT=8080

# 管理员账户
ADMIN_EMAIL=dev@example.com
ADMIN_PASSWORD=dev123456

# 日志配置
RUST_LOG=debug
LOG_LEVEL=debug
LOG_WITH_THREAD_IDS=true
LOG_WITH_LINE_NUMBER=true
LOG_WITH_FILE=true
LOG_WITH_TARGET=false
LOG_FILE_PATH=logs/app.log
```

### 4. 创建数据库

```bash
# 创建数据库
createdb purple

# 运行迁移脚本
psql -U purple -d purple -f migrations/init.sql
```

### 5. 验证安装

```bash
# 检查 Rust 版本
rustc --version
cargo --version

# 检查数据库连接
psql -U purple -d purple -c "SELECT version();"

# 编译项目
cargo check
```

## 开发工作流

### 1. 日常开发命令

```bash
# 启动开发服务器
cargo run

# 监听文件变化自动重启（需要安装 cargo-watch）
cargo install cargo-watch
cargo watch -x run

# 代码检查
cargo check

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 静态分析
cargo clippy
```

### 2. 数据库操作

```bash
# 连接数据库
psql -U purple -d purple

# 查看表结构
\dt

# 重置数据库
dropdb purple && createdb purple
psql -U purple -d purple -f migrations/init.sql
```

### 3. 调试技巧

```bash
# 开启详细日志
RUST_LOG=debug cargo run

# 仅显示应用日志
RUST_LOG=purple=debug cargo run

# 开启 SQL 查询日志
RUST_LOG=sqlx=debug cargo run
```

## IDE 配置

### VS Code 设置

创建 `.vscode/settings.json`:

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "files.associations": {
        "*.rs": "rust"
    },
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### VS Code 任务配置

创建 `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo run",
            "type": "shell",
            "command": "cargo",
            "args": ["run"],
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {
            "label": "cargo test",
            "type": "shell",
            "command": "cargo",
            "args": ["test"],
            "group": "test"
        }
    ]
}
```

## 常见问题

### Q: 编译失败，提示缺少依赖

A: 确保已安装所有系统依赖：

```bash
# macOS
brew install pkg-config openssl

# Ubuntu/Debian
sudo apt install pkg-config libssl-dev
```

### Q: 数据库连接失败

A: 检查以下几点：

1. PostgreSQL 服务是否运行
2. 用户名和密码是否正确
3. 数据库是否存在
4. .env 文件中的 DATABASE_URL 是否正确

### Q: 端口被占用

A: 修改 .env 文件中的 SERVER_PORT，或者停止占用端口的进程：

```bash
# 查找占用端口的进程
lsof -i :8080

# 杀死进程
kill -9 <PID>
```

### Q: 权限错误

A: 确保当前用户对项目目录有读写权限：

```bash
# 修改目录权限
chmod -R 755 purple/
```

## 开发最佳实践

### 1. 代码质量

- 运行 `cargo fmt` 格式化代码
- 运行 `cargo clippy` 进行静态分析
- 编写单元测试和集成测试
- 使用有意义的提交信息

### 2. 性能调试

```bash
# 安装性能分析工具
cargo install cargo-flamegraph

# 生成火焰图
cargo flamegraph --bin purple
```

### 3. 内存检查

```bash
# 安装 valgrind（仅 Linux）
sudo apt install valgrind

# 内存检查
cargo build
valgrind --tool=memcheck target/debug/purple
```

### 4. 依赖管理

```bash
# 检查过时的依赖
cargo install cargo-outdated
cargo outdated

# 更新依赖
cargo update

# 检查未使用的依赖
cargo install cargo-udeps
cargo +nightly udeps
```

## 下一步

环境搭建完成后，您可以：

1. 阅读 [项目架构文档](architecture.md)
2. 查看 [编码规范](coding-style.md)
3. 学习 [测试指南](testing.md)
4. 开始开发新功能

如果遇到问题，请查看项目的 Issue 页面或联系维护者。
