# API 接口文档

Purple 是一个基于 Rust 和 Actix-web 构建的现代 Web API 项目，提供用户管理、套餐管理、优惠券系统等完整功能。

## 🚀 快速开始

### 基础信息

- **基础URL**: `http://127.0.0.1:8080`
- **API版本**: v1
- **认证方式**: JWT Bearer Token
- **数据格式**: JSON

### 在线文档

- **Swagger UI**: [http://127.0.0.1:8080/swagger-ui/](http://127.0.0.1:8080/swagger-ui/)
- **OpenAPI规范**: [http://127.0.0.1:8080/api-docs/openapi.json](http://127.0.0.1:8080/api-docs/openapi.json)

## 📋 目录

- [认证接口](./authentication.md) - 用户注册、登录和Token管理
- [用户管理](#用户管理) - 用户CRUD操作
- [套餐管理](#套餐管理) - 套餐配置和管理
- [优惠券系统](#优惠券系统) - 优惠券创建和验证
- [订阅管理](#订阅管理) - 用户订阅和配置获取

## 🛡️ 认证说明

除以下公开接口外，所有API都需要JWT认证：

- `GET /` - 项目信息
- `GET /health` - 健康检查  
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `GET /swagger-ui/` - API文档
- `GET /api-docs/openapi.json` - OpenAPI规范

**认证Header格式:**

```
Authorization: Bearer {your_jwt_token}
```

详细认证文档请参考 [认证接口文档](./authentication.md)

## 📊 统一响应格式

### 成功响应

```json
{
  "success": true,
  "data": { /* 实际数据 */ },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "错误信息",
    "details": "详细错误信息（可选）",
    "field": "field_name（字段级验证错误，可选）"
  },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

### 分页响应

```json
{
  "success": true,
  "data": [ /* 数据数组 */ ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 100,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  },
  "meta": {
    "timestamp": 1751886867,
    "request_id": "uuid-here"
  }
}
```

## 👥 用户管理

### 获取用户列表

```bash
GET /api/users?page=1&page_size=20
Authorization: Bearer {token}
```

### 获取用户详情

```bash
GET /api/users/{user_id}
Authorization: Bearer {token}
```

### 创建用户

```bash
POST /api/users
Authorization: Bearer {token}
Content-Type: application/json

{
  "username": "newuser",
  "email": "user@example.com",
  "password": "password123"
}
```

### 更新用户

```bash
PUT /api/users/{user_id}
Authorization: Bearer {token}
Content-Type: application/json

{
  "email": "newemail@example.com"
}
```

### 删除用户

```bash
DELETE /api/users/{user_id}
Authorization: Bearer {token}
```

### 更新用户状态

```bash
PUT /api/users/{user_id}/status
Authorization: Bearer {token}
Content-Type: application/json

{
  "banned": true
}
```

## 📦 套餐管理

### 获取套餐列表

```bash
GET /api/plans?page=1&page_size=20
Authorization: Bearer {token}
```

### 获取启用的套餐

```bash
GET /api/plans/enabled
Authorization: Bearer {token}
```

### 获取套餐详情

```bash
GET /api/plans/{plan_id}
Authorization: Bearer {token}
```

### 创建套餐

```bash
POST /api/plans
Authorization: Bearer {token}
Content-Type: application/json

{
  "group_id": 1,
  "transfer_enable": 107374182400,
  "name": "标准套餐",
  "show": true,
  "sort": 1,
  "renew": true,
  "content": "套餐描述",
  "month_price": 10.00,
  "quarter_price": 28.00,
  "half_year_price": 50.00,
  "year_price": 90.00,
  "two_year_price": 160.00,
  "three_year_price": 220.00,
  "onetime_price": null,
  "reset_price": null,
  "capacity_limit": 3
}
```

### 获取套餐统计

```bash
GET /api/plans/stats
Authorization: Bearer {token}
```

### 获取套餐价格

```bash
GET /api/plans/{plan_id}/pricing
Authorization: Bearer {token}
```

### 检查套餐可用性

```bash
GET /api/plans/{plan_id}/availability
Authorization: Bearer {token}
```

## 🎫 优惠券系统

### 获取优惠券列表

```bash
GET /api/coupons?page=1&page_size=20
Authorization: Bearer {token}
```

### 创建优惠券

```bash
POST /api/coupons
Authorization: Bearer {token}
Content-Type: application/json

{
  "name": "新用户优惠",
  "type": 1,
  "value": 10,
  "limit_use": 100,
  "limit_use_with_user": 1,
  "limit_plan_ids": [1, 2, 3],
  "started_at": "2024-01-01T00:00:00Z",
  "ended_at": "2024-12-31T23:59:59Z"
}
```

### 验证优惠券

```bash
POST /api/coupons/verify
Authorization: Bearer {token}
Content-Type: application/json

{
  "code": "COUPON_CODE",
  "amount": 100
}
```

## 📡 订阅管理

### 获取订阅信息

```bash
GET /api/subscribe/info
Authorization: Bearer {token}
```

### 获取订阅链接

```bash
GET /api/subscribe/link
Authorization: Bearer {token}
```

### 重置订阅Token

```bash
POST /api/subscribe/reset-token
Authorization: Bearer {token}
```

### 获取订阅统计

```bash
GET /api/subscribe/stats
Authorization: Bearer {token}
```

### 获取节点状态

```bash
GET /api/subscribe/nodes/status
Authorization: Bearer {token}
```

### 获取客户端配置（无需认证）

```bash
GET /api/subscribe/config?token={subscribe_token}&type=clash
```

## 📈 错误代码说明

| 错误代码 | HTTP状态码 | 说明 |
|---------|-----------|------|
| `SUCCESS` | 200 | 请求成功 |
| `INTERNAL_ERROR` | 500 | 内部服务器错误 |
| `INVALID_PARAMS` | 400 | 请求参数无效 |
| `VALIDATION_ERROR` | 400 | 参数验证失败 |
| `UNAUTHORIZED` | 401 | 未授权访问 |
| `INVALID_TOKEN` | 401 | 无效的Token |
| `TOKEN_EXPIRED` | 401 | Token已过期 |
| `INVALID_CREDENTIALS` | 401 | 用户名或密码错误 |
| `USER_NOT_FOUND` | 404 | 用户不存在 |
| `USER_ALREADY_EXISTS` | 409 | 用户已存在 |
| `USER_DISABLED` | 403 | 用户已被禁用 |
| `PLAN_NOT_FOUND` | 404 | 套餐不存在 |
| `COUPON_NOT_FOUND` | 404 | 优惠券不存在 |
| `COUPON_EXPIRED` | 400 | 优惠券已过期 |
| `ORDER_NOT_FOUND` | 404 | 订单不存在 |
| `PAYMENT_FAILED` | 400 | 支付失败 |

## 💡 使用技巧

### 1. 批量操作

某些接口支持批量操作，如批量更新套餐状态：

```bash
POST /api/plans/batch/status
Authorization: Bearer {token}
Content-Type: application/json

{
  "plan_ids": [1, 2, 3],
  "show": false
}
```

### 2. 分页参数

大部分列表接口都支持分页：

- `page`: 页码，从1开始
- `page_size`: 每页数量，默认20，最大100

### 3. 排序和过滤

部分接口支持排序和过滤参数，具体请参考各接口文档。

### 4. 响应时间

所有请求都会在响应中包含 `request_id`，便于问题追踪和日志查询。

## 🔍 调试工具

### 使用curl进行测试

```bash
# 设置基础URL和Token
BASE_URL="http://127.0.0.1:8080"
TOKEN="your_jwt_token_here"

# 测试认证接口
curl -X GET "$BASE_URL/api/users" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" | jq .
```

### 使用Postman

1. 导入OpenAPI规范：`http://127.0.0.1:8080/api-docs/openapi.json`
2. 设置环境变量：
   - `base_url`: `http://127.0.0.1:8080`
   - `token`: 您的JWT Token
3. 在请求Header中添加：`Authorization: Bearer {{token}}`

## 📞 技术支持

如果您在使用API时遇到问题，请：

1. 首先查看 [Swagger UI](http://127.0.0.1:8080/swagger-ui/) 确认接口调用方式
2. 检查请求格式和认证信息是否正确
3. 查看响应中的 `request_id` 并记录错误信息
4. 联系技术支持并提供详细的错误信息

---

*更多详细信息请参考项目的 [开发文档](../development/) 和 [部署文档](../deployment/)*
