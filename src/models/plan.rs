use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "plan_status", rename_all = "snake_case")]
pub enum PlanStatus {
    Active,
    Inactive,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Plan {
    pub id: i32,
    pub group_id: i32,
    pub transfer_enable: i32,
    pub name: String,
    pub speed_limit: Option<i32>,
    pub show: bool,
    pub sort: Option<i32>,
    pub renew: bool,
    pub content: Option<String>,
    pub month_price: Option<i32>,
    pub quarter_price: Option<i32>,
    pub half_year_price: Option<i32>,
    pub year_price: Option<i32>,
    pub two_year_price: Option<i32>,
    pub three_year_price: Option<i32>,
    pub onetime_price: Option<i32>,
    pub reset_price: Option<i32>,
    pub reset_traffic_method: Option<bool>,
    pub capacity_limit: Option<i32>,
    pub daily_unit_price: Option<i32>,
    pub transfer_unit_price: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePlanRequest {
    #[validate(range(min = 1))]
    pub group_id: i32,
    #[validate(range(min = 0))]
    pub transfer_enable: i32,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(range(min = 0))]
    pub speed_limit: Option<i32>,
    pub show: Option<bool>,
    #[validate(range(min = 0))]
    pub sort: Option<i32>,
    pub renew: Option<bool>,
    pub content: Option<String>,
    #[validate(range(min = 0))]
    pub month_price: Option<i32>,
    #[validate(range(min = 0))]
    pub quarter_price: Option<i32>,
    #[validate(range(min = 0))]
    pub half_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub two_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub three_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub onetime_price: Option<i32>,
    #[validate(range(min = 0))]
    pub reset_price: Option<i32>,
    pub reset_traffic_method: Option<bool>,
    #[validate(range(min = 0))]
    pub capacity_limit: Option<i32>,
    #[validate(range(min = 0))]
    pub daily_unit_price: Option<i32>,
    #[validate(range(min = 0))]
    pub transfer_unit_price: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePlanRequest {
    #[validate(range(min = 1))]
    pub group_id: Option<i32>,
    #[validate(range(min = 0))]
    pub transfer_enable: Option<i32>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(range(min = 0))]
    pub speed_limit: Option<i32>,
    pub show: Option<bool>,
    #[validate(range(min = 0))]
    pub sort: Option<i32>,
    pub renew: Option<bool>,
    pub content: Option<String>,
    #[validate(range(min = 0))]
    pub month_price: Option<i32>,
    #[validate(range(min = 0))]
    pub quarter_price: Option<i32>,
    #[validate(range(min = 0))]
    pub half_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub two_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub three_year_price: Option<i32>,
    #[validate(range(min = 0))]
    pub onetime_price: Option<i32>,
    #[validate(range(min = 0))]
    pub reset_price: Option<i32>,
    pub reset_traffic_method: Option<bool>,
    #[validate(range(min = 0))]
    pub capacity_limit: Option<i32>,
    #[validate(range(min = 0))]
    pub daily_unit_price: Option<i32>,
    #[validate(range(min = 0))]
    pub transfer_unit_price: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanResponse {
    pub id: i32,
    pub group_id: i32,
    pub transfer_enable: i32,
    pub name: String,
    pub speed_limit: Option<i32>,
    pub show: bool,
    pub sort: Option<i32>,
    pub renew: bool,
    pub content: Option<String>,
    pub month_price: Option<i32>,
    pub quarter_price: Option<i32>,
    pub half_year_price: Option<i32>,
    pub year_price: Option<i32>,
    pub two_year_price: Option<i32>,
    pub three_year_price: Option<i32>,
    pub onetime_price: Option<i32>,
    pub reset_price: Option<i32>,
    pub reset_traffic_method: Option<bool>,
    pub capacity_limit: Option<i32>,
    pub daily_unit_price: Option<i32>,
    pub transfer_unit_price: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}

impl From<Plan> for PlanResponse {
    fn from(plan: Plan) -> Self {
        Self {
            id: plan.id,
            group_id: plan.group_id,
            transfer_enable: plan.transfer_enable,
            name: plan.name,
            speed_limit: plan.speed_limit,
            show: plan.show,
            sort: plan.sort,
            renew: plan.renew,
            content: plan.content,
            month_price: plan.month_price,
            quarter_price: plan.quarter_price,
            half_year_price: plan.half_year_price,
            year_price: plan.year_price,
            two_year_price: plan.two_year_price,
            three_year_price: plan.three_year_price,
            onetime_price: plan.onetime_price,
            reset_price: plan.reset_price,
            reset_traffic_method: plan.reset_traffic_method,
            capacity_limit: plan.capacity_limit,
            daily_unit_price: plan.daily_unit_price,
            transfer_unit_price: plan.transfer_unit_price,
            created_at: plan.created_at,
            updated_at: plan.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanListResponse {
    pub plans: Vec<PlanResponse>,
    pub total: i64,
}
