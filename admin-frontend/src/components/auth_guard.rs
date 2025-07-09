use crate::services::auth::AuthService;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AuthGuard(children: Children) -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    // 简单的同步检查
    let is_authenticated = AuthService::is_authenticated();

    if is_authenticated {
        // 已认证，显示子组件
        children().into_view()
    } else {
        // 未认证，重定向到登录页面
        navigate("/login", Default::default());
        view! {
            <div style="display: flex; justify-content: center; align-items: center; min-height: 100vh;">
                <div style="text-align: center;">
                    <div style="font-size: 1.2rem; color: #667eea;">
                        "正在跳转到登录页面..."
                    </div>
                </div>
            </div>
        }.into_view()
    }
}
