use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::{
    auth::{RegisterRequest, LoginRequest, TokenResponse, Claims},
    user::{User, CreateUserRequest},
};
use crate::repositories::UserRepository;

pub struct AuthService {
    user_repository: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repository: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<User> {
        // 检查用户名是否已存在
        if let Some(_) = self.user_repository.get_by_username(&request.username).await? {
            anyhow::bail!("用户名已存在");
        }

        // 检查邮箱是否已存在
        if let Some(_) = self.user_repository.get_by_email(&request.email).await? {
            anyhow::bail!("邮箱已存在");
        }

        // 生成密码哈希
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)?
            .to_string();

        // 创建用户
        let user = self.user_repository
            .create(&CreateUserRequest {
                username: request.username,
                email: request.email,
                password: password_hash,
            })
            .await?;

        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<TokenResponse> {
        // 获取用户
        let user = match self.user_repository.get_by_username(&request.username).await? {
            Some(user) => user,
            None => anyhow::bail!("用户名或密码错误"),
        };

        // 验证密码
        let parsed_hash = PasswordHash::new(&user.password)?;
        if !Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            anyhow::bail!("用户名或密码错误");
        }

        // 生成JWT
        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp();
        let claims = Claims {
            sub: user.id,
            exp: exp,
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: exp - now.timestamp(),
        })
    }
} 