use super::presets::*;
use super::types::{Theme, ThemeConfig, ThemeMode};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::window;

pub struct ThemeManager {
    themes: HashMap<String, ThemeConfig>,
    current_theme_id: String,
    current_mode: ThemeMode,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();

        // 注册预设主题
        themes.insert("default".to_string(), get_default_theme());
        themes.insert("ocean".to_string(), get_ocean_theme());
        themes.insert("sunset".to_string(), get_sunset_theme());

        Self {
            themes,
            current_theme_id: "default".to_string(),
            current_mode: ThemeMode::Light,
        }
    }

    pub fn register_theme(&mut self, theme: ThemeConfig) {
        self.themes.insert(theme.id.clone(), theme);
    }

    pub fn get_theme(&self, theme_id: &str) -> Option<&ThemeConfig> {
        self.themes.get(theme_id)
    }

    pub fn get_current_theme(&self) -> Theme {
        let config = self
            .themes
            .get(&self.current_theme_id)
            .cloned()
            .unwrap_or_else(get_default_theme);

        // 根据模式选择相应的配置
        let final_config = match self.current_mode {
            ThemeMode::Dark => {
                // 根据当前主题选择对应的深色版本
                match self.current_theme_id.as_str() {
                    "ocean" => get_ocean_dark_theme(),
                    "sunset" => get_sunset_dark_theme(),
                    _ => get_default_dark_theme(),
                }
            }
            ThemeMode::Light => config,
        };

        Theme::new(self.current_mode, final_config)
    }

    pub fn set_theme(&mut self, theme_id: &str) -> bool {
        if self.themes.contains_key(theme_id) {
            self.current_theme_id = theme_id.to_string();
            self.save_to_storage();
            true
        } else {
            false
        }
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.current_mode = mode;
        self.save_to_storage();
    }

    pub fn toggle_mode(&mut self) {
        self.current_mode = self.current_mode.next();
        self.save_to_storage();
    }

    pub fn get_available_themes(&self) -> Vec<&ThemeConfig> {
        self.themes.values().collect()
    }

    pub fn get_current_mode(&self) -> ThemeMode {
        self.current_mode
    }

    pub fn get_current_theme_id(&self) -> &str {
        &self.current_theme_id
    }

    fn save_to_storage(&self) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme_id", &self.current_theme_id);
                let _ = storage.set_item("theme_mode", self.current_mode.to_string());
            }
        }
    }

    pub fn load_from_storage(&mut self) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme_id)) = storage.get_item("theme_id") {
                    if self.themes.contains_key(&theme_id) {
                        self.current_theme_id = theme_id;
                    }
                }

                if let Ok(Some(mode_str)) = storage.get_item("theme_mode") {
                    self.current_mode = ThemeMode::from_string(&mode_str);
                }
            }
        }
    }

    pub fn apply_theme(&self, theme: &Theme) {
        // 添加调试日志
        web_sys::console::log_2(
            &"Applying theme:".into(),
            &format!("{}({})", theme.config.name, theme.mode.to_string()).into(),
        );

        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let class_list = body.class_list();

                    // 移除所有主题类
                    for theme_config in self.themes.values() {
                        for mode in [ThemeMode::Light, ThemeMode::Dark] {
                            let class_name =
                                format!("theme-{}-{}", theme_config.id, mode.to_string());
                            let _ = class_list.remove_1(&class_name);
                        }
                        let base_class = format!("theme-{}", theme_config.id);
                        let _ = class_list.remove_1(&base_class);
                    }

                    // 移除老式的主题类（向后兼容）
                    let _ = class_list.remove_1("light-theme");
                    let _ = class_list.remove_1("dark-theme");

                    // 添加当前主题类
                    let base_class = theme.get_base_class();
                    let css_class = theme.get_css_class();
                    let _ = class_list.add_1(&base_class);
                    let _ = class_list.add_1(&css_class);

                    // 为了向后兼容，添加老式的主题类
                    // 检查当前主题配置是否为深色主题
                    let is_dark_theme = theme.config.colors.background.contains("2d3748")
                        || theme.config.colors.background.contains("1a202c")
                        || theme.config.colors.background.contains("164e63")
                        || theme.config.colors.background.contains("134e4a")
                        || theme.config.colors.background.contains("7c2d12")
                        || theme.config.colors.background.contains("991b1b")
                        || theme.config.colors.text_primary.starts_with("#e")
                        || theme.config.colors.text_primary.starts_with("#f");

                    let legacy_class = if is_dark_theme {
                        let _ = class_list.add_1("dark-theme");
                        "dark-theme"
                    } else {
                        let _ = class_list.add_1("light-theme");
                        "light-theme"
                    };

                    // 调试输出
                    web_sys::console::log_2(
                        &"Added CSS classes:".into(),
                        &format!("{base_class}, {css_class}, {legacy_class}").into(),
                    );

                    // 设置CSS自定义属性
                    if let Some(style) = document
                        .document_element()
                        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let style_declaration = style.style();

                        // 应用主题色彩变量
                        let colors = &theme.config.colors;
                        let _ = style_declaration.set_property("--color-primary", &colors.primary);
                        let _ =
                            style_declaration.set_property("--color-secondary", &colors.secondary);
                        let _ = style_declaration.set_property("--color-accent", &colors.accent);
                        let _ = style_declaration
                            .set_property("--color-background", &colors.background);
                        let _ = style_declaration.set_property("--color-surface", &colors.surface);
                        let _ = style_declaration.set_property("--color-card", &colors.card);
                        let _ = style_declaration
                            .set_property("--color-text-primary", &colors.text_primary);
                        let _ = style_declaration
                            .set_property("--color-text-secondary", &colors.text_secondary);
                        let _ = style_declaration
                            .set_property("--color-text-muted", &colors.text_muted);
                        let _ = style_declaration.set_property("--color-border", &colors.border);
                        let _ = style_declaration.set_property("--color-divider", &colors.divider);
                        let _ = style_declaration.set_property("--color-success", &colors.success);
                        let _ = style_declaration.set_property("--color-warning", &colors.warning);
                        let _ = style_declaration.set_property("--color-error", &colors.error);
                        let _ = style_declaration.set_property("--color-info", &colors.info);
                        let _ = style_declaration
                            .set_property("--color-success-bg", &colors.success_bg);
                        let _ = style_declaration
                            .set_property("--color-warning-bg", &colors.warning_bg);
                        let _ =
                            style_declaration.set_property("--color-error-bg", &colors.error_bg);
                        let _ = style_declaration.set_property("--color-info-bg", &colors.info_bg);
                        let _ = style_declaration.set_property("--color-hover", &colors.hover);
                        let _ = style_declaration.set_property("--color-active", &colors.active);
                        let _ =
                            style_declaration.set_property("--color-disabled", &colors.disabled);
                        let _ = style_declaration.set_property("--shadow", &colors.shadow);
                        let _ = style_declaration.set_property("--overlay", &colors.overlay);

                        // 应用渐变
                        for (key, value) in &theme.config.gradients {
                            let var_name = format!("--gradient-{key}");
                            let _ = style_declaration.set_property(&var_name, value);
                        }

                        // 应用阴影
                        for (key, value) in &theme.config.shadows {
                            let var_name = format!("--shadow-{key}");
                            let _ = style_declaration.set_property(&var_name, value);
                        }

                        // 应用自定义属性
                        for (key, value) in &theme.config.custom_properties {
                            let _ = style_declaration.set_property(key, value);
                        }

                        // 调试输出颜色变量
                        web_sys::console::log_2(
                            &"Set CSS variables:".into(),
                            &format!(
                                "--color-background: {}, --color-text-primary: {}",
                                colors.background, colors.text_primary
                            )
                            .into(),
                        );
                    }
                }
            }
        }
    }
}
