use crate::components::{auth_guard::AuthGuard, header::Header, sidebar::Sidebar};
use crate::utils::theme::use_theme;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    let (theme, _) = use_theme();

    // 当主题变化时，更新body的主题类
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let class_list = body.class_list();

                    // 移除现有的主题类
                    let _ = class_list.remove_1("light-theme");
                    let _ = class_list.remove_1("dark-theme");

                    // 添加新的主题类
                    let theme_class = format!("{}-theme", theme.get().to_string());
                    let _ = class_list.add_1(&theme_class);
                }
            }
        }
    });

    view! {
        <AuthGuard>
            <div style="min-height: 100vh;">
                <Header/>
                <div style="display: flex;">
                    <Sidebar/>
                    <main class="main-content" style="flex: 1; padding: 1.5rem;">
                        <Outlet/>
                    </main>
                </div>
            </div>
        </AuthGuard>
    }
}
