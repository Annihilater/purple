#!/bin/bash

# Purple Workspace 构建脚本

set -e

echo "🚀 Purple Workspace 构建开始..."

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检查依赖
check_dependencies() {
    echo -e "${BLUE}🔍 检查依赖...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo 未找到，请先安装 Rust${NC}"
        exit 1
    fi
    
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}⚠️  Trunk 未找到，前端构建将被跳过${NC}"
        echo -e "${YELLOW}    请手动安装: cargo install trunk${NC}"
    fi
    
    # 检查WebAssembly目标是否安装
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo -e "${YELLOW}⚠️  WebAssembly目标未安装，尝试安装...${NC}"
        rustup target add wasm32-unknown-unknown
        
        # 验证安装成功
        if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
            echo -e "${RED}❌ WebAssembly目标安装失败${NC}"
            echo -e "${YELLOW}    前端构建将被跳过${NC}"
            echo -e "${YELLOW}    如需手动安装，请运行: rustup target add wasm32-unknown-unknown${NC}"
        else
            echo -e "${GREEN}✅ WebAssembly目标安装成功${NC}"
        fi
    fi
    
    echo -e "${GREEN}✅ 依赖检查完成${NC}"
}

# 构建后端
build_backend() {
    echo -e "${BLUE}🔧 构建后端...${NC}"
    cd backend
    cargo build --release
    cd ..
    echo -e "${GREEN}✅ 后端构建完成${NC}"
}

# 构建管理员前端
build_admin_frontend() {
    echo -e "${BLUE}🎨 构建管理员前端...${NC}"
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}⚠️  Trunk 未安装，跳过管理员前端构建${NC}"
        return 0
    fi
    
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo -e "${YELLOW}⚠️  WebAssembly目标未安装，跳过管理员前端构建${NC}"
        return 0
    fi
    
    cd admin-frontend
    trunk build --release
    cd ..
    echo -e "${GREEN}✅ 管理员前端构建完成${NC}"
}

# 构建用户前端
build_user_frontend() {
    echo -e "${BLUE}🎨 构建用户前端...${NC}"
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}⚠️  Trunk 未安装，跳过用户前端构建${NC}"
        return 0
    fi
    
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo -e "${YELLOW}⚠️  WebAssembly目标未安装，跳过用户前端构建${NC}"
        return 0
    fi
    
    cd user-frontend
    trunk build --release
    cd ..
    echo -e "${GREEN}✅ 用户前端构建完成${NC}"
}

# 构建共享库
build_shared() {
    echo -e "${BLUE}📦 构建共享库...${NC}"
    cd shared
    cargo build --release
    cd ..
    echo -e "${GREEN}✅ 共享库构建完成${NC}"
}

# 运行测试
run_tests() {
    echo -e "${BLUE}🧪 运行测试...${NC}"
    cargo test --workspace
    echo -e "${GREEN}✅ 测试完成${NC}"
}

# 代码检查
run_lint() {
    echo -e "${BLUE}🔍 代码检查...${NC}"
    cargo fmt --all -- --check
    # 只显示重要的 clippy 警告，忽略常见的未使用警告
    cargo clippy --workspace --quiet -- -W clippy::all -A dead_code -A unused_variables -A unused_imports
    echo -e "${GREEN}✅ 代码检查通过${NC}"
}

# 主函数
main() {
    case "${1:-all}" in
        "deps")
            check_dependencies
            ;;
        "backend")
            build_backend
            ;;
        "admin-frontend")
            build_admin_frontend
            ;;
        "user-frontend")
            build_user_frontend
            ;;
        "frontend-all")
            build_admin_frontend
            build_user_frontend
            ;;
        "shared")
            build_shared
            ;;
        "test")
            run_tests
            ;;
        "lint")
            run_lint
            ;;
        "check")
            cargo check --workspace
            ;;
        "clean")
            cargo clean
            cd admin-frontend && trunk clean && cd ..
            cd user-frontend && trunk clean && cd ..
            ;;
        "all")
            check_dependencies
            build_shared
            build_backend
            build_admin_frontend
            build_user_frontend
            run_tests
            echo -e "${GREEN}🎉 Purple Workspace 构建完成！${NC}"
            ;;
        *)
            echo "使用方法: $0 [deps|backend|admin-frontend|user-frontend|frontend-all|shared|test|lint|check|clean|all]"
            echo ""
            echo "命令说明："
            echo "  deps          - 检查并安装依赖"
            echo "  backend       - 构建后端"
            echo "  admin-frontend - 构建管理员前端"
            echo "  user-frontend  - 构建用户前端"
            echo "  frontend-all   - 构建两个前端"
            echo "  shared        - 构建共享库"
            echo "  test          - 运行测试"
            echo "  lint          - 代码检查"
            echo "  check         - 快速检查编译"
            echo "  clean         - 清理构建缓存"
            echo "  all           - 完整构建（默认）"
            exit 1
            ;;
    esac
}

main "$@"