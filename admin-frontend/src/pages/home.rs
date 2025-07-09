use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero fade-in">
            <h1 class="hero-title">"管理控制台"</h1>
            <p class="hero-subtitle">"现代化的企业级管理平台，助力您的业务增长"</p>

            <div class="features-grid">
                <div class="feature-card fade-in">
                    <div class="feature-icon">"👥"</div>
                    <h3 class="feature-title">"用户管理"</h3>
                    <p class="feature-description">
                        "全面的用户生命周期管理，支持角色权限控制，让团队协作更高效"
                    </p>
                </div>

                <div class="feature-card fade-in">
                    <div class="feature-icon">"📦"</div>
                    <h3 class="feature-title">"套餐管理"</h3>
                    <p class="feature-description">
                        "灵活的产品套餐配置，支持多种计费模式，满足不同业务需求"
                    </p>
                </div>

                <div class="feature-card fade-in">
                    <div class="feature-icon">"🎫"</div>
                    <h3 class="feature-title">"优惠券系统"</h3>
                    <p class="feature-description">
                        "智能化的营销工具，支持多种优惠策略，提升用户转化率"
                    </p>
                </div>

                <div class="feature-card fade-in">
                    <div class="feature-icon">"📊"</div>
                    <h3 class="feature-title">"数据分析"</h3>
                    <p class="feature-description">
                        "实时业务数据监控，深度洞察用户行为，助力数据驱动决策"
                    </p>
                </div>

                <div class="feature-card fade-in">
                    <div class="feature-icon">"🔐"</div>
                    <h3 class="feature-title">"安全保障"</h3>
                    <p class="feature-description">
                        "企业级安全防护，多重认证机制，保障您的数据安全无忧"
                    </p>
                </div>

                <div class="feature-card fade-in">
                    <div class="feature-icon">"⚡"</div>
                    <h3 class="feature-title">"高性能"</h3>
                    <p class="feature-description">
                        "基于 Rust 技术栈，极致的性能表现，支持高并发业务场景"
                    </p>
                </div>
            </div>

            <div style="margin-top: 3rem;">
                <a href="/dashboard" class="btn">
                    "进入控制台"
                </a>
                <a href="/login" class="btn btn-secondary" style="margin-left: 1rem;">
                    "重新登录"
                </a>
            </div>
        </div>
    }
}
