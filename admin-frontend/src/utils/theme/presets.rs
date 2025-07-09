use super::types::{ThemeColors, ThemeConfig};
use std::collections::HashMap;

pub fn get_default_theme() -> ThemeConfig {
    ThemeConfig {
        id: "default".to_string(),
        name: "默认主题".to_string(),
        description: "Purple 默认蓝紫色主题".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#667eea".to_string(),
            secondary: "#764ba2".to_string(),
            accent: "#f093fb".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
            surface: "#ffffff".to_string(),
            card: "rgba(255, 255, 255, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#2d3748".to_string(),
            text_secondary: "#4a5568".to_string(),
            text_muted: "#718096".to_string(),

            // 边框和分割线
            border: "#e2e8f0".to_string(),
            divider: "#cbd5e0".to_string(),

            // 状态颜色
            success: "#38a169".to_string(),
            warning: "#d69e2e".to_string(),
            error: "#e53e3e".to_string(),
            info: "#3182ce".to_string(),

            // 状态背景色
            success_bg: "#f0fff4".to_string(),
            warning_bg: "#fffbeb".to_string(),
            error_bg: "#fed7d7".to_string(),
            info_bg: "#ebf8ff".to_string(),

            // 交互状态
            hover: "#f7fafc".to_string(),
            active: "#edf2f7".to_string(),
            disabled: "#a0aec0".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            overlay: "rgba(0, 0, 0, 0.5)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.1)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(102, 126, 234, 0.4)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(102, 126, 234, 0.6)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}

pub fn get_default_dark_theme() -> ThemeConfig {
    ThemeConfig {
        id: "default".to_string(),
        name: "默认主题（深色）".to_string(),
        description: "Purple 默认主题的深色版本".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#90cdf4".to_string(),
            secondary: "#a78bfa".to_string(),
            accent: "#fbb6ce".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #2d3748 0%, #1a202c 100%)".to_string(),
            surface: "#2d3748".to_string(),
            card: "rgba(45, 55, 72, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#e2e8f0".to_string(),
            text_secondary: "#cbd5e0".to_string(),
            text_muted: "#a0aec0".to_string(),

            // 边框和分割线
            border: "#4a5568".to_string(),
            divider: "#718096".to_string(),

            // 状态颜色
            success: "#68d391".to_string(),
            warning: "#f6e05e".to_string(),
            error: "#fc8181".to_string(),
            info: "#63b3ed".to_string(),

            // 状态背景色
            success_bg: "rgba(56, 161, 105, 0.2)".to_string(),
            warning_bg: "rgba(214, 158, 46, 0.2)".to_string(),
            error_bg: "rgba(229, 62, 62, 0.2)".to_string(),
            info_bg: "rgba(49, 130, 206, 0.2)".to_string(),

            // 交互状态
            hover: "rgba(74, 85, 104, 0.3)".to_string(),
            active: "rgba(74, 85, 104, 0.5)".to_string(),
            disabled: "#718096".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.3)".to_string(),
            overlay: "rgba(0, 0, 0, 0.7)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.3)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(102, 126, 234, 0.3)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(102, 126, 234, 0.4)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}

pub fn get_ocean_theme() -> ThemeConfig {
    ThemeConfig {
        id: "ocean".to_string(),
        name: "海洋主题".to_string(),
        description: "清新的蓝绿色海洋主题".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#0891b2".to_string(),
            secondary: "#0d9488".to_string(),
            accent: "#06b6d4".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #0891b2 0%, #0d9488 100%)".to_string(),
            surface: "#ffffff".to_string(),
            card: "rgba(255, 255, 255, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#0f172a".to_string(),
            text_secondary: "#334155".to_string(),
            text_muted: "#64748b".to_string(),

            // 边框和分割线
            border: "#e2e8f0".to_string(),
            divider: "#cbd5e0".to_string(),

            // 状态颜色
            success: "#059669".to_string(),
            warning: "#d97706".to_string(),
            error: "#dc2626".to_string(),
            info: "#0284c7".to_string(),

            // 状态背景色
            success_bg: "#ecfdf5".to_string(),
            warning_bg: "#fffbeb".to_string(),
            error_bg: "#fef2f2".to_string(),
            info_bg: "#eff6ff".to_string(),

            // 交互状态
            hover: "#f0f9ff".to_string(),
            active: "#e0f2fe".to_string(),
            disabled: "#94a3b8".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            overlay: "rgba(0, 0, 0, 0.5)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #0891b2 0%, #0d9488 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #06b6d4 0%, #0891b2 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.1)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(8, 145, 178, 0.4)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(8, 145, 178, 0.6)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}

pub fn get_sunset_theme() -> ThemeConfig {
    ThemeConfig {
        id: "sunset".to_string(),
        name: "日落主题".to_string(),
        description: "温暖的橙红色日落主题".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#ea580c".to_string(),
            secondary: "#dc2626".to_string(),
            accent: "#f59e0b".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #ea580c 0%, #dc2626 100%)".to_string(),
            surface: "#ffffff".to_string(),
            card: "rgba(255, 255, 255, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#1c1917".to_string(),
            text_secondary: "#44403c".to_string(),
            text_muted: "#78716c".to_string(),

            // 边框和分割线
            border: "#e7e5e4".to_string(),
            divider: "#d6d3d1".to_string(),

            // 状态颜色
            success: "#16a34a".to_string(),
            warning: "#ca8a04".to_string(),
            error: "#dc2626".to_string(),
            info: "#2563eb".to_string(),

            // 状态背景色
            success_bg: "#f0fdf4".to_string(),
            warning_bg: "#fefce8".to_string(),
            error_bg: "#fef2f2".to_string(),
            info_bg: "#eff6ff".to_string(),

            // 交互状态
            hover: "#fef7ed".to_string(),
            active: "#fed7aa".to_string(),
            disabled: "#a8a29e".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.1)".to_string(),
            overlay: "rgba(0, 0, 0, 0.5)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #ea580c 0%, #dc2626 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #f59e0b 0%, #ea580c 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.1)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(234, 88, 12, 0.4)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(234, 88, 12, 0.6)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}

pub fn get_ocean_dark_theme() -> ThemeConfig {
    ThemeConfig {
        id: "ocean".to_string(),
        name: "海洋主题（深色）".to_string(),
        description: "海洋主题的深色版本".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#22d3ee".to_string(),
            secondary: "#2dd4bf".to_string(),
            accent: "#38bdf8".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #164e63 0%, #134e4a 100%)".to_string(),
            surface: "#1e293b".to_string(),
            card: "rgba(30, 41, 59, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#f1f5f9".to_string(),
            text_secondary: "#cbd5e1".to_string(),
            text_muted: "#94a3b8".to_string(),

            // 边框和分割线
            border: "#334155".to_string(),
            divider: "#475569".to_string(),

            // 状态颜色
            success: "#34d399".to_string(),
            warning: "#fbbf24".to_string(),
            error: "#f87171".to_string(),
            info: "#60a5fa".to_string(),

            // 状态背景色
            success_bg: "rgba(52, 211, 153, 0.2)".to_string(),
            warning_bg: "rgba(251, 191, 36, 0.2)".to_string(),
            error_bg: "rgba(248, 113, 113, 0.2)".to_string(),
            info_bg: "rgba(96, 165, 250, 0.2)".to_string(),

            // 交互状态
            hover: "rgba(51, 65, 85, 0.5)".to_string(),
            active: "rgba(51, 65, 85, 0.7)".to_string(),
            disabled: "#64748b".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.4)".to_string(),
            overlay: "rgba(0, 0, 0, 0.7)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #0891b2 0%, #0d9488 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #06b6d4 0%, #0891b2 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.4)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(34, 211, 238, 0.3)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(34, 211, 238, 0.4)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}

pub fn get_sunset_dark_theme() -> ThemeConfig {
    ThemeConfig {
        id: "sunset".to_string(),
        name: "日落主题（深色）".to_string(),
        description: "日落主题的深色版本".to_string(),
        colors: ThemeColors {
            // 主要颜色
            primary: "#fb923c".to_string(),
            secondary: "#f87171".to_string(),
            accent: "#fbbf24".to_string(),

            // 背景颜色
            background: "linear-gradient(135deg, #7c2d12 0%, #991b1b 100%)".to_string(),
            surface: "#292524".to_string(),
            card: "rgba(41, 37, 36, 0.95)".to_string(),

            // 文字颜色
            text_primary: "#fafaf9".to_string(),
            text_secondary: "#e7e5e4".to_string(),
            text_muted: "#d6d3d1".to_string(),

            // 边框和分割线
            border: "#44403c".to_string(),
            divider: "#57534e".to_string(),

            // 状态颜色
            success: "#4ade80".to_string(),
            warning: "#facc15".to_string(),
            error: "#f87171".to_string(),
            info: "#60a5fa".to_string(),

            // 状态背景色
            success_bg: "rgba(74, 222, 128, 0.2)".to_string(),
            warning_bg: "rgba(250, 204, 21, 0.2)".to_string(),
            error_bg: "rgba(248, 113, 113, 0.2)".to_string(),
            info_bg: "rgba(96, 165, 250, 0.2)".to_string(),

            // 交互状态
            hover: "rgba(68, 64, 60, 0.5)".to_string(),
            active: "rgba(68, 64, 60, 0.7)".to_string(),
            disabled: "#78716c".to_string(),

            // 阴影和特效
            shadow: "0 1px 3px rgba(0, 0, 0, 0.4)".to_string(),
            overlay: "rgba(0, 0, 0, 0.7)".to_string(),
        },
        gradients: {
            let mut gradients = HashMap::new();
            gradients.insert(
                "primary".to_string(),
                "linear-gradient(135deg, #ea580c 0%, #dc2626 100%)".to_string(),
            );
            gradients.insert(
                "secondary".to_string(),
                "linear-gradient(135deg, #f59e0b 0%, #ea580c 100%)".to_string(),
            );
            gradients
        },
        shadows: {
            let mut shadows = HashMap::new();
            shadows.insert("sm".to_string(), "0 1px 3px rgba(0, 0, 0, 0.4)".to_string());
            shadows.insert(
                "md".to_string(),
                "0 4px 15px rgba(251, 146, 60, 0.3)".to_string(),
            );
            shadows.insert(
                "lg".to_string(),
                "0 8px 25px rgba(251, 146, 60, 0.4)".to_string(),
            );
            shadows
        },
        custom_properties: HashMap::new(),
    }
}
