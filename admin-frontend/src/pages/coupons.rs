use crate::components::common::*;
use crate::services::coupon_service::{Coupon, CouponService, CouponStats};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[component]
pub fn CouponsPage() -> impl IntoView {
    let coupons = create_rw_signal(Vec::<Coupon>::new());
    let loading = create_rw_signal(false);
    let error = create_rw_signal(None::<String>);
    let search_term = create_rw_signal(String::new());
    let filter_status = create_rw_signal(String::from("all")); // all, active, inactive
    let filter_type = create_rw_signal(String::from("all")); // all, fixed, percentage
    let show_add_modal = create_rw_signal(false);
    let show_edit_modal = create_rw_signal(false);
    let selected_coupon = create_rw_signal(None::<Coupon>);
    let stats = create_rw_signal(CouponStats {
        total_coupons: 0,
        active_coupons: 0,
        total_used: 0,
        total_quota: 0,
    });

    // 加载优惠券数据
    let load_coupons = create_action(move |_: &()| async move {
        loading.set(true);
        error.set(None);

        match CouponService::list_coupons(1, 1000, false, false).await {
            Ok((coupon_list, _total)) => {
                coupons.set(coupon_list);
                // 加载统计数据
                if let Ok(stats_data) = CouponService::get_coupon_stats().await {
                    stats.set(stats_data);
                }
            }
            Err(e) => {
                error.set(Some(e));
            }
        }

        loading.set(false);
    });

    // 页面加载时获取数据
    create_effect(move |_| {
        load_coupons.dispatch(());
    });

    // 筛选后的优惠券
    let filtered_coupons = create_rw_signal(Vec::<Coupon>::new());

    // 更新筛选结果
    create_effect(move |_| {
        let search = search_term.get().to_lowercase();
        let status_filter = filter_status.get();
        let type_filter = filter_type.get();

        let filtered = coupons
            .get()
            .into_iter()
            .filter(|coupon| {
                // 搜索过滤
                let search_match = search.is_empty()
                    || coupon.code.to_lowercase().contains(&search)
                    || coupon.name.to_lowercase().contains(&search);

                // 状态过滤
                let status_match = match status_filter.as_str() {
                    "active" => coupon.status,
                    "inactive" => !coupon.status,
                    _ => true,
                };

                // 类型过滤
                let type_match = match type_filter.as_str() {
                    "fixed" => coupon.type_ == "fixed",
                    "percentage" => coupon.type_ == "percentage",
                    _ => true,
                };

                search_match && status_match && type_match
            })
            .collect::<Vec<_>>();

        filtered_coupons.set(filtered);
    });

    // 统计数据
    let stats_memo = create_memo(move |_| {
        let stats_data = stats.get();
        (
            stats_data.total_coupons,
            stats_data.active_coupons,
            stats_data.total_used,
            stats_data.total_quota,
        )
    });

    let render_coupon_row = Box::new(|coupon: &Coupon| {
        let coupon = coupon.clone(); // 克隆以避免生命周期问题

        let type_display = match coupon.type_.as_str() {
            "fixed" => format!("￥{:.2}", coupon.value as f64 / 100.0),
            "percentage" => format!("{}%", coupon.value),
            _ => "未知".to_string(),
        };

        let usage_display = if let Some(limit) = coupon.limit_quota {
            format!("{}/{}", coupon.used_quota, limit)
        } else {
            format!("{}/∞", coupon.used_quota)
        };

        let usage_percentage = if let Some(limit) = coupon.limit_quota {
            if limit > 0 {
                (coupon.used_quota as f64 / limit as f64 * 100.0).min(100.0)
            } else {
                0.0
            }
        } else {
            0.0
        };

        let usage_color = if usage_percentage < 50.0 {
            "success"
        } else if usage_percentage < 80.0 {
            "warning"
        } else {
            "error"
        };

        let min_amount_display = if let Some(min) = coupon.min_amount {
            format!("￥{:.2}", min as f64 / 100.0)
        } else {
            "无限制".to_string()
        };

        let max_amount_display = if let Some(max) = coupon.max_amount {
            format!("￥{:.2}", max as f64 / 100.0)
        } else {
            "无限制".to_string()
        };

        let period_display = match coupon.limit_period.as_ref().map(|s| s.as_str()) {
            Some("monthly") => "月付",
            Some("quarterly") => "季付",
            Some("half_year") => "半年付",
            Some("year") => "年付",
            _ => "无限制",
        };

        let status_variant = if coupon.status { "success" } else { "error" };
        let status_text = if coupon.status { "启用" } else { "禁用" };

        // 判断是否过期
        let is_expired = coupon.ended_at.as_ref().map_or(false, |end_time| {
            // 简单的时间比较，实际应该使用真实的时间比较
            end_time.as_str() < "2024-01-16 00:00:00"
        });

        view! {
            <td>
                <div class="coupon-info">
                    <div class="coupon-code">{coupon.code}</div>
                    <div class="coupon-name">{coupon.name}</div>
                    <div class="coupon-id">#{coupon.id}</div>
                </div>
            </td>
            <td>
                <div class="coupon-type">
                    <StatusBadge
                        status=match coupon.type_.as_str() {
                            "fixed" => "固定金额",
                            "percentage" => "百分比",
                            _ => "未知"
                        }.to_string()
                        variant=match coupon.type_.as_str() {
                            "fixed" => "info",
                            "percentage" => "warning",
                            _ => "error"
                        }.to_string()
                    />
                    <div class="coupon-value">{type_display}</div>
                </div>
            </td>
            <td>
                <div class="usage-info">
                    <div class="usage-stats">
                        <div class="usage-text">{usage_display}</div>
                        {if coupon.limit_quota.is_some() {
                            view! {
                                <div class="usage-bar">
                                    <div
                                        class=format!("usage-fill usage-{}", usage_color)
                                        style=format!("width: {}%", usage_percentage as i32)
                                    ></div>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    </div>
                </div>
            </td>
            <td>
                <div class="amount-limits">
                    <div class="min-amount">
                        <span class="amount-label">"最小: "</span>
                        <span class="amount-value">{min_amount_display}</span>
                    </div>
                    {if coupon.type_ == "percentage" && coupon.max_amount.is_some() {
                        view! {
                            <div class="max-amount">
                                <span class="amount-label">"最大: "</span>
                                <span class="amount-value">{max_amount_display}</span>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </td>
            <td>
                <div class="period-info">
                    <span class="period-text">{period_display}</span>
                    {if let Some(user_limit) = coupon.limit_use_with_user {
                        view! {
                            <div class="user-limit">
                                <span class="limit-label">"每用户限制: "</span>
                                <span class="limit-value">{user_limit}</span>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </td>
            <td>
                <div class="time-info">
                    {if let Some(start_time) = coupon.started_at {
                        let start_date = start_time.split(' ').next().unwrap_or("").to_string();
                        view! {
                            <div class="start-time">
                                <span class="time-label">"开始: "</span>
                                <span class="time-value">{start_date}</span>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                    {if let Some(end_time) = coupon.ended_at {
                        let end_date = end_time.split(' ').next().unwrap_or("").to_string();
                        view! {
                            <div class="end-time">
                                <span class="time-label">"结束: "</span>
                                <span class=format!("time-value {}", if is_expired { "expired" } else { "" })>
                                    {end_date}
                                </span>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="end-time">
                                <span class="time-label">"结束: "</span>
                                <span class="time-value">"永久"</span>
                            </div>
                        }.into_view()
                    }}
                </div>
            </td>
            <td>
                <div class="status-info">
                    <StatusBadge
                        status=status_text.to_string()
                        variant=status_variant.to_string()
                    />
                    {if is_expired {
                        view! {
                            <div class="expired-badge">
                                <StatusBadge
                                    status="已过期".to_string()
                                    variant="error".to_string()
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </td>
        }.into_view()
    });

    let on_add = Some(Rc::new(move || {
        show_add_modal.set(true);
    }) as Rc<dyn Fn()>);

    let on_edit = Some(Rc::new(move |index: usize| {
        if let Some(coupon) = filtered_coupons.get_untracked().get(index) {
            selected_coupon.set(Some(coupon.clone()));
            show_edit_modal.set(true);
        }
    }) as Rc<dyn Fn(usize)>);

    let on_delete = Some(Rc::new(move |index: usize| {
        if let Some(coupon) = filtered_coupons.get_untracked().get(index) {
            let coupon_id = coupon.id;
            let load_coupons = load_coupons.clone();

            // 在实际应用中，这里应该显示确认对话框
            spawn_local(async move {
                match CouponService::delete_coupon(coupon_id).await {
                    Ok(_) => {
                        // 重新加载数据
                        load_coupons.dispatch(());
                    }
                    Err(e) => {
                        log::error!("删除优惠券失败: {}", e);
                        // 在实际应用中，这里应该显示错误消息
                    }
                }
            });
        }
    }) as Rc<dyn Fn(usize)>);

    view! {
        <PageTemplate title="优惠券管理".to_string() subtitle="管理系统优惠券和促销活动".to_string()>
            // 加载状态
            {move || {
                if loading.get() {
                    view! {
                        <div class="loading-container">
                            <div class="loading-spinner">
                                <span>"加载中..."</span>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

            // 错误状态
            {move || {
                if let Some(error_msg) = error.get() {
                    view! {
                        <div class="error-container">
                            <div class="error-message">
                                <span>"❌ 加载失败: "{error_msg}</span>
                                <button
                                    class="btn btn-primary"
                                    on:click=move |_| load_coupons.dispatch(())
                                >
                                    "重试"
                                </button>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

            // 统计卡片
            <div class="stats-grid">
                <StatsCard
                    title="总优惠券数".to_string()
                    value=Signal::derive(move || stats_memo.get().0.to_string())
                    icon="🎫".to_string()
                    color="blue".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="启用中".to_string()
                    value=Signal::derive(move || stats_memo.get().1.to_string())
                    icon="✅".to_string()
                    color="green".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总使用次数".to_string()
                    value=Signal::derive(move || stats_memo.get().2.to_string())
                    icon="📊".to_string()
                    color="purple".to_string()
                    change=None
                    change_type=None
                />
                <StatsCard
                    title="总配额".to_string()
                    value=Signal::derive(move || {
                        let total_quota = stats_memo.get().3;
                        if total_quota > 0 {
                            total_quota.to_string()
                        } else {
                            "无限制".to_string()
                        }
                    })
                    icon="🎯".to_string()
                    color="orange".to_string()
                    change=None
                    change_type=None
                />
            </div>

            // 搜索和筛选
            <div class="content-card">
                <div class="content-card-header">
                    <div class="search-filters">
                        <div class="search-input-group">
                            <input
                                type="text"
                                class="search-input"
                                placeholder="搜索优惠券代码或名称..."
                                prop:value=move || search_term.get()
                                on:input=move |ev| {
                                    search_term.set(event_target_value(&ev));
                                }
                            />
                            <button class="search-button">
                                <span class="search-icon">"🔍"</span>
                            </button>
                        </div>
                        <div class="filter-group">
                            <select
                                class="filter-select"
                                on:change=move |ev| {
                                    filter_status.set(event_target_value(&ev));
                                }
                            >
                                <option value="all" selected=move || filter_status.get() == "all">"全部状态"</option>
                                <option value="active" selected=move || filter_status.get() == "active">"启用"</option>
                                <option value="inactive" selected=move || filter_status.get() == "inactive">"禁用"</option>
                            </select>
                            <select
                                class="filter-select"
                                on:change=move |ev| {
                                    filter_type.set(event_target_value(&ev));
                                }
                            >
                                <option value="all" selected=move || filter_type.get() == "all">"全部类型"</option>
                                <option value="fixed" selected=move || filter_type.get() == "fixed">"固定金额"</option>
                                <option value="percentage" selected=move || filter_type.get() == "percentage">"百分比折扣"</option>
                            </select>
                        </div>
                    </div>
                    <div class="header-actions">
                        <button class="btn btn-secondary" on:click=move |_| {
                            search_term.set(String::new());
                            filter_status.set("all".to_string());
                            filter_type.set("all".to_string());
                        }>
                            <span class="btn-icon">"🔄"</span>
                            "重置"
                        </button>
                        <button class="btn btn-primary" on:click=move |_| show_add_modal.set(true)>
                            <span class="btn-icon">"+"</span>
                            "新增优惠券"
                        </button>
                    </div>
                </div>

                // 优惠券列表
                <DataTable
                    headers=vec![
                        "优惠券信息".to_string(),
                        "类型和面值".to_string(),
                        "使用情况".to_string(),
                        "金额限制".to_string(),
                        "使用限制".to_string(),
                        "有效期".to_string(),
                        "状态".to_string(),
                    ]
                    data=filtered_coupons.read_only()
                    render_row=render_coupon_row
                    on_add=on_add
                    on_edit=on_edit
                    on_delete=on_delete
                />
            </div>

            // 新增优惠券模态框
            {move || {
                if show_add_modal.get() {
                    view! {
                        <div class="modal-overlay" on:click=move |_| show_add_modal.set(false)>
                            <div class="modal-content" on:click=move |ev| ev.stop_propagation()>
                                <div class="modal-header">
                                    <h3>"新增优惠券"</h3>
                                    <button class="modal-close" on:click=move |_| show_add_modal.set(false)>
                                        "×"
                                    </button>
                                </div>
                                <div class="modal-body">
                                    <div class="form-notice">
                                        <p>"新增优惠券功能开发中..."</p>
                                    </div>
                                </div>
                                <div class="modal-footer">
                                    <button class="btn btn-secondary" on:click=move |_| show_add_modal.set(false)>
                                        "取消"
                                    </button>
                                    <button class="btn btn-primary">
                                        "确定"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

            // 编辑优惠券模态框
            {move || {
                if show_edit_modal.get() {
                    view! {
                        <div class="modal-overlay" on:click=move |_| show_edit_modal.set(false)>
                            <div class="modal-content" on:click=move |ev| ev.stop_propagation()>
                                <div class="modal-header">
                                    <h3>"编辑优惠券"</h3>
                                    <button class="modal-close" on:click=move |_| show_edit_modal.set(false)>
                                        "×"
                                    </button>
                                </div>
                                <div class="modal-body">
                                    <div class="form-notice">
                                        <p>"编辑优惠券功能开发中..."</p>
                                        {if let Some(coupon) = selected_coupon.get() {
                                            view! {
                                                <p>"当前选择: "{coupon.name}</p>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </div>
                                </div>
                                <div class="modal-footer">
                                    <button class="btn btn-secondary" on:click=move |_| show_edit_modal.set(false)>
                                        "取消"
                                    </button>
                                    <button class="btn btn-primary">
                                        "保存"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </PageTemplate>
    }
}
