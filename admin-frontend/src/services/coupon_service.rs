use crate::services::api::ApiClient;
use purple_shared::ApiResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 后端优惠券响应类型 (匹配后端模型)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendCouponResponse {
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: bool, // true = 固定金额, false = 百分比
    pub value: i32,
    pub show: bool,
    pub limit_use: Option<i32>,
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: i32,
    pub ended_at: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponPageResponse {
    pub data: Vec<BackendCouponResponse>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

// 前端优惠券类型 (适配前端页面)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coupon {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub type_: String,                    // "fixed" or "percentage"
    pub value: u32,                       // 固定金额（分）或百分比
    pub min_amount: Option<u32>,          // 最小使用金额（分）
    pub max_amount: Option<u32>,          // 最大优惠金额（分），仅百分比有效
    pub limit_quota: Option<u32>,         // 使用次数限制
    pub used_quota: u32,                  // 已使用次数
    pub limit_use_with_user: Option<u32>, // 每用户限制次数
    pub limit_plan_ids: Option<Vec<u32>>, // 限制套餐ID
    pub limit_period: Option<String>,     // 限制周期
    pub started_at: Option<String>,       // 开始时间
    pub ended_at: Option<String>,         // 结束时间
    pub status: bool,                     // 状态：true启用，false禁用
    pub created_at: String,
    pub updated_at: String,
}

// 转换函数：将后端响应转换为前端格式
impl From<BackendCouponResponse> for Coupon {
    fn from(backend: BackendCouponResponse) -> Self {
        // 解析套餐ID字符串
        let limit_plan_ids = backend.limit_plan_ids.and_then(|ids| {
            if ids.is_empty() {
                None
            } else {
                Some(
                    ids.split(',')
                        .filter_map(|id| id.trim().parse::<u32>().ok())
                        .collect(),
                )
            }
        });

        // 格式化时间戳为字符串
        let started_at = if backend.started_at > 0 {
            Some(format_timestamp(backend.started_at))
        } else {
            None
        };

        let ended_at = if backend.ended_at > 0 {
            Some(format_timestamp(backend.ended_at))
        } else {
            None
        };

        Self {
            id: backend.id as u32,
            code: backend.code,
            name: backend.name,
            type_: if backend.r#type {
                "fixed".to_string()
            } else {
                "percentage".to_string()
            },
            value: backend.value as u32,
            min_amount: None, // 后端暂未支持
            max_amount: None, // 后端暂未支持
            limit_quota: backend.limit_use.map(|x| x as u32),
            used_quota: 0, // 后端暂未返回已使用次数
            limit_use_with_user: backend.limit_use_with_user.map(|x| x as u32),
            limit_plan_ids,
            limit_period: backend.limit_period,
            started_at,
            ended_at,
            status: backend.show,
            created_at: format_timestamp(backend.created_at),
            updated_at: format_timestamp(backend.updated_at),
        }
    }
}

// 时间戳转换函数
fn format_timestamp(timestamp: i32) -> String {
    // 这里假设时间戳是秒级的
    if timestamp > 0 {
        // 简单的时间格式化，实际应用中可能需要更复杂的处理
        let js_date = js_sys::Date::new(&(timestamp as f64 * 1000.0).into());
        let date_string = js_date.to_iso_string();
        let js_string: String = date_string.as_string().unwrap_or_default();
        // 简化显示格式
        if js_string.len() > 10 {
            js_string[0..10].to_string() + " " + &js_string[11..19]
        } else {
            js_string
        }
    } else {
        "1970-01-01 00:00:00".to_string()
    }
}

pub struct CouponService;

impl CouponService {
    /// 获取优惠券列表
    pub async fn list_coupons(
        page: u32,
        page_size: u32,
        _show_only: bool,
        _enabled_only: bool,
    ) -> Result<(Vec<Coupon>, u64), String> {
        // 调用后端API
        let endpoint = format!("/api/coupons?page={}&page_size={}", page, page_size);

        match ApiClient::get::<CouponPageResponse>(&endpoint).await {
            Ok(response) => {
                if response.success {
                    if let Some(data) = response.data {
                        let coupons = data.data.into_iter().map(Coupon::from).collect();
                        Ok((coupons, data.pagination.total))
                    } else {
                        Ok((vec![], 0))
                    }
                } else {
                    let error_msg = response
                        .error
                        .map(|e| e.message)
                        .unwrap_or_else(|| "未知错误".to_string());
                    Err(error_msg)
                }
            }
            Err(e) => {
                log::error!("获取优惠券列表失败: {:?}", e);
                Err(format!("网络请求失败: {}", e))
            }
        }
    }

    /// 创建优惠券
    pub async fn create_coupon(_request: CreateCouponRequest) -> Result<Coupon, String> {
        // TODO: 实现创建优惠券
        Err("创建优惠券功能尚未实现".to_string())
    }

    /// 更新优惠券
    pub async fn update_coupon(_id: u32, _request: UpdateCouponRequest) -> Result<Coupon, String> {
        // TODO: 实现更新优惠券
        Err("更新优惠券功能尚未实现".to_string())
    }

    /// 删除优惠券
    pub async fn delete_coupon(id: u32) -> Result<(), String> {
        let endpoint = format!("/api/coupons/{}", id);

        match ApiClient::delete::<()>(&endpoint).await {
            Ok(response) => {
                if response.success {
                    Ok(())
                } else {
                    let error_msg = response
                        .error
                        .map(|e| e.message)
                        .unwrap_or_else(|| "删除失败".to_string());
                    Err(error_msg)
                }
            }
            Err(e) => {
                log::error!("删除优惠券失败: {:?}", e);
                Err(format!("网络请求失败: {}", e))
            }
        }
    }

    /// 获取优惠券统计信息
    pub async fn get_coupon_stats() -> Result<CouponStats, String> {
        // 先获取所有优惠券来计算统计
        match Self::list_coupons(1, 1000, false, false).await {
            Ok((coupons, total)) => {
                let active_coupons = coupons.iter().filter(|c| c.status).count();
                let total_used = coupons.iter().map(|c| c.used_quota).sum::<u32>();
                let total_quota = coupons.iter().filter_map(|c| c.limit_quota).sum::<u32>();

                Ok(CouponStats {
                    total_coupons: total as u32,
                    active_coupons: active_coupons as u32,
                    total_used,
                    total_quota,
                })
            }
            Err(e) => Err(e),
        }
    }
}

// 统计数据
#[derive(Debug, Clone)]
pub struct CouponStats {
    pub total_coupons: u32,
    pub active_coupons: u32,
    pub total_used: u32,
    pub total_quota: u32,
}

// 创建优惠券请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub name: String,
    pub r#type: bool,
    pub value: i32,
    pub show: bool,
    pub limit_use: Option<i32>,
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: i32,
    pub ended_at: i32,
}

// 更新优惠券请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<bool>,
    pub value: Option<i32>,
    pub show: Option<bool>,
    pub limit_use: Option<i32>,
    pub limit_use_with_user: Option<i32>,
    pub limit_plan_ids: Option<String>,
    pub limit_period: Option<String>,
    pub started_at: Option<i32>,
    pub ended_at: Option<i32>,
}
