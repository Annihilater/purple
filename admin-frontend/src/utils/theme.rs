pub mod css_generator;
pub mod hooks;
pub mod manager;
pub mod presets;
pub mod types;

pub use css_generator::*;
pub use hooks::{use_theme, use_theme_system};
pub use manager::ThemeManager;
pub use presets::*;
pub use types::{Theme, ThemeColors, ThemeConfig, ThemeMode};

// 为了向后兼容，保留原有的简单接口
use leptos::*;
use web_sys::window;

pub fn get_stored_theme() -> ThemeMode {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(mode)) = storage.get_item("theme_mode") {
                return ThemeMode::from_string(&mode);
            }
        }
    }
    ThemeMode::Light
}

pub fn store_theme(mode: ThemeMode) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme_mode", mode.to_string());
        }
    }
}
