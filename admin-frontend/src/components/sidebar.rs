use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside style="width: 16rem; background: white; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1); height: 100vh; overflow-y: auto;">
            <nav style="margin-top: 1.25rem; padding: 0 0.5rem;">
                <ul style="display: flex; flex-direction: column; gap: 1.5rem;">
                    // 设置分组
                    <li>
                        <div style="font-size: 0.75rem; font-weight: 600; color: #a0aec0; text-transform: uppercase; letter-spacing: 0.05em; padding: 0.75rem 0.5rem;">
                            "设置"
                        </div>
                        <ul style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <li>
                                <A href="/settings/system" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"⚙️"</span>
                                    "系统设置"
                                </A>
                            </li>
                            <li>
                                <A href="/settings/payment" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"💳"</span>
                                    "支付设置"
                                </A>
                            </li>
                            <li>
                                <A href="/settings/theme" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🎨"</span>
                                    "主题配置"
                                </A>
                            </li>
                        </ul>
                    </li>

                    // 服务器分组
                    <li>
                        <div style="font-size: 0.75rem; font-weight: 600; color: #a0aec0; text-transform: uppercase; letter-spacing: 0.05em; padding: 0.75rem 0.5rem;">
                            "服务器"
                        </div>
                        <ul style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <li>
                                <A href="/server/nodes" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🖥️"</span>
                                    "节点管理"
                                </A>
                            </li>
                            <li>
                                <A href="/server/permissions" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🔐"</span>
                                    "权限组管理"
                                </A>
                            </li>
                            <li>
                                <A href="/server/routes" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🛣️"</span>
                                    "路由管理"
                                </A>
                            </li>
                        </ul>
                    </li>

                    // 财务分组
                    <li>
                        <div style="font-size: 0.75rem; font-weight: 600; color: #a0aec0; text-transform: uppercase; letter-spacing: 0.05em; padding: 0.75rem 0.5rem;">
                            "财务"
                        </div>
                        <ul style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <li>
                                <A href="/finance/subscriptions" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"📋"</span>
                                    "订阅管理"
                                </A>
                            </li>
                            <li>
                                <A href="/finance/orders" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🧾"</span>
                                    "订单管理"
                                </A>
                            </li>
                            <li>
                                <A href="/finance/coupons" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🎫"</span>
                                    "优惠券管理"
                                </A>
                            </li>
                        </ul>
                    </li>

                    // 用户分组
                    <li>
                        <div style="font-size: 0.75rem; font-weight: 600; color: #a0aec0; text-transform: uppercase; letter-spacing: 0.05em; padding: 0.75rem 0.5rem;">
                            "用户"
                        </div>
                        <ul style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <li>
                                <A href="/users/management" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"👥"</span>
                                    "用户管理"
                                </A>
                            </li>
                            <li>
                                <A href="/users/announcements" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"📢"</span>
                                    "公告管理"
                                </A>
                            </li>
                            <li>
                                <A href="/users/tickets" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"🎫"</span>
                                    "工单管理"
                                </A>
                            </li>
                            <li>
                                <A href="/users/knowledge" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"📚"</span>
                                    "知识库管理"
                                </A>
                            </li>
                        </ul>
                    </li>

                    // 指标分组
                    <li>
                        <div style="font-size: 0.75rem; font-weight: 600; color: #a0aec0; text-transform: uppercase; letter-spacing: 0.05em; padding: 0.75rem 0.5rem;">
                            "指标"
                        </div>
                        <ul style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <li>
                                <A href="/metrics/queues" class="sidebar-link">
                                    <span style="margin-right: 0.75rem;">"📊"</span>
                                    "队列管理"
                                </A>
                            </li>
                        </ul>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
