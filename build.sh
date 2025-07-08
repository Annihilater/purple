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
        echo -e "${YELLOW}    请手动安装: cargo install trunk --locked${NC}"
    fi
    
    if ! command -v wasm-pack &> /dev/null; then
        echo -e "${YELLOW}⚠️  wasm-pack 未找到，正在安装...${NC}"
        cargo install wasm-pack
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

# 构建前端
build_frontend() {
    echo -e "${BLUE}🎨 构建前端...${NC}"
    if ! command -v trunk &> /dev/null; then
        echo -e "${YELLOW}⚠️  Trunk 未安装，跳过前端构建${NC}"
        return 0
    fi
    cd frontend
    trunk build --release
    cd ..
    echo -e "${GREEN}✅ 前端构建完成${NC}"
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
        "frontend")
            build_frontend
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
            cd frontend && trunk clean && cd ..
            ;;
        "all")
            check_dependencies
            build_shared
            build_backend
            build_frontend
            run_tests
            echo -e "${GREEN}🎉 Purple Workspace 构建完成！${NC}"
            ;;
        *)
            echo "使用方法: $0 [deps|backend|frontend|shared|test|lint|check|clean|all]"
            echo ""
            echo "命令说明："
            echo "  deps     - 检查并安装依赖"
            echo "  backend  - 构建后端"
            echo "  frontend - 构建前端"
            echo "  shared   - 构建共享库"
            echo "  test     - 运行测试"
            echo "  lint     - 代码检查"
            echo "  check    - 快速检查编译"
            echo "  clean    - 清理构建缓存"
            echo "  all      - 完整构建（默认）"
            exit 1
            ;;
    esac
}

main "$@"