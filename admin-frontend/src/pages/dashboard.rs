use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div style="padding: 2rem;">
            <div style="margin-bottom: 2rem;">
                <h1 class="page-title" style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;">"仪表盘"</h1>
                <p class="page-subtitle" style="color: #718096;">"系统概览和关键指标"</p>
            </div>

            // 统计卡片
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;">
                <div class="card stat-card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"👥"</div>
                    <div>
                        <div class="stat-number" style="font-size: 1.5rem; font-weight: 700;">"1,234"</div>
                        <div class="stat-label" style="font-size: 0.875rem;">"总用户数"</div>
                    </div>
                </div>

                <div class="card stat-card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"📊"</div>
                    <div>
                        <div class="stat-number" style="font-size: 1.5rem; font-weight: 700;">"856"</div>
                        <div class="stat-label" style="font-size: 0.875rem;">"活跃用户"</div>
                    </div>
                </div>

                <div class="card stat-card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"💰"</div>
                    <div>
                        <div class="stat-number" style="font-size: 1.5rem; font-weight: 700;">"￥12,345"</div>
                        <div class="stat-label" style="font-size: 0.875rem;">"本月收入"</div>
                    </div>
                </div>

                <div class="card stat-card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"🎫"</div>
                    <div>
                        <div class="stat-number" style="font-size: 1.5rem; font-weight: 700;">"23"</div>
                        <div class="stat-label" style="font-size: 0.875rem;">"待处理工单"</div>
                    </div>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 1.5rem;">
                // 最近活动
                <div class="card">
                    <h2 class="card-title" style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;">"最近活动"</h2>
                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                        <div class="activity-item" style="display: flex; align-items: center; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"👤"</div>
                            <div>
                                <div class="activity-title" style="font-size: 0.875rem; font-weight: 500;">"新用户注册"</div>
                                <div class="activity-subtitle" style="font-size: 0.75rem;">"user@example.com - 2分钟前"</div>
                            </div>
                        </div>
                        <div class="activity-item" style="display: flex; align-items: center; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"💳"</div>
                            <div>
                                <div class="activity-title" style="font-size: 0.875rem; font-weight: 500;">"订单支付成功"</div>
                                <div class="activity-subtitle" style="font-size: 0.75rem;">"订单 #12345 - 15分钟前"</div>
                            </div>
                        </div>
                        <div class="activity-item" style="display: flex; align-items: center; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"🎫"</div>
                            <div>
                                <div class="activity-title" style="font-size: 0.875rem; font-weight: 500;">"新工单创建"</div>
                                <div class="activity-subtitle" style="font-size: 0.75rem;">"连接问题 - 1小时前"</div>
                            </div>
                        </div>
                    </div>
                </div>

                // 系统状态
                <div class="card">
                    <h2 class="card-title" style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;">"系统状态"</h2>
                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                        <div class="status-item status-success" style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"✅"</div>
                                <div>
                                    <div class="status-title" style="font-size: 0.875rem; font-weight: 500;">"API 服务"</div>
                                    <div class="status-subtitle" style="font-size: 0.75rem;">"运行正常"</div>
                                </div>
                            </div>
                            <div class="status-value status-value-success" style="font-weight: 500;">"99.9%"</div>
                        </div>
                        <div class="status-item status-success" style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"✅"</div>
                                <div>
                                    <div class="status-title" style="font-size: 0.875rem; font-weight: 500;">"数据库"</div>
                                    <div class="status-subtitle" style="font-size: 0.75rem;">"运行正常"</div>
                                </div>
                            </div>
                            <div class="status-value status-value-success" style="font-weight: 500;">"100%"</div>
                        </div>
                        <div class="status-item status-warning" style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"⚠️"</div>
                                <div>
                                    <div class="status-title" style="font-size: 0.875rem; font-weight: 500;">"邮件服务"</div>
                                    <div class="status-subtitle" style="font-size: 0.75rem;">"延迟中"</div>
                                </div>
                            </div>
                            <div class="status-value status-value-warning" style="font-weight: 500;">"95.2%"</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
