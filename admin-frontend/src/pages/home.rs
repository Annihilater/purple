use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero fade-in">
            <div style="text-align: center; padding: 4rem 2rem;">
                <h1 class="hero-title">"Purple 管理系统"</h1>
                <p class="hero-subtitle">"现代化的企业级管理平台，助力您的业务增长"</p>
                
                // 登录注册按钮
                <div style="margin: 3rem 0;">
                    <a href="/login" class="btn" style="margin-right: 1rem;">
                        "管理员登录"
                    </a>
                    <a href="/login" class="btn btn-secondary">
                        "注册账户"
                    </a>
                </div>
            </div>

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
            
            // 底部行动区域
            <div style="text-align: center; margin-top: 4rem; padding: 2rem; background: rgba(255, 255, 255, 0.1); border-radius: 16px;">
                <h2 style="font-size: 1.5rem; font-weight: 600; color: white; margin-bottom: 1rem;">
                    "立即开始管理您的业务"
                </h2>
                <p style="color: rgba(255, 255, 255, 0.8); margin-bottom: 2rem;">
                    "加入数千家企业的选择，体验现代化管理系统带来的效率提升"
                </p>
                <a href="/login" class="btn" style="font-size: 1.1rem; padding: 1rem 2rem;">
                    "立即开始 →"
                </a>
            </div>
        </div>
    }
}
