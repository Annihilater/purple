use crate::services::auth::AuthService;
use purple_shared::ApiResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient;

impl ApiClient {
    const BASE_URL: &'static str = "http://127.0.0.1:8080";

    fn client() -> Client {
        Client::new()
    }

    fn get_auth_header() -> Option<String> {
        AuthService::get_token().map(|token| format!("Bearer {}", token))
    }

    pub async fn get<T>(endpoint: &str) -> Result<ApiResponse<T>, reqwest::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut request = Self::client().get(format!("{}{}", Self::BASE_URL, endpoint));

        if let Some(auth) = Self::get_auth_header() {
            request = request.header("Authorization", auth);
        }

        let response = request
            .header("Content-Type", "application/json")
            .send()
            .await?;

        response.json::<ApiResponse<T>>().await
    }

    pub async fn post<Req, Res>(
        endpoint: &str,
        data: &Req,
    ) -> Result<ApiResponse<Res>, reqwest::Error>
    where
        Req: Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        let mut request = Self::client().post(format!("{}{}", Self::BASE_URL, endpoint));

        if let Some(auth) = Self::get_auth_header() {
            request = request.header("Authorization", auth);
        }

        let response = request
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await?;

        response.json::<ApiResponse<Res>>().await
    }

    pub async fn put<Req, Res>(
        endpoint: &str,
        data: &Req,
    ) -> Result<ApiResponse<Res>, reqwest::Error>
    where
        Req: Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        let mut request = Self::client().put(format!("{}{}", Self::BASE_URL, endpoint));

        if let Some(auth) = Self::get_auth_header() {
            request = request.header("Authorization", auth);
        }

        let response = request
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await?;

        response.json::<ApiResponse<Res>>().await
    }

    pub async fn delete<T>(endpoint: &str) -> Result<ApiResponse<T>, reqwest::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut request = Self::client().delete(format!("{}{}", Self::BASE_URL, endpoint));

        if let Some(auth) = Self::get_auth_header() {
            request = request.header("Authorization", auth);
        }

        let response = request
            .header("Content-Type", "application/json")
            .send()
            .await?;

        response.json::<ApiResponse<T>>().await
    }
} 