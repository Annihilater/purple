use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div style="padding: 2rem;">
            <div style="margin-bottom: 2rem;">
                <h1 style="font-size: 2rem; font-weight: 700; color: #1a202c; margin-bottom: 0.5rem;">"仪表盘"</h1>
                <p style="color: #718096;">"系统概览和关键指标"</p>
            </div>

            // 统计卡片
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;">
                <div class="card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"👥"</div>
                    <div>
                        <div style="font-size: 1.5rem; font-weight: 700; color: #1a202c;">"1,234"</div>
                        <div style="font-size: 0.875rem; color: #718096;">"总用户数"</div>
                    </div>
                </div>

                <div class="card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"📊"</div>
                    <div>
                        <div style="font-size: 1.5rem; font-weight: 700; color: #1a202c;">"856"</div>
                        <div style="font-size: 0.875rem; color: #718096;">"活跃用户"</div>
                    </div>
                </div>

                <div class="card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"💰"</div>
                    <div>
                        <div style="font-size: 1.5rem; font-weight: 700; color: #1a202c;">"￥12,345"</div>
                        <div style="font-size: 0.875rem; color: #718096;">"本月收入"</div>
                    </div>
                </div>

                <div class="card" style="display: flex; align-items: center;">
                    <div style="font-size: 2rem; margin-right: 1rem;">"🎫"</div>
                    <div>
                        <div style="font-size: 1.5rem; font-weight: 700; color: #1a202c;">"23"</div>
                        <div style="font-size: 0.875rem; color: #718096;">"待处理工单"</div>
                    </div>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 1.5rem;">
                // 最近活动
                <div class="card">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #1a202c;">"最近活动"</h2>
                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                        <div style="display: flex; align-items: center; padding: 0.75rem; background: #f7fafc; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"👤"</div>
                            <div>
                                <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"新用户注册"</div>
                                <div style="font-size: 0.75rem; color: #a0aec0;">"user@example.com - 2分钟前"</div>
                            </div>
                        </div>
                        <div style="display: flex; align-items: center; padding: 0.75rem; background: #f7fafc; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"💳"</div>
                            <div>
                                <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"订单支付成功"</div>
                                <div style="font-size: 0.75rem; color: #a0aec0;">"订单 #12345 - 15分钟前"</div>
                            </div>
                        </div>
                        <div style="display: flex; align-items: center; padding: 0.75rem; background: #f7fafc; border-radius: 0.5rem;">
                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"🎫"</div>
                            <div>
                                <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"新工单创建"</div>
                                <div style="font-size: 0.75rem; color: #a0aec0;">"连接问题 - 1小时前"</div>
                            </div>
                        </div>
                    </div>
                </div>

                // 系统状态
                <div class="card">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #1a202c;">"系统状态"</h2>
                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; background: #f0fff4; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"✅"</div>
                                <div>
                                    <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"API 服务"</div>
                                    <div style="font-size: 0.75rem; color: #a0aec0;">"运行正常"</div>
                                </div>
                            </div>
                            <div style="color: #38a169; font-weight: 500;">"99.9%"</div>
                        </div>
                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; background: #f0fff4; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"✅"</div>
                                <div>
                                    <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"数据库"</div>
                                    <div style="font-size: 0.75rem; color: #a0aec0;">"运行正常"</div>
                                </div>
                            </div>
                            <div style="color: #38a169; font-weight: 500;">"100%"</div>
                        </div>
                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem; background: #fffbeb; border-radius: 0.5rem;">
                            <div style="display: flex; align-items: center;">
                                <div style="font-size: 1.25rem; margin-right: 0.75rem;">"⚠️"</div>
                                <div>
                                    <div style="font-size: 0.875rem; font-weight: 500; color: #1a202c;">"邮件服务"</div>
                                    <div style="font-size: 0.75rem; color: #a0aec0;">"延迟中"</div>
                                </div>
                            </div>
                            <div style="color: #d69e2e; font-weight: 500;">"95.2%"</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
