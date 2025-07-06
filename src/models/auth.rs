use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
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
