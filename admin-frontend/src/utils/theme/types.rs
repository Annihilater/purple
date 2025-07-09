use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeColors {
    // 主要颜色
    pub primary: String,
    pub secondary: String,
    pub accent: String,

    // 背景颜色
    pub background: String,
    pub surface: String,
    pub card: String,

    // 文字颜色
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,

    // 边框和分割线
    pub border: String,
    pub divider: String,

    // 状态颜色
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,

    // 状态背景色
    pub success_bg: String,
    pub warning_bg: String,
    pub error_bg: String,
    pub info_bg: String,

    // 交互状态
    pub hover: String,
    pub active: String,
    pub disabled: String,

    // 阴影和特效
    pub shadow: String,
    pub overlay: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub colors: ThemeColors,
    pub gradients: HashMap<String, String>,
    pub shadows: HashMap<String, String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn to_string(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::Light,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub config: ThemeConfig,
}

impl Theme {
    pub fn new(mode: ThemeMode, config: ThemeConfig) -> Self {
        Self { mode, config }
    }

    pub fn get_css_class(&self) -> String {
        format!("theme-{}-{}", self.config.id, self.mode.to_string())
    }

    pub fn get_base_class(&self) -> String {
        format!("theme-{}", self.config.id)
    }
}
