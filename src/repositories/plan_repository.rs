use crate::models::plan::{CreatePlanRequest, Plan, UpdatePlanRequest};
use anyhow::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PlanRepository {
    pool: PgPool,
}

impl PlanRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, plan: &CreatePlanRequest) -> Result<Plan> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let plan = sqlx::query_as!(
            Plan,
            r#"
            INSERT INTO purple_plan (
                group_id, transfer_enable, name, speed_limit, show, sort, renew, content,
                month_price, quarter_price, half_year_price, year_price, two_year_price,
                three_year_price, onetime_price, reset_price, reset_traffic_method,
                capacity_limit, daily_unit_price, transfer_unit_price, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $21)
            RETURNING *
            "#,
            plan.group_id,
            plan.transfer_enable,
            plan.name,
            plan.speed_limit,
            plan.show.unwrap_or(false),
            plan.sort,
            plan.renew.unwrap_or(true),
            plan.content,
            plan.month_price,
            plan.quarter_price,
            plan.half_year_price,
            plan.year_price,
            plan.two_year_price,
            plan.three_year_price,
            plan.onetime_price,
            plan.reset_price,
            plan.reset_traffic_method,
            plan.capacity_limit,
            plan.daily_unit_price,
            plan.transfer_unit_price,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn find_all(&self, page: i32, page_size: i32) -> Result<(Vec<Plan>, i64)> {
        let offset = (page - 1) * page_size;

        let plans = sqlx::query_as!(
            Plan,
            r#"
            SELECT * FROM purple_plan
            ORDER BY sort ASC NULLS LAST, id DESC
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM purple_plan
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        Ok((plans, total))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Plan>> {
        let plan = sqlx::query_as!(
            Plan,
            r#"
            SELECT * FROM purple_plan WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn update(&self, id: i32, plan: &UpdatePlanRequest) -> Result<Plan> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i32;

        let plan = sqlx::query_as!(
            Plan,
            r#"
            UPDATE purple_plan
            SET 
                group_id = COALESCE($1, group_id),
                transfer_enable = COALESCE($2, transfer_enable),
                name = COALESCE($3, name),
                speed_limit = COALESCE($4, speed_limit),
                show = COALESCE($5, show),
                sort = COALESCE($6, sort),
                renew = COALESCE($7, renew),
                content = COALESCE($8, content),
                month_price = COALESCE($9, month_price),
                quarter_price = COALESCE($10, quarter_price),
                half_year_price = COALESCE($11, half_year_price),
                year_price = COALESCE($12, year_price),
                two_year_price = COALESCE($13, two_year_price),
                three_year_price = COALESCE($14, three_year_price),
                onetime_price = COALESCE($15, onetime_price),
                reset_price = COALESCE($16, reset_price),
                reset_traffic_method = COALESCE($17, reset_traffic_method),
                capacity_limit = COALESCE($18, capacity_limit),
                daily_unit_price = COALESCE($19, daily_unit_price),
                transfer_unit_price = COALESCE($20, transfer_unit_price),
                updated_at = $21
            WHERE id = $22
            RETURNING *
            "#,
            plan.group_id,
            plan.transfer_enable,
            plan.name,
            plan.speed_limit,
            plan.show,
            plan.sort,
            plan.renew,
            plan.content,
            plan.month_price,
            plan.quarter_price,
            plan.half_year_price,
            plan.year_price,
            plan.two_year_price,
            plan.three_year_price,
            plan.onetime_price,
            plan.reset_price,
            plan.reset_traffic_method,
            plan.capacity_limit,
            plan.daily_unit_price,
            plan.transfer_unit_price,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM purple_plan
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_enabled(&self) -> Result<Vec<Plan>> {
        let plans = sqlx::query_as!(
            Plan,
            r#"
            SELECT * FROM purple_plan
            WHERE show = true
            ORDER BY sort ASC NULLS LAST, id DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(plans)
    }

    pub async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<Plan>> {
        let plans = sqlx::query_as!(
            Plan,
            r#"
            SELECT * FROM purple_plan
            WHERE id = ANY($1)
            ORDER BY sort ASC NULLS LAST, id DESC
            "#,
            ids as &[i32]
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(plans)
    }

    pub async fn find_by_group_id(&self, group_id: i32) -> Result<Vec<Plan>> {
        let plans = sqlx::query_as!(
            Plan,
            r#"
            SELECT * FROM purple_plan
            WHERE group_id = $1
            ORDER BY sort ASC NULLS LAST, id DESC
            "#,
            group_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(plans)
    }
}
