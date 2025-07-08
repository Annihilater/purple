use crate::services::api::ApiClient;
use purple_shared::{LoginRequest, LoginResponse};

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
