use crate::models::user::{CreateUser, User};
use anyhow::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: CreateUser) -> Result<User> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO purple_user (
                email, password, invite_user_id, uuid, token,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $6)
            RETURNING *
            "#,
            user.email,
            user.password,
            user.invite_user_id,
            user.uuid,
            user.token,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM purple_user WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM purple_user WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, user: &User) -> Result<User> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE purple_user
            SET 
                email = $1,
                password = $2,
                password_algo = $3,
                password_salt = $4,
                telegram_id = $5,
                invite_user_id = $6,
                balance = $7,
                discount = $8,
                commission_type = $9,
                commission_rate = $10,
                commission_balance = $11,
                t = $12,
                u = $13,
                d = $14,
                transfer_enable = $15,
                banned = $16,
                is_admin = $17,
                is_staff = $18,
                last_login_at = $19,
                last_login_ip = $20,
                uuid = $21,
                group_id = $22,
                plan_id = $23,
                speed_limit = $24,
                token = $25,
                remind_expire = $26,
                remind_traffic = $27,
                expired_at = $28,
                remarks = $29,
                updated_at = $30
            WHERE id = $31
            RETURNING *
            "#,
            user.email,
            user.password,
            user.password_algo,
            user.password_salt,
            user.telegram_id,
            user.invite_user_id,
            user.balance,
            user.discount,
            user.commission_type,
            user.commission_rate,
            user.commission_balance,
            user.t,
            user.u,
            user.d,
            user.transfer_enable,
            user.banned,
            user.is_admin,
            user.is_staff,
            user.last_login_at,
            user.last_login_ip,
            user.uuid,
            user.group_id,
            user.plan_id,
            user.speed_limit,
            user.token,
            user.remind_expire,
            user.remind_traffic,
            user.expired_at,
            user.remarks,
            now,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM purple_user
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
