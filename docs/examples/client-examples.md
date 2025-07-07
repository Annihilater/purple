# 客户端调用示例

本文档提供了使用不同编程语言和工具调用 Purple API 的完整示例。

## JavaScript / TypeScript 示例

### 基础 API 客户端
```javascript
class PurpleAPIClient {
    constructor(baseURL = 'http://127.0.0.1:8080', token = null) {
        this.baseURL = baseURL;
        this.token = token;
    }

    // 设置认证令牌
    setToken(token) {
        this.token = token;
    }

    // 通用请求方法
    async request(endpoint, options = {}) {
        const url = `${ this.baseURL }${ endpoint } `;
        const headers = {
            'Content-Type': 'application/json',
            ...options.headers
        };

        if (this.token) {
            headers['Authorization'] = `Bearer ${ this.token } `;
        }

        const response = await fetch(url, {
            ...options,
            headers
        });

        const data = await response.json();

        if (!data.success) {
            throw new Error(`API Error: ${ data.error.code } - ${ data.error.message } `);
        }

        return data;
    }

    // 认证相关方法
    async register(username, email, password) {
        return this.request('/api/auth/register', {
            method: 'POST',
            body: JSON.stringify({ username, email, password })
        });
    }

    async login(username, password) {
        const result = await this.request('/api/auth/login', {
            method: 'POST',
            body: JSON.stringify({ username, password })
        });
        
        if (result.data && result.data.access_token) {
            this.setToken(result.data.access_token);
        }
        
        return result;
    }

    // 用户管理方法
    async getUsers(page = 1, pageSize = 20) {
        return this.request(`/ api / users ? page = ${ page }& page_size=${ pageSize } `);
    }

    async getUser(userId) {
        return this.request(`/ api / users / ${ userId } `);
    }

    async createUser(userData) {
        return this.request('/api/users', {
            method: 'POST',
            body: JSON.stringify(userData)
        });
    }

    async updateUser(userId, userData) {
        return this.request(`/ api / users / ${ userId } `, {
            method: 'PUT',
            body: JSON.stringify(userData)
        });
    }

    async deleteUser(userId) {
        return this.request(`/ api / users / ${ userId } `, {
            method: 'DELETE'
        });
    }

    // 套餐管理方法
    async getPlans(page = 1, pageSize = 20) {
        return this.request(`/ api / plans ? page = ${ page }& page_size=${ pageSize } `);
    }

    async getEnabledPlans() {
        return this.request('/api/plans/enabled');
    }

    async createPlan(planData) {
        return this.request('/api/plans', {
            method: 'POST',
            body: JSON.stringify(planData)
        });
    }

    // 优惠券管理方法
    async getCoupons(page = 1, pageSize = 20) {
        return this.request(`/ api / coupons ? page = ${ page }& page_size=${ pageSize } `);
    }

    async createCoupon(couponData) {
        return this.request('/api/coupons', {
            method: 'POST',
            body: JSON.stringify(couponData)
        });
    }

    async verifyCoupon(code, amount) {
        return this.request('/api/coupons/verify', {
            method: 'POST',
            body: JSON.stringify({ code, amount })
        });
    }

    // 健康检查
    async healthCheck() {
        return this.request('/health');
    }
}

// 使用示例
async function example() {
    const client = new PurpleAPIClient();

    try {
        // 1. 健康检查
        const health = await client.healthCheck();
        console.log('Health check:', health);

        // 2. 用户登录
        const loginResult = await client.login('admin', 'password123');
        console.log('Login successful:', loginResult);

        // 3. 获取用户列表
        const users = await client.getUsers(1, 10);
        console.log('Users:', users);

        // 4. 获取套餐列表
        const plans = await client.getPlans();
        console.log('Plans:', plans);

    } catch (error) {
        console.error('API Error:', error.message);
    }
}

export default PurpleAPIClient;
```

### React Hooks 示例
    ```jsx
import React, { useState, useEffect, createContext, useContext } from 'react';

// API Context
const APIContext = createContext();

export const APIProvider = ({ children }) => {
    const [client] = useState(() => new PurpleAPIClient());
    const [user, setUser] = useState(null);
    const [token, setToken] = useState(localStorage.getItem('purple_token'));

    useEffect(() => {
        if (token) {
            client.setToken(token);
            // 可以在这里验证token有效性
        }
    }, [token, client]);

    const login = async (username, password) => {
        try {
            const result = await client.login(username, password);
            const newToken = result.data.access_token;
            
            setToken(newToken);
            localStorage.setItem('purple_token', newToken);
            
            return result;
        } catch (error) {
            throw error;
        }
    };

    const logout = () => {
        setToken(null);
        setUser(null);
        localStorage.removeItem('purple_token');
        client.setToken(null);
    };

    return (
        <APIContext.Provider value={{
            client,
            user,
            token,
            login,
            logout,
            isAuthenticated: !!token
        }}>
            {children}
        </APIContext.Provider>
    );
};

// 自定义 Hooks
export const useAPI = () => {
    const context = useContext(APIContext);
    if (!context) {
        throw new Error('useAPI must be used within APIProvider');
    }
    return context;
};

export const useUsers = (page = 1, pageSize = 20) => {
    const { client } = useAPI();
    const [users, setUsers] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    useEffect(() => {
        const fetchUsers = async () => {
            setLoading(true);
            setError(null);
            
            try {
                const result = await client.getUsers(page, pageSize);
                setUsers(result.data || []);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        fetchUsers();
    }, [client, page, pageSize]);

    return { users, loading, error };
};

// 组件示例
export const UserList = () => {
    const { users, loading, error } = useUsers();

    if (loading) return <div>Loading...</div>;
    if (error) return <div>Error: {error}</div>;

    return (
        <div>
            <h2>Users</h2>
            <ul>
                {users.map(user => (
                    <li key={user.id}>
                        {user.username} - {user.email}
                    </li>
                ))}
            </ul>
        </div>
    );
};
```

## Python 示例

### 基础客户端类
    ```python
import requests
import json
from typing import Optional, Dict, Any, List

class PurpleAPIClient:
    def __init__(self, base_url: str = "http://127.0.0.1:8080", token: Optional[str] = None):
        self.base_url = base_url.rstrip('/')
        self.token = token
        self.session = requests.Session()
        
        if token:
            self.set_token(token)
    
    def set_token(self, token: str):
        """设置认证令牌"""
        self.token = token
        self.session.headers.update({'Authorization': f'Bearer {token}'})
    
    def _request(self, method: str, endpoint: str, **kwargs) -> Dict[Any, Any]:
        """通用请求方法"""
        url = f"{self.base_url}{endpoint}"
        
        # 设置默认头部
        headers = kwargs.get('headers', {})
        headers.setdefault('Content-Type', 'application/json')
        kwargs['headers'] = headers
        
        response = self.session.request(method, url, **kwargs)
        response.raise_for_status()
        
        data = response.json()
        
        if not data.get('success', False):
            error_info = data.get('error', {})
            raise Exception(f"API Error: {error_info.get('code')} - {error_info.get('message')}")
        
        return data
    
    # 认证方法
    def register(self, username: str, email: str, password: str) -> Dict[Any, Any]:
        """用户注册"""
        payload = {
            'username': username,
            'email': email,
            'password': password
        }
        return self._request('POST', '/api/auth/register', json=payload)
    
    def login(self, username: str, password: str) -> Dict[Any, Any]:
        """用户登录"""
        payload = {
            'username': username,
            'password': password
        }
        result = self._request('POST', '/api/auth/login', json=payload)
        
        if result.get('data', {}).get('access_token'):
            self.set_token(result['data']['access_token'])
        
        return result
    
    # 用户管理方法
    def get_users(self, page: int = 1, page_size: int = 20) -> Dict[Any, Any]:
        """获取用户列表"""
        params = {'page': page, 'page_size': page_size}
        return self._request('GET', '/api/users', params=params)
    
    def get_user(self, user_id: int) -> Dict[Any, Any]:
        """获取用户详情"""
        return self._request('GET', f'/api/users/{user_id}')
    
    def create_user(self, user_data: Dict[str, Any]) -> Dict[Any, Any]:
        """创建用户"""
        return self._request('POST', '/api/users', json=user_data)
    
    def update_user(self, user_id: int, user_data: Dict[str, Any]) -> Dict[Any, Any]:
        """更新用户"""
        return self._request('PUT', f'/api/users/{user_id}', json=user_data)
    
    def delete_user(self, user_id: int) -> Dict[Any, Any]:
        """删除用户"""
        return self._request('DELETE', f'/api/users/{user_id}')
    
    # 套餐管理方法
    def get_plans(self, page: int = 1, page_size: int = 20) -> Dict[Any, Any]:
        """获取套餐列表"""
        params = {'page': page, 'page_size': page_size}
        return self._request('GET', '/api/plans', params=params)
    
    def get_enabled_plans(self) -> Dict[Any, Any]:
        """获取启用的套餐"""
        return self._request('GET', '/api/plans/enabled')
    
    def create_plan(self, plan_data: Dict[str, Any]) -> Dict[Any, Any]:
        """创建套餐"""
        return self._request('POST', '/api/plans', json=plan_data)
    
    # 优惠券管理方法
    def get_coupons(self, page: int = 1, page_size: int = 20) -> Dict[Any, Any]:
        """获取优惠券列表"""
        params = {'page': page, 'page_size': page_size}
        return self._request('GET', '/api/coupons', params=params)
    
    def create_coupon(self, coupon_data: Dict[str, Any]) -> Dict[Any, Any]:
        """创建优惠券"""
        return self._request('POST', '/api/coupons', json=coupon_data)
    
    def verify_coupon(self, code: str, amount: int) -> Dict[Any, Any]:
        """验证优惠券"""
        payload = {'code': code, 'amount': amount}
        return self._request('POST', '/api/coupons/verify', json=payload)
    
    # 健康检查
    def health_check(self) -> Dict[Any, Any]:
        """健康检查"""
        return self._request('GET', '/health')


# 使用示例
def main():
    client = PurpleAPIClient()
    
    try:
        # 健康检查
        health = client.health_check()
        print(f"Health check: {health}")
        
        # 用户登录
        login_result = client.login('admin', 'password123')
        print(f"Login successful: {login_result}")
        
        # 获取用户列表
        users = client.get_users(page=1, page_size=10)
        print(f"Users: {users}")
        
        # 获取套餐列表
        plans = client.get_plans()
        print(f"Plans: {plans}")
        
    except Exception as e:
        print(f"API Error: {e}")


if __name__ == "__main__":
    main()
```

### Django 集成示例
    ```python
# settings.py
PURPLE_API_CONFIG = {
    'BASE_URL': 'http://127.0.0.1:8080',
    'TIMEOUT': 30,
}

# services.py
from django.conf import settings
from .purple_client import PurpleAPIClient

class PurpleService:
    def __init__(self):
        config = settings.PURPLE_API_CONFIG
        self.client = PurpleAPIClient(base_url=config['BASE_URL'])
    
    def authenticate_user(self, username: str, password: str) -> Optional[str]:
        """用户认证并返回token"""
        try:
            result = self.client.login(username, password)
            return result.get('data', {}).get('access_token')
        except Exception:
            return None
    
    def get_user_plans(self, token: str) -> List[Dict]:
        """获取用户可用套餐"""
        client = PurpleAPIClient(token=token)
        try:
            result = client.get_enabled_plans()
            return result.get('data', [])
        except Exception:
            return []

# views.py
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.utils.decorators import method_decorator
from django.views import View
import json

@method_decorator(csrf_exempt, name='dispatch')
class PurpleProxyView(View):
    def __init__(self):
        super().__init__()
        self.purple_service = PurpleService()
    
    def post(self, request):
        data = json.loads(request.body)
        action = data.get('action')
        
        if action == 'login':
            username = data.get('username')
            password = data.get('password')
            token = self.purple_service.authenticate_user(username, password)
            
            if token:
                return JsonResponse({'success': True, 'token': token})
            else:
                return JsonResponse({'success': False, 'error': 'Invalid credentials'})
        
        return JsonResponse({'success': False, 'error': 'Unknown action'})
```

## Go 示例

    ```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "io/ioutil"
    "net/http"
    "time"
)

type PurpleClient struct {
    BaseURL string
    Token   string
    Client  *http.Client
}

type APIResponse struct {
    Success bool                   `json: "success"`
    Data    interface{}           `json: "data,omitempty"`
    Error   *ErrorInfo            `json: "error,omitempty"`
    Meta    map[string]interface{} `json: "meta"`
}

type ErrorInfo struct {
    Code    string `json: "code"`
    Message string `json: "message"`
    Details string `json: "details,omitempty"`
}

func NewPurpleClient(baseURL string) *PurpleClient {
    return &PurpleClient{
        BaseURL: baseURL,
        Client: &http.Client{
            Timeout: 30 * time.Second,
        },
    }
}

func (c *PurpleClient) SetToken(token string) {
    c.Token = token
}

func (c *PurpleClient) makeRequest(method, endpoint string, payload interface{}) (*APIResponse, error) {
    url := c.BaseURL + endpoint
    
    var bodyReader *bytes.Reader
    if payload != nil {
        jsonData, err := json.Marshal(payload)
        if err != nil {
            return nil, err
        }
        bodyReader = bytes.NewReader(jsonData)
    } else {
        bodyReader = bytes.NewReader([]byte{})
    }
    
    req, err := http.NewRequest(method, url, bodyReader)
    if err != nil {
        return nil, err
    }
    
    req.Header.Set("Content-Type", "application/json")
    if c.Token != "" {
        req.Header.Set("Authorization", "Bearer "+c.Token)
    }
    
    resp, err := c.Client.Do(req)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        return nil, err
    }
    
    var apiResp APIResponse
    if err := json.Unmarshal(body, &apiResp); err != nil {
        return nil, err
    }
    
    if !apiResp.Success && apiResp.Error != nil {
        return nil, fmt.Errorf("API Error: %s - %s", apiResp.Error.Code, apiResp.Error.Message)
    }
    
    return &apiResp, nil
}

// 认证方法
func (c *PurpleClient) Login(username, password string) (*APIResponse, error) {
    payload := map[string]string{
        "username": username,
        "password": password,
    }
    
    resp, err := c.makeRequest("POST", "/api/auth/login", payload)
    if err != nil {
        return nil, err
    }
    
    // 自动设置token
    if data, ok := resp.Data.(map[string]interface{}); ok {
        if token, ok := data["access_token"].(string); ok {
            c.SetToken(token)
        }
    }
    
    return resp, nil
}

// 用户管理方法
func (c *PurpleClient) GetUsers(page, pageSize int) (*APIResponse, error) {
    endpoint := fmt.Sprintf("/api/users?page=%d&page_size=%d", page, pageSize)
    return c.makeRequest("GET", endpoint, nil)
}

func (c *PurpleClient) CreateUser(userData map[string]interface{}) (*APIResponse, error) {
    return c.makeRequest("POST", "/api/users", userData)
}

// 健康检查
func (c *PurpleClient) HealthCheck() (*APIResponse, error) {
    return c.makeRequest("GET", "/health", nil)
}

func main() {
    client := NewPurpleClient("http://127.0.0.1:8080")
    
    // 健康检查
    health, err := client.HealthCheck()
    if err != nil {
        fmt.Printf("Health check failed: %v\n", err)
        return
    }
    fmt.Printf("Health check: %+v\n", health)
    
    // 用户登录
    loginResp, err := client.Login("admin", "password123")
    if err != nil {
        fmt.Printf("Login failed: %v\n", err)
        return
    }
    fmt.Printf("Login successful: %+v\n", loginResp)
    
    // 获取用户列表
    users, err := client.GetUsers(1, 10)
    if err != nil {
        fmt.Printf("Get users failed: %v\n", err)
        return
    }
    fmt.Printf("Users: %+v\n", users)
}
```

## cURL 示例

### 基础使用
    ```bash
#!/bin/bash

# 设置基础变量
BASE_URL="http://127.0.0.1:8080"
TOKEN=""

# 健康检查
curl -X GET "$BASE_URL/health" \
  -H "Content-Type: application/json" | jq .

# 用户登录
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password123"
  }')

echo "Login response: $LOGIN_RESPONSE"

# 提取token
TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.data.access_token')
echo "Token: $TOKEN"

# 获取用户列表
curl -X GET "$BASE_URL/api/users?page=1&page_size=10" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" | jq .

# 创建用户
curl -X POST "$BASE_URL/api/users" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "email": "newuser@example.com",
    "password": "password123"
  }' | jq .

# 获取套餐列表
curl -X GET "$BASE_URL/api/plans" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" | jq .
```

这些示例展示了如何在不同的编程环境中集成和使用 Purple API，提供了完整的错误处理、认证管理和数据操作功能。