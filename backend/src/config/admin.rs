use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use tracing::{info, warn};
use uuid::Uuid;

use crate::config::AdminConfig;

/// 初始化管理员账户
///
/// 检查管理员账户是否存在，如果不存在则创建，如果存在但密码不匹配则更新密码
pub async fn initialize_admin_account(pool: &PgPool, admin_config: &AdminConfig) -> Result<()> {
    info!("开始初始化管理员账户...");

    // 查询是否存在管理员账户
    let existing_admin = sqlx::query!(
        "SELECT id, email, password FROM purple_user WHERE is_admin = true AND email = $1",
        admin_config.email
    )
    .fetch_optional(pool)
    .await?;

    match existing_admin {
        Some(admin) => {
            info!("发现现有管理员账户: {}", admin.email);

            // 检查密码是否匹配
            if verify_password(&admin_config.password, &admin.password)? {
                info!("管理员密码验证成功，无需更新");
            } else {
                warn!("管理员密码不匹配，正在更新密码...");
                update_admin_password(pool, admin.id, &admin_config.password).await?;
                info!("管理员密码已更新");
            }
        }
        None => {
            info!("未发现管理员账户，正在创建新的管理员账户...");
            create_admin_account(pool, admin_config).await?;
            info!("管理员账户创建成功: {}", admin_config.email);
        }
    }

    Ok(())
}

/// 创建新的管理员账户
async fn create_admin_account(pool: &PgPool, admin_config: &AdminConfig) -> Result<()> {
    let hashed_password = hash_password(&admin_config.password)?;
    let uuid = Uuid::new_v4().to_string();
    let token = generate_random_token();
    let now = chrono::Utc::now().timestamp() as i32;

    sqlx::query!(
        r#"
        INSERT INTO purple_user (
            email, password, uuid, token, is_admin, is_staff, 
            balance, commission_balance, t, u, d, transfer_enable, 
            banned, remind_expire, remind_traffic, created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, true, true,
            0, 0, 0, 0, 0, 0,
            false, true, true, $5, $5
        )
        "#,
        admin_config.email,
        hashed_password,
        uuid,
        token,
        now
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// 更新管理员密码
async fn update_admin_password(pool: &PgPool, admin_id: i32, new_password: &str) -> Result<()> {
    let hashed_password = hash_password(new_password)?;
    let now = chrono::Utc::now().timestamp() as i32;

    sqlx::query!(
        "UPDATE purple_user SET password = $1, updated_at = $2 WHERE id = $3",
        hashed_password,
        now,
        admin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// 生成随机token
fn generate_random_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

/// 验证管理员账户状态
pub async fn verify_admin_account(pool: &PgPool, email: &str) -> Result<bool> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM purple_user WHERE email = $1 AND is_admin = true AND banned = false",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(count.unwrap_or(0) > 0)
}

/// 获取管理员账户信息
pub async fn get_admin_info(pool: &PgPool, email: &str) -> Result<Option<AdminInfo>> {
    let admin = sqlx::query_as!(
        AdminInfo,
        r#"
        SELECT id, email, is_admin, is_staff, banned, created_at, updated_at
        FROM purple_user 
        WHERE email = $1 AND is_admin = true
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(admin)
}

/// 管理员信息结构
#[derive(Debug)]
pub struct AdminInfo {
    pub id: i32,
    pub email: String,
    pub is_admin: bool,
    pub is_staff: bool,
    pub banned: bool,
    pub created_at: i32,
    pub updated_at: i32,
}

/// 哈希密码
fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("密码哈希失败: {}", e))?;
    Ok(password_hash.to_string())
}

/// 验证密码
fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("密码哈希解析失败: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
