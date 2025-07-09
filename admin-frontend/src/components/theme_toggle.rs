use crate::utils::theme::{use_theme, Theme};
use leptos::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    let toggle_theme = move |_| {
        set_theme.update(|t| *t = t.toggle());
    };

    view! {
        <button
            class="theme-toggle-btn"
            on:click=toggle_theme
            title=move || match theme.get() {
                Theme::Light => "切换到深色模式",
                Theme::Dark => "切换到浅色模式",
            }
        >
            <span class="theme-icon">
                {move || match theme.get() {
                    Theme::Light => "🌙",
                    Theme::Dark => "☀️",
                }}
            </span>
        </button>
    }
}
