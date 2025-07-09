use leptos::*;
use web_sys::window;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

pub fn get_stored_theme() -> Theme {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(theme)) = storage.get_item("theme") {
                return Theme::from_string(&theme);
            }
        }
    }
    Theme::Light
}

pub fn store_theme(theme: Theme) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", theme.to_string());
        }
    }
}

pub fn apply_theme(theme: Theme) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let class_list = body.class_list();

                // 移除现有的主题类
                let _ = class_list.remove_1("light-theme");
                let _ = class_list.remove_1("dark-theme");

                // 添加新的主题类
                let theme_class = match theme {
                    Theme::Light => "light-theme",
                    Theme::Dark => "dark-theme",
                };
                let _ = class_list.add_1(theme_class);
            }
        }
    }
}

pub fn use_theme() -> (ReadSignal<Theme>, WriteSignal<Theme>) {
    let (theme, set_theme) = create_signal(get_stored_theme());

    // 初始化时应用主题
    create_effect(move |_| {
        let current_theme = theme.get();
        apply_theme(current_theme);
        store_theme(current_theme);
    });

    (theme, set_theme)
}
