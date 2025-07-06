use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    models::{
        auth::{LoginRequest, RegisterRequest},
        user::CreateUser,
    },
    repositories::UserRepository,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // 用户ID
    exp: i64, // 过期时间
}

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<i32> {
        // 检查邮箱是否已存在
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            anyhow::bail!("Email already exists");
        }

        // 生成密码哈希
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .to_string();

        // 创建用户
        let user = CreateUser {
            email: req.email,
            password: password_hash,
            invite_user_id: None, // TODO: 处理邀请码
            uuid: Uuid::new_v4().to_string(),
            token: Uuid::new_v4().to_string(),
        };

        let user = self.user_repo.create(user).await?;
        Ok(user.id)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<(String, i32)> {
        // 查找用户
        let user = match self.user_repo.find_by_email(&req.username).await? {
            Some(user) => user,
            None => anyhow::bail!("Invalid username or password"),
        };

        // 验证密码
        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        if !Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            anyhow::bail!("Invalid username or password");
        }

        // 检查账号状态
        if user.banned.unwrap_or(false) {
            anyhow::bail!("Account is banned");
        }

        // 生成JWT
        let exp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 + 24 * 3600; // 24小时过期
        let claims = Claims { sub: user.id, exp };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok((
            token,
            (exp - SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64) as i32,
        ))
    }
}
