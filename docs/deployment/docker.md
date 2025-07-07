# Docker 部署指南

本文档提供 Purple 项目的 Docker 容器化部署方案。

## 快速开始

### 使用 Docker Compose（推荐）

1. **克隆项目**

```bash
git clone https://github.com/your-org/purple.git
cd purple
```

2. **配置环境变量**

```bash
cp .env.example .env
# 编辑 .env 文件设置生产环境配置
```

3. **启动服务**

```bash
docker-compose up -d
```

4. **验证部署**

```bash
curl http://localhost:8080/health
```

## Docker Compose 配置

### docker-compose.yml

```yaml
version: '3.8'

services:
  purple-api:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://purple:${POSTGRES_PASSWORD}@postgres:5432/purple
      - JWT_SECRET=${JWT_SECRET}
      - ADMIN_EMAIL=${ADMIN_EMAIL}
      - ADMIN_PASSWORD=${ADMIN_PASSWORD}
      - RUST_LOG=info
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped
    networks:
      - purple-network
    volumes:
      - ./logs:/app/logs

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=purple
      - POSTGRES_USER=purple
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U purple -d purple"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped
    networks:
      - purple-network

volumes:
  postgres_data:

networks:
  purple-network:
    driver: bridge
```

### .env 配置文件

```env
# PostgreSQL 配置
POSTGRES_PASSWORD=secure_password_here

# JWT 配置
JWT_SECRET=production-jwt-secret-key-change-this

# 管理员账户
ADMIN_EMAIL=admin@yourdomain.com
ADMIN_PASSWORD=secure_admin_password

# 可选：自定义端口
API_PORT=8080
DB_PORT=5432
```

## Dockerfile

### 多阶段构建 Dockerfile

```dockerfile
# 构建阶段
FROM rust:1.70-slim as builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 创建工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟源文件以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖（缓存层）
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1001 purple

# 创建应用目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/purple /app/purple

# 创建日志目录
RUN mkdir -p /app/logs && chown -R purple:purple /app

# 切换到非 root 用户
USER purple

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动应用
CMD ["./purple"]
```

### 优化的 Dockerfile（使用 Alpine）

```dockerfile
# 构建阶段
FROM rust:1.70-alpine as builder

# 安装构建依赖
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# 设置静态链接
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

# 复制项目文件
COPY . .

# 构建发布版本
RUN cargo build --release --target x86_64-unknown-linux-musl

# 运行阶段
FROM alpine:3.18

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    curl

# 创建用户
RUN adduser -D -s /bin/sh purple

WORKDIR /app

# 复制二进制文件
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/purple .

# 设置权限
RUN chown purple:purple purple && chmod +x purple

# 切换用户
USER purple

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["./purple"]
```

## 部署配置

### 生产环境变量

```bash
# 必须修改的配置
export JWT_SECRET="your-production-jwt-secret-key"
export POSTGRES_PASSWORD="your-secure-database-password"
export ADMIN_EMAIL="admin@yourdomain.com"
export ADMIN_PASSWORD="your-secure-admin-password"

# 可选配置
export API_PORT=8080
export DB_PORT=5432
export RUST_LOG=warn
export LOG_LEVEL=warn
```

### Docker 网络配置

```bash
# 创建自定义网络
docker network create purple-network

# 启动数据库
docker run -d \
  --name purple-postgres \
  --network purple-network \
  -e POSTGRES_DB=purple \
  -e POSTGRES_USER=purple \
  -e POSTGRES_PASSWORD=secure_password \
  -v postgres_data:/var/lib/postgresql/data \
  postgres:15-alpine

# 启动应用
docker run -d \
  --name purple-api \
  --network purple-network \
  -p 8080:8080 \
  -e DATABASE_URL=postgresql://purple:secure_password@purple-postgres:5432/purple \
  -e JWT_SECRET=your-jwt-secret \
  purple:latest
```

## 常用 Docker 命令

### 构建和运行

```bash
# 构建镜像
docker build -t purple:latest .

# 运行容器
docker run -d -p 8080:8080 --name purple-app purple:latest

# 查看日志
docker logs purple-app

# 进入容器
docker exec -it purple-app sh
```

### Docker Compose 操作

```bash
# 启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f purple-api

# 重启服务
docker-compose restart purple-api

# 停止所有服务
docker-compose down

# 完全清理（包括数据卷）
docker-compose down -v
```

### 数据备份和恢复

```bash
# 备份数据库
docker exec purple-postgres pg_dump -U purple purple > backup.sql

# 恢复数据库
docker exec -i purple-postgres psql -U purple purple < backup.sql

# 备份数据卷
docker run --rm -v purple_postgres_data:/data -v $(pwd):/backup \
  alpine tar czf /backup/postgres_backup.tar.gz -C /data .
```

## 监控和日志

### 集成 Prometheus 监控

```yaml
version: '3.8'

services:
  purple-api:
    # ... 现有配置
    labels:
      - "prometheus.scrape=true"
      - "prometheus.port=8080"

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    networks:
      - purple-network

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    networks:
      - purple-network
```

### 日志聚合

```yaml
services:
  purple-api:
    # ... 现有配置
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  # 可选：添加 ELK 堆栈
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.5.0
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
    ports:
      - "9200:9200"

  logstash:
    image: docker.elastic.co/logstash/logstash:8.5.0
    volumes:
      - ./logstash.conf:/usr/share/logstash/pipeline/logstash.conf

  kibana:
    image: docker.elastic.co/kibana/kibana:8.5.0
    ports:
      - "5601:5601"
```

## 安全配置

### 安全加固

```yaml
services:
  purple-api:
    # 安全配置
    cap_drop:
      - ALL
    cap_add:
      - CHOWN
      - SETGID
      - SETUID
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp:noexec,nosuid,size=100m

  postgres:
    # 数据库安全配置
    cap_drop:
      - ALL
    cap_add:
      - CHOWN
      - DAC_OVERRIDE
      - SETGID
      - SETUID
    security_opt:
      - no-new-privileges:true
```

### 反向代理配置

```yaml
services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - purple-api
    networks:
      - purple-network
```

## 故障排除

### 常见问题

1. **容器无法启动**

```bash
# 查看详细错误信息
docker logs purple-api

# 检查配置
docker-compose config
```

2. **数据库连接失败**

```bash
# 检查数据库状态
docker-compose exec postgres pg_isready -U purple

# 测试连接
docker-compose exec purple-api wget -qO- http://localhost:8080/health
```

3. **权限问题**

```bash
# 检查文件权限
ls -la logs/

# 修复权限
sudo chown -R 1001:1001 logs/
```

### 性能调优

```yaml
services:
  purple-api:
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '1.0'
        reservations:
          memory: 256M
          cpus: '0.5'

  postgres:
    deploy:
      resources:
        limits:
          memory: 1G
          cpus: '2.0'
        reservations:
          memory: 512M
          cpus: '1.0'
```

## 更新和维护

### 滚动更新

```bash
# 构建新版本
docker build -t purple:v2.0.0 .

# 更新服务
docker-compose up -d --no-deps purple-api

# 验证更新
curl http://localhost:8080/health
```

### 定期维护

```bash
# 清理未使用的镜像
docker image prune -f

# 清理未使用的容器
docker container prune -f

# 清理未使用的网络
docker network prune -f

# 备份重要数据
docker-compose exec postgres pg_dump -U purple purple > backup-$(date +%Y%m%d).sql
```

这个 Docker 部署方案提供了完整的容器化解决方案，包括开发、测试和生产环境的配置，确保应用的稳定运行和便捷维护。
