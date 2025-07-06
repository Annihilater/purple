use crate::models::user::{CreateUser, User};
use sqlx::PgPool;
use anyhow::Result;

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
                balance, commission_type, commission_balance,
                t, u, d, transfer_enable, banned, is_admin,
                is_staff, remind_expire, remind_traffic,
                expired_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, 0, false, 0, 0, 0, 0, 0, false, false, false, true, true, 0, $6, $6)
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
            SELECT *
            FROM purple_user
            WHERE id = $1
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
            SELECT *
            FROM purple_user
            WHERE email = $1
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
                balance = $3,
                commission_type = $4,
                commission_balance = $5,
                transfer_enable = $6,
                banned = $7,
                is_admin = $8,
                is_staff = $9,
                group_id = $10,
                plan_id = $11,
                speed_limit = $12,
                remind_expire = $13,
                remind_traffic = $14,
                expired_at = $15,
                remarks = $16,
                updated_at = $17
            WHERE id = $18
            RETURNING *
            "#,
            user.email,
            user.password,
            user.balance,
            user.commission_type,
            user.commission_balance,
            user.transfer_enable,
            user.banned,
            user.is_admin,
            user.is_staff,
            user.group_id,
            user.plan_id,
            user.speed_limit,
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