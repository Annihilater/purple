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
    /// 创建新的认证服务实例
    ///
    /// # 参数
    ///
    /// * `user_repo` - 用户仓库实例
    /// * `jwt_secret` - JWT 密钥
    ///
    /// # 示例
    ///
    /// ```
    /// use purple_backend::services::AuthService;
    /// use purple_backend::repositories::UserRepository;
    /// use sqlx::PgPool;
    ///
    /// async fn create_auth_service() {
    ///     let pool = PgPool::connect("postgres://localhost/purple").await.unwrap();
    ///     let user_repo = UserRepository::new(pool);
    ///     let jwt_secret = "your-secret-key".to_string();
    ///     let auth_service = AuthService::new(user_repo, jwt_secret);
    /// }
    /// ```
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

    pub async fn login(&self, req: LoginRequest) -> Result<purple_shared::LoginResponse> {
        // 查找用户（使用邮箱登录）
        let user = match self.user_repo.find_by_email(&req.email).await? {
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

        // 转换为共享的User类型
        let shared_user = purple_shared::User {
            id: user.id,
            email: user.email,
            username: None, // 后端User模型没有username字段
            is_admin: user.is_admin.unwrap_or(false),
            is_enabled: !user.banned.unwrap_or(false),
            created_at: chrono::DateTime::from_timestamp(user.created_at as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            updated_at: chrono::DateTime::from_timestamp(user.updated_at as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
        };

        Ok(purple_shared::LoginResponse {
            token,
            user: shared_user,
        })
    }
}
