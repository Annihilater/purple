use crate::components::theme_selector::ThemeSelector;
use crate::utils::theme::use_theme_system;
use leptos::*;

#[component]
pub fn ThemeSettingsPage() -> impl IntoView {
    let (current_theme, _set_theme, _set_mode, _toggle_mode, _available_themes) =
        use_theme_system();

    view! {
        <div class="theme-settings-page">
            <div class="page-header">
                <h1 class="page-title">"主题设置"</h1>
                <p class="page-subtitle">"自定义你的界面外观"</p>
            </div>

            <div class="theme-settings-content">
                <div class="theme-settings-grid">
                    <div class="theme-settings-panel">
                        <h2 class="card-title">"主题选择器"</h2>
                        <ThemeSelector/>
                    </div>

                    <div class="theme-settings-panel">
                        <h2 class="card-title">"当前主题信息"</h2>
                        <div class="theme-info">
                            <div class="theme-info-item">
                                <span class="theme-info-label">"主题名称："</span>
                                <span class="theme-info-value">{move || current_theme.get().config.name}</span>
                            </div>
                            <div class="theme-info-item">
                                <span class="theme-info-label">"主题描述："</span>
                                <span class="theme-info-value">{move || current_theme.get().config.description}</span>
                            </div>
                            <div class="theme-info-item">
                                <span class="theme-info-label">"当前模式："</span>
                                <span class="theme-info-value">{move || match current_theme.get().mode {
                                    crate::utils::theme::ThemeMode::Light => "浅色模式",
                                    crate::utils::theme::ThemeMode::Dark => "深色模式",
                                }}</span>
                            </div>
                        </div>
                    </div>

                    <div class="theme-settings-panel">
                        <h2 class="card-title">"主题预览"</h2>
                        <div class="theme-preview">
                            <div class="preview-section">
                                <h3 class="preview-title">"颜色展示"</h3>
                                <div class="color-palette">
                                    <div class="color-item" style="background: var(--color-primary);">
                                        <span class="color-label">"主色"</span>
                                    </div>
                                    <div class="color-item" style="background: var(--color-secondary);">
                                        <span class="color-label">"辅色"</span>
                                    </div>
                                    <div class="color-item" style="background: var(--color-accent);">
                                        <span class="color-label">"强调色"</span>
                                    </div>
                                    <div class="color-item" style="background: var(--color-success);">
                                        <span class="color-label">"成功"</span>
                                    </div>
                                    <div class="color-item" style="background: var(--color-warning);">
                                        <span class="color-label">"警告"</span>
                                    </div>
                                    <div class="color-item" style="background: var(--color-error);">
                                        <span class="color-label">"错误"</span>
                                    </div>
                                </div>
                            </div>

                            <div class="preview-section">
                                <h3 class="preview-title">"组件预览"</h3>
                                <div class="component-preview">
                                    <div class="card preview-card">
                                        <h4 class="card-title">"示例卡片"</h4>
                                        <p class="card-text">"这是一个示例卡片，展示主题在不同组件上的效果。"</p>
                                        <div class="card-actions">
                                            <button class="btn">"主要按钮"</button>
                                            <button class="btn-secondary">"次要按钮"</button>
                                        </div>
                                    </div>

                                    <div class="activity-item">
                                        <div style="font-size: 1.25rem; margin-right: 0.75rem;">"📝"</div>
                                        <div>
                                            <div class="activity-title">"示例活动"</div>
                                            <div class="activity-subtitle">"这是一个活动项目示例"</div>
                                        </div>
                                    </div>

                                    <div class="status-item status-success">
                                        <div style="display: flex; align-items: center;">
                                            <div style="font-size: 1.25rem; margin-right: 0.75rem;">"✅"</div>
                                            <div>
                                                <div class="status-title">"系统状态"</div>
                                                <div class="status-subtitle">"运行正常"</div>
                                            </div>
                                        </div>
                                        <div class="status-value status-value-success">"100%"</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
