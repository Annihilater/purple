use crate::utils::theme::{use_theme_system, ThemeMode};
use leptos::*;

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let (current_theme, set_theme, set_mode, _toggle_mode, _available_themes) = use_theme_system();

    let theme_options = vec![
        ("default", "默认主题"),
        ("ocean", "海洋主题"),
        ("sunset", "日落主题"),
    ];

    let mode_options = vec![(ThemeMode::Light, "浅色"), (ThemeMode::Dark, "深色")];

    view! {
        <div class="theme-selector">
            <div class="theme-selector-section">
                <h3 class="theme-selector-title">"主题选择"</h3>
                <div class="theme-options">
                    {theme_options.into_iter().map(|(id, name)| {
                        let set_theme = set_theme.clone();
                        let theme_id = id.to_string();
                        let current_id = theme_id.clone();
                        let is_current = move || current_theme.get().config.id == current_id;

                        view! {
                            <button
                                class:theme-option=true
                                class:active=is_current
                                on:click=move |_| set_theme(&theme_id)
                            >
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div class="theme-selector-section">
                <h3 class="theme-selector-title">"模式选择"</h3>
                <div class="mode-options">
                    {mode_options.into_iter().map(|(mode, name)| {
                        let set_mode = set_mode.clone();
                        let is_current = move || current_theme.get().mode == mode;

                        view! {
                            <button
                                class:mode-option=true
                                class:active=is_current
                                on:click=move |_| set_mode(mode)
                            >
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
