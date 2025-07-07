# 认证 API 文档

Purple API 使用 JWT (JSON Web Token) 进行身份验证。除了公开接口外，所有 API 端点都需要有效的 JWT token。

## 公开接口

以下接口无需认证即可访问：

- `GET /` - 项目信息
- `GET /health` - 健康检查
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `GET /swagger-ui/` - API 文档
- `GET /api-docs/openapi.json` - OpenAPI 规范

## 用户注册

### 接口信息

- **URL**: `/api/auth/register`
- **方法**: `POST`
- **Content-Type**: `application/json`

### 请求参数

```json
{
  "username": "string",  // 用户名，3-20字符
  "email": "string",     // 邮箱地址，必须是有效邮箱格式
  "password": "string"   // 密码，6-32字符
}
```

### 请求示例

```bash
curl -X POST http://127.0.0.1:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "123456"
  }'
```

### 响应格式

#### 成功响应 (200)

```json
{
  "success": true,
  "data": 12345,  // 新创建的用户ID
  "meta": {
    "timestamp": 1751899623,
    "request_id": "uuid-here"
  }
}
```

#### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "USER_ALREADY_EXISTS",
    "message": "用户已存在",
    "details": "用户已存在"
  },
  "meta": {
    "timestamp": 1751899623,
    "request_id": "uuid-here"
  }
}
```

## 用户登录

### 接口信息

- **URL**: `/api/auth/login`
- **方法**: `POST`
- **Content-Type**: `application/json`

### 请求参数

```json
{
  "username": "string",  // 用户名，3-20字符
  "password": "string"   // 密码，6-32字符
}
```

### 请求示例

```bash
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "123456"
  }'
```

### 响应格式

#### 成功响应 (200)

```json
{
  "success": true,
  "data": {
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "token_type": "Bearer",
    "expires_in": 604800  // 7天，以秒为单位
  },
  "meta": {
    "timestamp": 1751899623,
    "request_id": "uuid-here"
  }
}
```

#### 错误响应

**用户名或密码错误 (401):**

```json
{
  "success": false,
  "error": {
    "code": "INVALID_CREDENTIALS",
    "message": "用户名或密码错误",
    "details": "用户名或密码错误"
  },
  "meta": {
    "timestamp": 1751899623,
    "request_id": "uuid-here"
  }
}
```

**账户已被禁用 (403):**

```json
{
  "success": false,
  "error": {
    "code": "USER_DISABLED",
    "message": "账户已被禁用",
    "details": "账户已被禁用"
  }
}
```

## 使用 JWT Token

### Authorization Header

获得 JWT token 后，在后续的 API 请求中需要在 HTTP Header 中包含认证信息：

```
Authorization: Bearer {your_jwt_token}
```

### 请求示例

```bash
# 假设登录后获得的token
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."

# 使用token访问受保护的API
curl -X GET http://127.0.0.1:8080/api/users \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"
```

### JavaScript 示例

```javascript
// 登录并保存token
const login = async (username, password) => {
  const response = await fetch('/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password })
  });
  
  const result = await response.json();
  if (result.success) {
    // 保存token到localStorage
    localStorage.setItem('auth_token', result.data.access_token);
    return result.data;
  } else {
    throw new Error(result.error.message);
  }
};

// 创建带认证的API请求函数
const authenticatedFetch = async (url, options = {}) => {
  const token = localStorage.getItem('auth_token');
  
  const defaultOptions = {
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
      ...options.headers
    }
  };
  
  return fetch(url, { ...defaultOptions, ...options });
};

// 使用示例
const fetchUsers = async () => {
  const response = await authenticatedFetch('/api/users');
  return response.json();
};
```

## Token 生命周期

- **有效期**: JWT token 默认有效期为 7 天
- **自动过期**: token 过期后需要重新登录
- **无刷新机制**: 当前版本不支持 refresh token，过期后需重新登录

## 错误代码

| 错误代码 | HTTP状态码 | 说明 |
|---------|-----------|------|
| `UNAUTHORIZED` | 401 | 缺少Authorization header |
| `INVALID_TOKEN` | 401 | JWT token无效或格式错误 |
| `TOKEN_EXPIRED` | 401 | JWT token已过期 |
| `INVALID_CREDENTIALS` | 401 | 用户名或密码错误 |
| `USER_NOT_FOUND` | 404 | 用户不存在 |
| `USER_DISABLED` | 403 | 用户账户已被禁用 |
| `USER_ALREADY_EXISTS` | 409 | 注册时用户已存在 |
| `VALIDATION_ERROR` | 400 | 请求参数验证失败 |

## 安全最佳实践

1. **HTTPS**: 生产环境中务必使用 HTTPS 传输
2. **Token存储**:
   - 前端应用：使用 localStorage 或 sessionStorage
   - 移动应用：使用安全的密钥存储
3. **Token过期处理**: 实现自动检测token过期并引导用户重新登录
4. **密码安全**:
   - 密码长度至少6位
   - 建议使用包含数字、字母、特殊字符的强密码
5. **错误处理**: 不要在错误信息中泄露敏感信息

## 常见问题

### Q: 如何检查token是否有效？

A: 可以通过调用任何需要认证的API来验证token。如果返回401状态码，说明token无效或过期。

### Q: 忘记密码怎么办？

A: 当前版本暂不支持密码重置功能，需要联系管理员重置密码。

### Q: 可以同时在多个设备登录吗？

A: 可以。JWT token是无状态的，同一个用户可以在多个设备上同时使用不同的token。

### Q: 如何退出登录？

A: 客户端删除本地存储的token即可。JWT token在服务端不会被主动撤销，但会在过期时间后自动失效。
