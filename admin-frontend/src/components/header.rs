use crate::{components::theme_toggle::ThemeToggle, services::auth::AuthService};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let logout = move |_| {
        AuthService::remove_token();
        navigate("/", Default::default());
    };

    view! {
        <header class="header">
            <div style="max-width: 1280px; margin: 0 auto; padding: 0 1.5rem;">
                <div style="display: flex; justify-content: space-between; align-items: center; height: 4rem;">
                    <div style="display: flex; align-items: center;">
                        <A href="/admin" class="header-logo">
                            "Purple"
                        </A>
                    </div>
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        <ThemeToggle/>
                        <span class="header-text">"管理员"</span>
                        <button
                            class="btn-secondary"
                            style="padding: 0.5rem 1rem; font-size: 0.875rem;"
                            on:click=logout
                        >
                            "退出"
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}
