#!/bin/bash

# 定义颜色
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# 定义端口
BACKEND_PORT=3000
ADMIN_FRONTEND_PORT=8000
USER_FRONTEND_PORT=8080

# 定义退出处理函数
cleanup() {
    echo -e "${YELLOW}正在停止所有服务...${NC}"
    kill $(jobs -p) 2>/dev/null
    exit 0
}

# 捕获中断信号
trap cleanup SIGINT

# 确保日志目录存在
mkdir -p logs

# 检查依赖
check_dependencies() {
    echo -e "${BLUE}🔍 检查依赖...${NC}"
    
    # 检查是否有wasm32目标，如果没有则安装
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo -e "${YELLOW}[!] 正在安装 WebAssembly 目标...${NC}"
        rustup target add wasm32-unknown-unknown
    fi

    # 检查是否安装了 trunk
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}[!] 正在安装 trunk...${NC}"
        cargo install trunk
    fi
    
    echo -e "${GREEN}✅ 依赖检查完成${NC}"
}

# 启动后端
start_backend() {
    echo -e "${GREEN}[*] 启动后端服务...${NC}"
    cd backend && RUST_LOG=debug cargo run | tee ../logs/backend.log &
    BACKEND_PID=$!
    echo -e "${GREEN}[✓] 后端服务已启动 (PID: ${BACKEND_PID})${NC}"
    cd ..
}

# 启动管理员前端
start_admin_frontend() {
    echo -e "${GREEN}[*] 启动管理员前端...${NC}"
    cd admin-frontend && trunk serve --port ${ADMIN_FRONTEND_PORT} --proxy-backend=http://localhost:${BACKEND_PORT}/api/ | tee ../logs/admin-frontend.log &
    ADMIN_FRONTEND_PID=$!
    echo -e "${GREEN}[✓] 管理员前端已启动 (PID: ${ADMIN_FRONTEND_PID})${NC}"
    cd ..
}

# 启动用户前端
start_user_frontend() {
    echo -e "${GREEN}[*] 启动用户前端...${NC}"
    cd user-frontend && trunk serve --port ${USER_FRONTEND_PORT} --proxy-backend=http://localhost:${BACKEND_PORT}/api/ | tee ../logs/user-frontend.log &
    USER_FRONTEND_PID=$!
    echo -e "${GREEN}[✓] 用户前端已启动 (PID: ${USER_FRONTEND_PID})${NC}"
    cd ..
}

# 显示服务状态
show_status() {
    echo -e "${BLUE}==================================${NC}"
    echo -e "${GREEN}服务已启动:${NC}"
    if [[ -n "${BACKEND_PID}" ]]; then
        echo -e "  - 后端: ${GREEN}http://localhost:${BACKEND_PORT}${NC}"
    fi
    if [[ -n "${ADMIN_FRONTEND_PID}" ]]; then
        echo -e "  - 管理员前端: ${GREEN}http://localhost:${ADMIN_FRONTEND_PORT}${NC}"
    fi
    if [[ -n "${USER_FRONTEND_PID}" ]]; then
        echo -e "  - 用户前端: ${GREEN}http://localhost:${USER_FRONTEND_PORT}${NC}"
    fi
    echo -e "${BLUE}==================================${NC}"
    echo -e "${YELLOW}按 Ctrl+C 停止所有服务${NC}"
}

# 主函数
main() {
    echo -e "${BLUE}==================================${NC}"
    echo -e "${BLUE}    启动 Purple 开发环境    ${NC}"
    echo -e "${BLUE}==================================${NC}"

    case "${1:-all}" in
        "deps")
            check_dependencies
            ;;
        "backend")
            check_dependencies
            start_backend
            show_status
            wait
            ;;
        "admin-frontend")
            check_dependencies
            start_admin_frontend
            show_status
            wait
            ;;
        "user-frontend")
            check_dependencies
            start_user_frontend
            show_status
            wait
            ;;
        "frontend-all")
            check_dependencies
            start_admin_frontend
            start_user_frontend
            show_status
            wait
            ;;
        "all")
            check_dependencies
            start_backend
            # 等待2秒，确保后端已经启动
            sleep 2
            start_admin_frontend
            start_user_frontend
            show_status
            wait
            ;;
        *)
            echo "使用方法: $0 [deps|backend|admin-frontend|user-frontend|frontend-all|all]"
            echo ""
            echo "命令说明："
            echo "  deps           - 检查并安装依赖"
            echo "  backend        - 只启动后端服务"
            echo "  admin-frontend - 只启动管理员前端"
            echo "  user-frontend  - 只启动用户前端"
            echo "  frontend-all   - 启动两个前端"
            echo "  all            - 启动所有服务（默认）"
            exit 1
            ;;
    esac
}

main "$@"