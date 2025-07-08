#!/bin/bash

# 定义颜色
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

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

echo -e "${BLUE}==================================${NC}"
echo -e "${BLUE}    启动 Purple 开发环境    ${NC}"
echo -e "${BLUE}==================================${NC}"

# 启动后端服务
echo -e "${GREEN}[*] 启动后端服务...${NC}"
cd backend && cargo run | tee ../logs/backend.log &
BACKEND_PID=$!
echo -e "${GREEN}[✓] 后端服务已启动 (PID: ${BACKEND_PID})${NC}"

# 等待2秒，确保后端已经启动
sleep 2

# 启动管理员前端
echo -e "${GREEN}[*] 启动管理员前端...${NC}"
cd ../admin-frontend && trunk serve --port 8081 | tee ../logs/admin-frontend.log &
ADMIN_FRONTEND_PID=$!
echo -e "${GREEN}[✓] 管理员前端已启动 (PID: ${ADMIN_FRONTEND_PID})${NC}"

# 启动用户前端
echo -e "${GREEN}[*] 启动用户前端...${NC}"
cd ../user-frontend && trunk serve --port 8082 | tee ../logs/user-frontend.log &
USER_FRONTEND_PID=$!
echo -e "${GREEN}[✓] 用户前端已启动 (PID: ${USER_FRONTEND_PID})${NC}"

echo -e "${BLUE}==================================${NC}"
echo -e "${GREEN}所有服务已启动:${NC}"
echo -e "  - 后端: ${GREEN}http://localhost:8080${NC}"
echo -e "  - 管理员前端: ${GREEN}http://localhost:8081${NC}"
echo -e "  - 用户前端: ${GREEN}http://localhost:8082${NC}"
echo -e "${BLUE}==================================${NC}"
echo -e "${YELLOW}按 Ctrl+C 停止所有服务${NC}"

# 等待所有后台进程完成
wait