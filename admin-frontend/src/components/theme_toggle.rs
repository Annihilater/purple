use crate::utils::theme::{use_theme, ThemeMode};
use leptos::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (get_theme_mode, toggle_mode) = use_theme();
    let get_theme_mode_for_title = get_theme_mode.clone();

    let toggle_theme = move |_| {
        toggle_mode();
    };

    view! {
        <button
            class="theme-toggle-btn"
            on:click=toggle_theme
            title=move || match get_theme_mode_for_title() {
                ThemeMode::Light => "切换到深色模式",
                ThemeMode::Dark => "切换到浅色模式",
            }
        >
            <span class="theme-icon">
                {move || match get_theme_mode() {
                    ThemeMode::Light => "🌙",
                    ThemeMode::Dark => "☀️",
                }}
            </span>
        </button>
    }
}
