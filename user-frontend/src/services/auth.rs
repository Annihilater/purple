use crate::services::api::ApiClient;
use purple_shared::{LoginRequest, LoginResponse, RegisterRequest, User};

// 登录函数，简化接口便于直接使用
pub async fn login(username: &str, password: &str) -> Result<(), String> {
    let request = LoginRequest {
        email: username.to_string(),
        password: password.to_string(),
    };

    let login_response = AuthService::login(request).await?;
    AuthService::save_token(&login_response.token);

    Ok(())
}

// 检查是否已认证
pub fn is_authenticated() -> bool {
    AuthService::get_token().is_some()
}

// 登出
pub fn logout() {
    AuthService::remove_token();
}

pub struct AuthService;

impl AuthService {
    pub async fn login(request: LoginRequest) -> Result<LoginResponse, String> {
        let response = ApiClient::post::<LoginRequest, LoginResponse>("/api/auth/login", &request)
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if response.success {
            response.data.ok_or_else(|| "登录响应数据为空".to_string())
        } else {
            Err(response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "登录失败".to_string()))
        }
    }

    pub async fn register(request: RegisterRequest) -> Result<User, String> {
        let response = ApiClient::post::<RegisterRequest, User>("/api/auth/register", &request)
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if response.success {
            response.data.ok_or_else(|| "注册响应数据为空".to_string())
        } else {
            Err(response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "注册失败".to_string()))
        }
    }

    pub fn save_token(token: &str) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("auth_token", token);
        }
    }

    pub fn get_token() -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("auth_token").ok().flatten())
    }

    pub fn remove_token() {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item("auth_token");
        }
    }
}
