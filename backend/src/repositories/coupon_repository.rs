use crate::models::coupon::{
    Coupon, CreateCouponRequest, UpdateCouponRequest, ValidateCouponResponse,
};
use anyhow::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CouponRepository {
    pool: PgPool,
}

impl CouponRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, coupon: &CreateCouponRequest) -> Result<Coupon> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let coupon = sqlx::query_as!(
            Coupon,
            r#"
            INSERT INTO purple_coupon (
                code, name, "type", value, show,
                limit_use, limit_use_with_user, limit_plan_ids, limit_period,
                started_at, ended_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12)
            RETURNING *
            "#,
            coupon.code,
            coupon.name,
            coupon.r#type,
            coupon.value,
            coupon.show,
            coupon.limit_use,
            coupon.limit_use_with_user,
            coupon.limit_plan_ids,
            coupon.limit_period,
            coupon.started_at,
            coupon.ended_at,
            now,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(coupon)
    }

    pub async fn find_all(&self, page: i32, page_size: i32) -> Result<(Vec<Coupon>, i64)> {
        let offset = (page - 1) * page_size;
        let coupons = sqlx::query_as!(
            Coupon,
            r#"
            SELECT * FROM purple_coupon
            ORDER BY id DESC
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!" FROM purple_coupon
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((coupons, total))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Coupon>> {
        let coupon = sqlx::query_as!(
            Coupon,
            r#"
            SELECT * FROM purple_coupon WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(coupon)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Coupon>> {
        let coupon = sqlx::query_as!(
            Coupon,
            r#"
            SELECT * FROM purple_coupon WHERE code = $1
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(coupon)
    }

    pub async fn update(&self, id: i32, coupon: &UpdateCouponRequest) -> Result<Coupon> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let coupon = sqlx::query_as!(
            Coupon,
            r#"
            UPDATE purple_coupon
            SET 
                code = COALESCE($1, code),
                name = COALESCE($2, name),
                "type" = COALESCE($3, "type"),
                value = COALESCE($4, value),
                show = COALESCE($5, show),
                limit_use = COALESCE($6, limit_use),
                limit_use_with_user = COALESCE($7, limit_use_with_user),
                limit_plan_ids = COALESCE($8, limit_plan_ids),
                limit_period = COALESCE($9, limit_period),
                started_at = COALESCE($10, started_at),
                ended_at = COALESCE($11, ended_at),
                updated_at = $12
            WHERE id = $13
            RETURNING *
            "#,
            coupon.code,
            coupon.name,
            coupon.r#type,
            coupon.value,
            coupon.show,
            coupon.limit_use,
            coupon.limit_use_with_user,
            coupon.limit_plan_ids,
            coupon.limit_period,
            coupon.started_at,
            coupon.ended_at,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(coupon)
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM purple_coupon
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
        only_enabled: bool,
        only_valid: bool,
    ) -> Result<(Vec<Coupon>, i64)> {
        let offset = (page - 1) * page_size;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let coupons = sqlx::query_as!(
            Coupon,
            r#"
            SELECT * FROM purple_coupon
            WHERE 
                ($1 = false OR show = true)
                AND ($2 = false OR (
                    started_at <= $3
                    AND ended_at >= $3
                ))
            ORDER BY id DESC
            LIMIT $4 OFFSET $5
            "#,
            only_enabled,
            only_valid,
            now,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!" FROM purple_coupon
            WHERE 
                ($1 = false OR show = true)
                AND ($2 = false OR (
                    started_at <= $3
                    AND ended_at >= $3
                ))
            "#,
            only_enabled,
            only_valid,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((coupons, total))
    }

    pub async fn validate_coupon(&self, code: &str, amount: i32) -> Result<ValidateCouponResponse> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let coupon = sqlx::query_as!(
            Coupon,
            r#"
            SELECT * FROM purple_coupon
            WHERE code = $1
            AND started_at <= $2
            AND ended_at >= $2
            AND show = true
            "#,
            code,
            now
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(coupon) = coupon {
            let discount_amount = if coupon.r#type {
                // 百分比折扣
                amount * coupon.value / 100
            } else {
                // 固定金额折扣
                std::cmp::min(coupon.value, amount)
            };

            Ok(ValidateCouponResponse {
                is_valid: true,
                message: Some("优惠券有效".to_string()),
                discount_amount: Some(discount_amount),
            })
        } else {
            Ok(ValidateCouponResponse {
                is_valid: false,
                message: Some("优惠券无效或已过期".to_string()),
                discount_amount: None,
            })
        }
    }

    pub async fn use_coupon(&self, id: i32) -> Result<Option<Coupon>> {
        // 由于数据库表结构中没有 used_count 字段，直接返回优惠券信息
        self.find_by_id(id).await
    }

    pub async fn get_stats(&self) -> Result<(i64, i64)> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        // 获取总优惠券数量
        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!" FROM purple_coupon
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // 获取有效且启用的优惠券数量
        let active = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!" FROM purple_coupon
            WHERE show = true
            AND started_at <= $1
            AND ended_at >= $1
            "#,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((total, active))
    }
}
