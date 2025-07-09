use super::types::{ThemeConfig, ThemeMode};

pub fn generate_theme_css(theme: &ThemeConfig, mode: ThemeMode) -> String {
    let theme_class = format!("theme-{}-{}", theme.id, mode.to_string());
    let base_class = format!("theme-{}", theme.id);

    format!(
        r#"
/* {} 主题 - {} 模式 */
.{}, .{} {{
    /* 颜色变量 */
    --color-primary: {};
    --color-secondary: {};
    --color-accent: {};
    --color-background: {};
    --color-surface: {};
    --color-card: {};
    --color-text-primary: {};
    --color-text-secondary: {};
    --color-text-muted: {};
    --color-border: {};
    --color-divider: {};
    --color-success: {};
    --color-warning: {};
    --color-error: {};
    --color-info: {};
    --color-success-bg: {};
    --color-warning-bg: {};
    --color-error-bg: {};
    --color-info-bg: {};
    --color-hover: {};
    --color-active: {};
    --color-disabled: {};
    --shadow: {};
    --overlay: {};
    
    /* 渐变变量 */
    {}
    
    /* 阴影变量 */
    {}
    
    /* 自定义属性 */
    {}
    
    /* 应用背景 */
    background: var(--color-background);
    color: var(--color-text-primary);
    transition: all 0.3s ease;
}}

.{} .header {{
    background: var(--color-surface);
    color: var(--color-text-primary);
    border-bottom: 1px solid var(--color-border);
    box-shadow: var(--shadow);
}}

.{} .sidebar {{
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
}}

.{} .main-content {{
    background: var(--color-background);
}}

.{} .card {{
    background: var(--color-card);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
}}

.{} .page-title {{
    color: var(--color-text-primary);
}}

.{} .page-subtitle {{
    color: var(--color-text-muted);
}}

.{} .activity-item {{
    background: var(--color-hover);
}}

.{} .activity-item:hover {{
    background: var(--color-active);
}}

.{} .activity-title {{
    color: var(--color-text-primary);
}}

.{} .activity-subtitle {{
    color: var(--color-text-muted);
}}

.{} .status-success {{
    background: var(--color-success-bg);
}}

.{} .status-warning {{
    background: var(--color-warning-bg);
}}

.{} .status-title {{
    color: var(--color-text-primary);
}}

.{} .status-subtitle {{
    color: var(--color-text-muted);
}}

.{} .status-value-success {{
    color: var(--color-success);
}}

.{} .status-value-warning {{
    color: var(--color-warning);
}}

.{} .sidebar-link {{
    color: var(--color-text-secondary);
}}

.{} .sidebar-link:hover {{
    background: var(--color-hover);
    color: var(--color-text-primary);
}}

.{} .sidebar-group-title {{
    color: var(--color-text-muted);
}}

.{} .header-logo {{
    color: var(--color-primary);
}}

.{} .header-text {{
    color: var(--color-text-secondary);
}}

.{} .stat-number {{
    color: var(--color-text-primary);
}}

.{} .stat-label {{
    color: var(--color-text-secondary);
}}

.{} .card-title {{
    color: var(--color-text-primary);
}}

.{} .form-input {{
    background: var(--color-surface);
    border: 2px solid var(--color-border);
    color: var(--color-text-primary);
}}

.{} .form-input:focus {{
    border-color: var(--color-primary);
}}

.{} .theme-toggle-btn {{
    border-color: var(--color-border);
}}

.{} .theme-toggle-btn:hover {{
    background: var(--color-hover);
    border-color: var(--color-divider);
}}
"#,
        theme.name,
        match mode {
            ThemeMode::Light => "浅色",
            ThemeMode::Dark => "深色",
        },
        base_class,
        theme_class,
        theme.colors.primary,
        theme.colors.secondary,
        theme.colors.accent,
        theme.colors.background,
        theme.colors.surface,
        theme.colors.card,
        theme.colors.text_primary,
        theme.colors.text_secondary,
        theme.colors.text_muted,
        theme.colors.border,
        theme.colors.divider,
        theme.colors.success,
        theme.colors.warning,
        theme.colors.error,
        theme.colors.info,
        theme.colors.success_bg,
        theme.colors.warning_bg,
        theme.colors.error_bg,
        theme.colors.info_bg,
        theme.colors.hover,
        theme.colors.active,
        theme.colors.disabled,
        theme.colors.shadow,
        theme.colors.overlay,
        theme
            .gradients
            .iter()
            .map(|(k, v)| format!("    --gradient-{k}: {v};"))
            .collect::<Vec<_>>()
            .join("\n"),
        theme
            .shadows
            .iter()
            .map(|(k, v)| format!("    --shadow-{k}: {v};"))
            .collect::<Vec<_>>()
            .join("\n"),
        theme
            .custom_properties
            .iter()
            .map(|(k, v)| format!("    {k}: {v};"))
            .collect::<Vec<_>>()
            .join("\n"),
        theme_class, // header
        theme_class, // sidebar
        theme_class, // main-content
        theme_class, // card
        theme_class, // page-title
        theme_class, // page-subtitle
        theme_class, // activity-item
        theme_class, // activity-item:hover
        theme_class, // activity-title
        theme_class, // activity-subtitle
        theme_class, // status-success
        theme_class, // status-warning
        theme_class, // status-title
        theme_class, // status-subtitle
        theme_class, // status-value-success
        theme_class, // status-value-warning
        theme_class, // sidebar-link
        theme_class, // sidebar-link:hover
        theme_class, // sidebar-group-title
        theme_class, // header-logo
        theme_class, // header-text
        theme_class, // stat-number
        theme_class, // stat-label
        theme_class, // card-title
        theme_class, // form-input
        theme_class, // form-input:focus
        theme_class, // theme-toggle-btn
        theme_class, // theme-toggle-btn:hover
    )
}

pub fn generate_all_themes_css() -> String {
    use super::presets::*;

    let mut css = String::new();

    // 生成所有主题的样式
    let themes = vec![
        get_default_theme(),
        get_default_dark_theme(),
        get_ocean_theme(),
        get_sunset_theme(),
    ];

    for theme in themes {
        css.push_str(&generate_theme_css(&theme, ThemeMode::Light));
        css.push_str(&generate_theme_css(&theme, ThemeMode::Dark));
    }

    css
}
