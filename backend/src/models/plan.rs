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

/// 套餐查询参数
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanQuery {
    /// 组ID
    pub group_id: Option<i32>,
    /// 是否显示
    pub show: Option<bool>,
    /// 是否允许续费
    pub renew: Option<bool>,
    /// 页码
    pub page: Option<u64>,
    /// 每页大小
    pub page_size: Option<u64>,
}

/// 套餐分页响应数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanPageData {
    /// 套餐列表
    pub items: Vec<Plan>,
    /// 总数
    pub total: u64,
    /// 当前页
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_prev: bool,
}

/// 套餐统计信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanStats {
    /// 总套餐数
    pub total_plans: i64,
    /// 显示中的套餐数
    pub active_plans: i64,
    /// 隐藏的套餐数
    pub hidden_plans: i64,
    /// 允许续费的套餐数
    pub renewable_plans: i64,
}

/// 套餐可用性信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanAvailability {
    /// 套餐ID
    pub id: i32,
    /// 套餐名称
    pub name: String,
    /// 是否可用
    pub available: bool,
    /// 不可用原因
    pub reason: Option<String>,
}

/// 套餐价格信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlanPricing {
    /// 套餐ID
    pub id: i32,
    /// 套餐名称
    pub name: String,
    /// 价格信息
    pub pricing: Vec<PriceOption>,
}

/// 价格选项
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PriceOption {
    /// 周期类型
    pub period_type: String,
    /// 周期天数
    pub period_days: i32,
    /// 价格（分）
    pub price: i32,
    /// 是否可用
    pub available: bool,
}

/// 格式化套餐流量
pub fn format_plan_traffic(bytes: i64) -> String {
    const GB: i64 = 1024 * 1024 * 1024;
    const TB: i64 = 1024 * GB;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else {
        format!("{} MB", bytes / (1024 * 1024))
    }
}

/// 格式化套餐价格
pub fn format_plan_price(price_in_cents: i32) -> String {
    format!("¥{:.2}", price_in_cents as f64 / 100.0)
}

/// 计算套餐折扣
pub fn calculate_plan_discount(original_price: i32, discounted_price: i32) -> f64 {
    if original_price == 0 {
        return 0.0;
    }

    let discount = (original_price - discounted_price) as f64 / original_price as f64;
    discount * 100.0
}
