#!/bin/bash

# 定义颜色
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# 获取脚本所在目录的绝对路径
SCRIPT_DIR="$( cd "$( dirname "$0" )" && pwd )"

# 切换到脚本所在目录
cd "$SCRIPT_DIR" || {
    echo -e "${RED}错误: 无法切换到脚本目录 ${SCRIPT_DIR}${NC}"
    exit 1
}

# 定义目录和端口
RUN_DIR="$SCRIPT_DIR/run"
LOGS_DIR="$SCRIPT_DIR/logs"
BACKEND_PORT=8080
ADMIN_FRONTEND_PORT=3000
USER_FRONTEND_PORT=8000

# 确保目录存在
mkdir -p "$RUN_DIR"
mkdir -p "$LOGS_DIR"

# 定义PID文件路径
BACKEND_PID_FILE="$RUN_DIR/backend.pid"
ADMIN_FRONTEND_PID_FILE="$RUN_DIR/admin-frontend.pid"
USER_FRONTEND_PID_FILE="$RUN_DIR/user-frontend.pid"

# 获取实际服务进程的PID
get_actual_pid() {
    local name="$1"
    local pattern="$2"
    local pids
    local pid
    
    # 使用 pgrep 获取匹配的进程列表
    if [ -n "$pattern" ]; then
        pids=$(pgrep -f "$pattern")
    else
        pids=$(pgrep -f "$name")
    fi
    
    # 如果没有找到进程，返回空
    if [ -z "$pids" ]; then
        return
    fi
    
    # 对于每个找到的进程
    for pid in $pids; do
        # 检查进程命令行，排除包含 "tee" 的进程
        local cmd
        cmd=$(ps -p "$pid" -o command=)
        if ! echo "$cmd" | grep -q "tee"; then
            # 如果是 trunk serve，还要检查它是否真的在监听端口
            if [[ "$name" == "trunk serve" ]]; then
                if [[ "$cmd" == *"trunk serve"* ]]; then
                    if [[ "$pattern" == *"admin-frontend"* ]] && [[ "$cmd" == *"--port ${ADMIN_FRONTEND_PORT}"* ]]; then
                        echo "$pid"
                        return
                    elif [[ "$pattern" == *"user-frontend"* ]] && [[ "$cmd" == *"--port ${USER_FRONTEND_PORT}"* ]]; then
                        echo "$pid"
                        return
                    fi
                fi
            else
                echo "$pid"
                return
            fi
        fi
    done
}

# 检查服务是否运行
is_service_running() {
    local name="$1"
    local pattern="$2"
    local pid
    
    pid=$(get_actual_pid "$name" "$pattern")
    if [ -n "$pid" ]; then
        # 对于 trunk serve，还要检查端口是否正在监听
        if [[ "$name" == "trunk serve" ]]; then
            local port
            local cmd
            cmd=$(ps -p "$pid" -o command=)
            if [[ "$pattern" == *"admin-frontend"* ]]; then
                port=$ADMIN_FRONTEND_PORT
            elif [[ "$pattern" == *"user-frontend"* ]]; then
                port=$USER_FRONTEND_PORT
            fi
            if [ -n "$port" ] && lsof -i :$port > /dev/null 2>&1; then
                if [[ "$cmd" == *"--port ${port}"* ]]; then
                    return 0  # 服务正在运行且端口在监听
                fi
            fi
            return 1  # 服务在运行但端口未监听或端口不匹配
        fi
        return 0  # 服务正在运行
    fi
    return 1  # 服务未运行
}

# 等待端口释放
wait_for_port_release() {
    local port=$1
    local timeout=10
    local count=0
    while [ $count -lt $timeout ]; do
        if ! lsof -i :$port > /dev/null 2>&1; then
            return 0  # 端口已释放
        fi
        sleep 1
        count=$((count + 1))
    done
    return 1  # 超时
}

# 停止服务
stop_service() {
    local name="$1"
    local pattern="$2"
    local pid
    
    pid=$(get_actual_pid "$name" "$pattern")
    if [ -n "$pid" ]; then
        echo -e "${GREEN}正在停止 $name...${NC}"
        kill -9 "$pid" 2>/dev/null
        # 等待进程结束和端口释放
        sleep 1
        if [ -n "$3" ]; then  # 如果提供了端口参数
            wait_for_port_release "$3"
        fi
        echo -e "${GREEN}[✓] $name 已停止${NC}"
    else
        echo -e "${YELLOW}警告: $name 未运行${NC}"
    fi
}

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

# 检查端口是否被占用
check_port() {
    local port=$1
    if lsof -i :$port > /dev/null 2>&1; then
        return 0  # 端口被占用
    fi
    return 1  # 端口未被占用
}

# 等待服务启动
wait_for_service() {
    local name="$1"
    local pattern="$2"
    local port="$3"
    local timeout=10
    local count=0
    
    # 如果指定了端口，先等待端口释放
    if [ -n "$port" ]; then
        if ! wait_for_port_release "$port"; then
            echo -e "${RED}错误: 等待端口 $port 释放超时${NC}"
            return 1
        fi
    fi
    
    while [ $count -lt $timeout ]; do
        if get_actual_pid "$name" "$pattern" > /dev/null; then
            # 如果指定了端口，还要检查端口是否被正确监听
            if [ -n "$port" ]; then
                if lsof -i :$port > /dev/null 2>&1; then
                    return 0  # 服务已启动且端口已监听
                fi
            else
                return 0  # 服务已启动（无需检查端口）
            fi
        fi
        sleep 1
        count=$((count + 1))
    done
    return 1  # 超时
}

# 启动后端
start_backend() {
    if is_service_running "purple-backend"; then
        echo -e "${YELLOW}警告: 后端服务已在运行${NC}"
        return 1
    fi

    if check_port $BACKEND_PORT; then
        echo -e "${RED}错误: 端口 $BACKEND_PORT 已被占用${NC}"
        return 1
    fi

    echo -e "${GREEN}[*] 启动后端服务...${NC}"
    cd backend && RUST_LOG=debug cargo run | tee "$LOGS_DIR/backend.log" &
    cd "$SCRIPT_DIR"
    
    if wait_for_service "purple-backend" "" "$BACKEND_PORT"; then
        echo -e "${GREEN}[✓] 后端服务已启动${NC}"
        return 0
    else
        echo -e "${RED}错误: 后端服务启动失败${NC}"
        return 1
    fi
}

# 启动管理员前端
start_admin_frontend() {
    if is_service_running "trunk serve" "trunk serve.*--port ${ADMIN_FRONTEND_PORT}"; then
        echo -e "${YELLOW}警告: 管理员前端已在运行${NC}"
        return 1
    fi

    if check_port $ADMIN_FRONTEND_PORT; then
        echo -e "${RED}错误: 端口 $ADMIN_FRONTEND_PORT 已被占用${NC}"
        return 1
    fi

    echo -e "${GREEN}[*] 启动管理员前端...${NC}"
    cd admin-frontend && trunk serve --port $ADMIN_FRONTEND_PORT --proxy-backend=http://localhost:$BACKEND_PORT/api/ | tee "$LOGS_DIR/admin-frontend.log" &
    cd "$SCRIPT_DIR"
    
    # 等待服务启动
    sleep 3
    if is_service_running "trunk serve" "trunk serve.*--port ${ADMIN_FRONTEND_PORT}"; then
        echo -e "${GREEN}[✓] 管理员前端已启动${NC}"
        return 0
    else
        echo -e "${RED}[✗] 管理员前端启动失败${NC}"
        return 1
    fi
}

# 启动用户前端
start_user_frontend() {
    if is_service_running "trunk serve" "trunk serve.*--port ${USER_FRONTEND_PORT}"; then
        echo -e "${YELLOW}警告: 用户前端已在运行${NC}"
        return 1
    fi

    if check_port $USER_FRONTEND_PORT; then
        echo -e "${RED}错误: 端口 $USER_FRONTEND_PORT 已被占用${NC}"
        return 1
    fi

    echo -e "${GREEN}[*] 启动用户前端...${NC}"
    cd user-frontend && trunk serve --port $USER_FRONTEND_PORT --proxy-backend=http://localhost:$BACKEND_PORT/api/ | tee "$LOGS_DIR/user-frontend.log" &
    cd "$SCRIPT_DIR"
    
    # 等待服务启动
    sleep 3
    if is_service_running "trunk serve" "trunk serve.*--port ${USER_FRONTEND_PORT}"; then
        echo -e "${GREEN}[✓] 用户前端已启动${NC}"
        return 0
    else
        echo -e "${RED}[✗] 用户前端启动失败${NC}"
        return 1
    fi
}

# 停止所有服务
stop_all() {
    stop_service "trunk serve" "user-frontend.*trunk serve" "$USER_FRONTEND_PORT"
    stop_service "trunk serve" "admin-frontend.*trunk serve" "$ADMIN_FRONTEND_PORT"
    stop_service "purple-backend" "" "$BACKEND_PORT"
}

# 显示服务状态
show_status() {
    echo "=================================="
    echo "服务状态:"
    
    # 检查后端状态
    local backend_pid=$(get_actual_pid "purple-backend" "")
    if [ -n "$backend_pid" ] && lsof -i :$BACKEND_PORT > /dev/null 2>&1; then
        echo "  - 后端: 运行中 (PID: $backend_pid) - http://localhost:$BACKEND_PORT"
    else
        echo "  - 后端: 未运行"
    fi
    
    # 检查管理员前端状态
    local admin_pid=$(get_actual_pid "trunk serve" "trunk serve.*--port ${ADMIN_FRONTEND_PORT}")
    if [ -n "$admin_pid" ] && lsof -i :$ADMIN_FRONTEND_PORT > /dev/null 2>&1; then
        local cmd
        cmd=$(ps -p "$admin_pid" -o command=)
        if [[ "$cmd" == *"--port ${ADMIN_FRONTEND_PORT}"* ]]; then
            echo "  - 管理员前端: 运行中 (PID: $admin_pid) - http://localhost:$ADMIN_FRONTEND_PORT"
        else
            echo "  - 管理员前端: 未运行"
        fi
    else
        echo "  - 管理员前端: 未运行"
    fi
    
    # 检查用户前端状态
    local user_pid=$(get_actual_pid "trunk serve" "trunk serve.*--port ${USER_FRONTEND_PORT}")
    if [ -n "$user_pid" ] && lsof -i :$USER_FRONTEND_PORT > /dev/null 2>&1; then
        local cmd
        cmd=$(ps -p "$user_pid" -o command=)
        if [[ "$cmd" == *"--port ${USER_FRONTEND_PORT}"* ]]; then
            echo "  - 用户前端: 运行中 (PID: $user_pid) - http://localhost:$USER_FRONTEND_PORT"
        else
            echo "  - 用户前端: 未运行"
        fi
    else
        echo "  - 用户前端: 未运行"
    fi
    
    echo "=================================="
}

# 主函数
main() {
    case "${1:-all}" in
        "deps")
            check_dependencies
            ;;
        "start")
            case "${2:-all}" in
                "backend")
                    check_dependencies
                    start_backend
                    ;;
                "admin-frontend")
                    check_dependencies
                    start_admin_frontend
                    ;;
                "user-frontend")
                    check_dependencies
                    start_user_frontend
                    ;;
                "frontend-all")
                    check_dependencies
                    if start_admin_frontend; then
                        start_user_frontend
                    fi
                    ;;
                "all")
                    check_dependencies
                    if start_backend; then
                        sleep 2
                        if start_admin_frontend; then
                            start_user_frontend
                        fi
                    fi
                    ;;
                *)
                    echo "使用方法: $0 start [backend|admin-frontend|user-frontend|frontend-all|all]"
                    exit 1
                    ;;
            esac
            show_status
            ;;
        "stop")
            case "${2:-all}" in
                "backend")
                    stop_service "purple-backend" "" "$BACKEND_PORT"
                    ;;
                "admin-frontend")
                    stop_service "trunk serve" "admin-frontend.*trunk serve" "$ADMIN_FRONTEND_PORT"
                    ;;
                "user-frontend")
                    stop_service "trunk serve" "user-frontend.*trunk serve" "$USER_FRONTEND_PORT"
                    ;;
                "frontend-all")
                    stop_service "trunk serve" "admin-frontend.*trunk serve" "$ADMIN_FRONTEND_PORT"
                    stop_service "trunk serve" "user-frontend.*trunk serve" "$USER_FRONTEND_PORT"
                    ;;
                "all")
                    stop_all
                    ;;
                *)
                    echo "使用方法: $0 stop [backend|admin-frontend|user-frontend|frontend-all|all]"
                    exit 1
                    ;;
            esac
            show_status
            ;;
        "status")
            show_status
            ;;
        *)
            echo "使用方法: $0 [deps|start|stop|status]"
            echo ""
            echo "命令说明："
            echo "  deps                    - 检查并安装依赖"
            echo "  start [service]         - 启动指定服务"
            echo "  stop [service]          - 停止指定服务"
            echo "  status                  - 显示所有服务状态"
            echo ""
            echo "服务选项："
            echo "  backend                 - 后端服务"
            echo "  admin-frontend          - 管理员前端"
            echo "  user-frontend           - 用户前端"
            echo "  frontend-all            - 所有前端"
            echo "  all                     - 所有服务（默认）"
            exit 1
            ;;
    esac
}

main "$@"