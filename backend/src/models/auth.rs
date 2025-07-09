use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[schema(title = "用户注册请求")]
pub struct RegisterRequest {
    /// 用户名，3-20个字符
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3-20个字符之间"))]
    #[schema(example = "testuser", min_length = 3, max_length = 20)]
    pub username: String,

    /// 邮箱地址，必须是有效格式
    #[validate(email(message = "请输入有效的邮箱地址"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    /// 登录密码，6-32个字符
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32个字符之间"))]
    #[schema(example = "password123", min_length = 6, max_length = 32)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[schema(title = "用户登录请求")]
pub struct LoginRequest {
    /// 邮箱地址，必须是有效格式
    #[validate(email(message = "请输入有效的邮箱地址"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    /// 登录密码，6-32个字符
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32个字符之间"))]
    #[schema(example = "password123", min_length = 6, max_length = 32)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "JWT令牌响应")]
pub struct TokenResponse {
    /// JWT访问令牌，用于API认证
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub access_token: String,

    /// 令牌类型，固定为Bearer
    #[schema(example = "Bearer")]
    pub token_type: String,

    /// 令牌有效期（秒），默认604800秒（7天）
    #[schema(example = 604800)]
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: i32, // user_id
    pub exp: i64, // expiration time
    pub iat: i64, // issued at
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(7))
            .expect("valid timestamp")
            .timestamp();

        Claims {
            sub: user_id,
            exp,
            iat: chrono::Utc::now().timestamp(),
        }
    }

    pub fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let key = EncodingKey::from_secret(jwt_secret.as_bytes());
        encode(&Header::default(), self, &key)
    }
}
