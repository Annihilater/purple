use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;

use crate::{
    models::{
        auth::{Claims, LoginRequest, RegisterRequest, TokenResponse},
        user::{CreateUser, User},
    },
    repositories::UserRepository,
};

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

    pub async fn register(&self, req: RegisterRequest) -> Result<User> {
        // 检查邮箱是否已存在
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            anyhow::bail!("邮箱已存在");
        }

        // 生成密码哈希
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("密码加密失败: {}", e))?
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
        Ok(user)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<TokenResponse> {
        // 查找用户（支持用户名或邮箱登录）
        let user = match self.user_repo.find_by_email(&req.username).await? {
            Some(user) => user,
            None => anyhow::bail!("用户名或密码错误"),
        };

        // 验证密码
        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|e| anyhow::anyhow!("密码哈希解析失败: {}", e))?;
        if Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            anyhow::bail!("用户名或密码错误");
        }

        // 检查账号状态
        if user.banned.unwrap_or(false) {
            anyhow::bail!("账号已被禁用");
        }

        // 生成JWT
        let claims = Claims::new(user.id);
        let token = claims.encode()?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: claims.exp - chrono::Utc::now().timestamp(),
        })
    }
}
