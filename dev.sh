#!/bin/bash

# Purple 开发环境启动脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 默认端口
BACKEND_PORT=${BACKEND_PORT:-8080}
FRONTEND_PORT=${FRONTEND_PORT:-8000}

# 启动后端开发服务器
start_backend() {
    echo -e "${BLUE}🚀 启动后端开发服务器 (端口: $BACKEND_PORT)...${NC}"
    cd backend
    RUST_LOG=debug cargo run
}

# 启动前端开发服务器
start_frontend() {
    echo -e "${BLUE}🎨 启动前端开发服务器 (端口: $FRONTEND_PORT)...${NC}"
    cd frontend
    trunk serve --port=$FRONTEND_PORT --open
}

# 同时启动前后端
start_both() {
    echo -e "${BLUE}🔄 启动完整开发环境...${NC}"
    
    # 在后台启动后端
    echo -e "${YELLOW}启动后端服务器...${NC}"
    cd backend
    RUST_LOG=debug cargo run &
    BACKEND_PID=$!
    cd ..
    
    # 等待后端启动
    sleep 3
    
    # 启动前端
    echo -e "${YELLOW}启动前端服务器...${NC}"
    cd frontend
    trunk serve --port=$FRONTEND_PORT --open &
    FRONTEND_PID=$!
    cd ..
    
    echo -e "${GREEN}✅ 开发环境已启动${NC}"
    echo -e "${GREEN}后端地址: http://localhost:$BACKEND_PORT${NC}"
    echo -e "${GREEN}前端地址: http://localhost:$FRONTEND_PORT${NC}"
    echo -e "${GREEN}API 文档: http://localhost:$BACKEND_PORT/swagger-ui/${NC}"
    
    # 等待中断信号
    trap "echo -e '${YELLOW}正在关闭开发服务器...${NC}'; kill $BACKEND_PID $FRONTEND_PID; exit 0" INT
    
    # 保持脚本运行
    wait
}

# 数据库操作
setup_database() {
    echo -e "${BLUE}🗄️  设置数据库...${NC}"
    cd backend
    
    # 检查 PostgreSQL 是否运行
    if ! pg_isready -q; then
        echo -e "${RED}❌ PostgreSQL 未运行，请先启动 PostgreSQL 服务${NC}"
        exit 1
    fi
    
    # 执行迁移
    echo -e "${YELLOW}执行数据库迁移...${NC}"
    if [ -f "migrations/init.sql" ]; then
        psql -U purple -d purple -f migrations/init.sql
        echo -e "${GREEN}✅ 数据库迁移完成${NC}"
    else
        echo -e "${RED}❌ 迁移文件未找到${NC}"
        exit 1
    fi
}

# 安装开发依赖
install_dev_deps() {
    echo -e "${BLUE}📦 安装开发依赖...${NC}"
    
    # 安装 Trunk (前端构建工具)
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}安装 Trunk...${NC}"
        cargo install trunk
    fi
    
    # 安装 wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        echo -e "${YELLOW}安装 wasm-pack...${NC}"
        cargo install wasm-pack
    fi
    
    # 安装 cargo-watch (可选的热重载工具)
    if ! command -v cargo-watch &> /dev/null; then
        echo -e "${YELLOW}安装 cargo-watch...${NC}"
        cargo install cargo-watch
    fi
    
    echo -e "${GREEN}✅ 开发依赖安装完成${NC}"
}

# 主函数
main() {
    case "${1:-help}" in
        "backend")
            start_backend
            ;;
        "frontend")
            start_frontend
            ;;
        "both"|"all")
            start_both
            ;;
        "db"|"database")
            setup_database
            ;;
        "deps")
            install_dev_deps
            ;;
        "help")
            echo "Purple 开发环境工具"
            echo ""
            echo "使用方法: $0 [backend|frontend|both|db|deps|help]"
            echo ""
            echo "命令说明："
            echo "  backend   - 启动后端开发服务器"
            echo "  frontend  - 启动前端开发服务器" 
            echo "  both/all  - 同时启动前后端开发服务器"
            echo "  db        - 设置数据库"
            echo "  deps      - 安装开发依赖"
            echo "  help      - 显示此帮助信息"
            echo ""
            echo "环境变量："
            echo "  BACKEND_PORT  - 后端端口 (默认: 8080)"
            echo "  FRONTEND_PORT - 前端端口 (默认: 8000)"
            ;;
        *)
            echo -e "${RED}❌ 未知命令: $1${NC}"
            echo "使用 '$0 help' 查看帮助"
            exit 1
            ;;
    esac
}

main "$@"