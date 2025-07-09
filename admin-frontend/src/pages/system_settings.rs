use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSettings {
    pub site_name: String,
    pub site_url: String,
    pub site_description: String,
    pub logo_url: String,
    pub favicon_url: String,
    pub contact_email: String,
    pub support_url: String,
    pub terms_url: String,
    pub privacy_url: String,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            site_name: "Purple VPN".to_string(),
            site_url: "https://purple.example.com".to_string(),
            site_description: "高速稳定的VPN服务".to_string(),
            logo_url: "/assets/logo.png".to_string(),
            favicon_url: "/assets/favicon.ico".to_string(),
            contact_email: "support@purple.example.com".to_string(),
            support_url: "https://help.purple.example.com".to_string(),
            terms_url: "/terms".to_string(),
            privacy_url: "/privacy".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_encryption: String, // none, ssl, tls
    pub from_email: String,
    pub from_name: String,
    pub test_email: String,
}

impl Default for EmailSettings {
    fn default() -> Self {
        Self {
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: "".to_string(),
            smtp_password: "".to_string(),
            smtp_encryption: "tls".to_string(),
            from_email: "noreply@purple.example.com".to_string(),
            from_name: "Purple VPN".to_string(),
            test_email: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSettings {
    pub currency: String,
    pub stripe_enabled: bool,
    pub stripe_public_key: String,
    pub stripe_secret_key: String,
    pub alipay_enabled: bool,
    pub alipay_app_id: String,
    pub alipay_private_key: String,
    pub wechat_enabled: bool,
    pub wechat_app_id: String,
    pub wechat_secret: String,
}

impl Default for PaymentSettings {
    fn default() -> Self {
        Self {
            currency: "CNY".to_string(),
            stripe_enabled: false,
            stripe_public_key: "".to_string(),
            stripe_secret_key: "".to_string(),
            alipay_enabled: true,
            alipay_app_id: "".to_string(),
            alipay_private_key: "".to_string(),
            wechat_enabled: true,
            wechat_app_id: "".to_string(),
            wechat_secret: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub register_enabled: bool,
    pub email_verification: bool,
    pub captcha_enabled: bool,
    pub captcha_provider: String, // recaptcha, hcaptcha
    pub captcha_site_key: String,
    pub captcha_secret_key: String,
    pub password_min_length: u8,
    pub session_timeout: u32, // minutes
    pub max_login_attempts: u8,
    pub lockout_duration: u32, // minutes
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            register_enabled: true,
            email_verification: true,
            captcha_enabled: false,
            captcha_provider: "recaptcha".to_string(),
            captcha_site_key: "".to_string(),
            captcha_secret_key: "".to_string(),
            password_min_length: 8,
            session_timeout: 1440, // 24 hours
            max_login_attempts: 5,
            lockout_duration: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub build_date: String,
    pub php_version: String,
    pub database_version: String,
    pub server_info: String,
    pub disk_usage: String,
    pub memory_usage: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            build_date: "2024-01-01".to_string(),
            php_version: "Rust 1.75.0".to_string(),
            database_version: "PostgreSQL 15.0".to_string(),
            server_info: "Linux Ubuntu 22.04".to_string(),
            disk_usage: "45% (12GB / 25GB)".to_string(),
            memory_usage: "65% (2.6GB / 4GB)".to_string(),
        }
    }
}

#[component]
pub fn SystemSettings() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    // 根据 URL 路径确定初始标签页
    let initial_tab = create_memo(move |_| match location.pathname.get().as_str() {
        "/admin/settings/payment" => "payment".to_string(),
        "/admin/settings/system" => "system".to_string(),
        _ => "site".to_string(),
    });

    let (active_tab, set_active_tab) = create_signal(initial_tab.get_untracked());

    // 监听 URL 变化并更新标签页
    create_effect(move |_| {
        set_active_tab.set(initial_tab.get());
    });

    let (site_settings, set_site_settings) = create_signal(SiteSettings::default());
    let (email_settings, set_email_settings) = create_signal(EmailSettings::default());
    let (payment_settings, set_payment_settings) = create_signal(PaymentSettings::default());
    let (security_settings, set_security_settings) = create_signal(SecuritySettings::default());
    let (system_info, _set_system_info) = create_signal(SystemInfo::default());
    let (saving, set_saving) = create_signal(false);
    let (success_message, set_success_message) = create_signal(Option::<String>::None);

    let tabs = vec![
        ("site", "站点设置", "🌐"),
        ("email", "邮件设置", "📧"),
        ("payment", "支付设置", "💳"),
        ("security", "安全设置", "🔒"),
        ("system", "系统信息", "ℹ️"),
    ];

    let save_settings = move |_| {
        set_saving.set(true);

        // 模拟保存操作
        set_timeout(
            move || {
                set_saving.set(false);
                set_success_message.set(Some("设置保存成功！".to_string()));

                // 3秒后清除成功消息
                set_timeout(
                    move || set_success_message.set(None),
                    std::time::Duration::from_secs(3),
                );
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="system-settings-page">
            <div class="page-header">
                <h1 class="page-title">"系统设置"</h1>
                <p class="page-subtitle">"配置站点、邮件、支付和安全相关设置"</p>
            </div>

            <div class="settings-container">
                // 侧边栏标签页
                <div class="settings-sidebar">
                    <div class="settings-tabs">
                        {tabs.into_iter().map(|(key, label, icon)| {
                            let tab_key = key.to_string();
                            let tab_key_clone = tab_key.clone();
                            let navigate = navigate.clone();

                            view! {
                                <button
                                    class="settings-tab"
                                    class:active=move || active_tab.get() == tab_key
                                    on:click=move |_| {
                                        let url = match tab_key_clone.as_str() {
                                            "site" => "/admin/settings/system",
                                            "email" => "/admin/settings/system",
                                            "payment" => "/admin/settings/payment",
                                            "security" => "/admin/settings/system",
                                            "system" => "/admin/settings/system",
                                            _ => "/admin/settings/system",
                                        };
                                        navigate(url, Default::default());
                                    }
                                >
                                    <span class="tab-icon">{icon}</span>
                                    <span class="tab-label">{label}</span>
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // 主要设置内容
                <div class="settings-content">
                    // 成功消息
                    {move || {
                        success_message.get().map(|msg| {
                            view! {
                                <div class="success-banner">
                                    <span class="success-icon">"✅"</span>
                                    <span class="success-text">{msg}</span>
                                </div>
                            }
                        })
                    }}

                    // 站点设置
                    <div class="settings-panel" class:active=move || active_tab.get() == "site">
                        <SiteSettingsPanel
                            settings=site_settings
                            set_settings=set_site_settings
                        />
                    </div>

                    // 邮件设置
                    <div class="settings-panel" class:active=move || active_tab.get() == "email">
                        <EmailSettingsPanel
                            settings=email_settings
                            set_settings=set_email_settings
                        />
                    </div>

                    // 支付设置
                    <div class="settings-panel" class:active=move || active_tab.get() == "payment">
                        <PaymentSettingsPanel
                            settings=payment_settings
                            set_settings=set_payment_settings
                        />
                    </div>

                    // 安全设置
                    <div class="settings-panel" class:active=move || active_tab.get() == "security">
                        <SecuritySettingsPanel
                            settings=security_settings
                            set_settings=set_security_settings
                        />
                    </div>

                    // 系统信息
                    <div class="settings-panel" class:active=move || active_tab.get() == "system">
                        <SystemInfoPanel system_info=system_info />
                    </div>

                    // 保存按钮 (除了系统信息页面外都显示)
                    <div class="settings-actions" class:hidden=move || active_tab.get() == "system">
                        <button
                            class="btn btn-primary save-btn"
                            class:loading=saving
                            on:click=save_settings
                            disabled=saving
                        >
                            {move || if saving.get() { "保存中..." } else { "保存设置" }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SiteSettingsPanel(
    settings: ReadSignal<SiteSettings>,
    set_settings: WriteSignal<SiteSettings>,
) -> impl IntoView {
    view! {
        <div class="settings-form">
            <h2 class="form-title">"站点设置"</h2>
            <p class="form-description">"配置站点的基本信息和外观"</p>

            <div class="form-grid">
                <div class="form-group">
                    <label class="form-label">"站点名称"</label>
                    <input
                        type="text"
                        class="form-input"
                        prop:value=move || settings.get().site_name
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.site_name = value);
                        }
                    />
                    <span class="form-hint">"显示在浏览器标题栏和页面头部"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"站点URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().site_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.site_url = value);
                        }
                    />
                    <span class="form-hint">"站点的完整URL地址"</span>
                </div>

                <div class="form-group span-2">
                    <label class="form-label">"站点描述"</label>
                    <textarea
                        class="form-textarea"
                        rows="3"
                        prop:value=move || settings.get().site_description
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.site_description = value);
                        }
                    ></textarea>
                    <span class="form-hint">"站点的简短描述，用于SEO和页面介绍"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"Logo URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().logo_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.logo_url = value);
                        }
                    />
                    <span class="form-hint">"站点Logo图片地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"Favicon URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().favicon_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.favicon_url = value);
                        }
                    />
                    <span class="form-hint">"站点图标地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"联系邮箱"</label>
                    <input
                        type="email"
                        class="form-input"
                        prop:value=move || settings.get().contact_email
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.contact_email = value);
                        }
                    />
                    <span class="form-hint">"用户联系的邮箱地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"帮助中心URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().support_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.support_url = value);
                        }
                    />
                    <span class="form-hint">"帮助文档和支持页面地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"服务条款URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().terms_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.terms_url = value);
                        }
                    />
                    <span class="form-hint">"服务条款页面地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"隐私政策URL"</label>
                    <input
                        type="url"
                        class="form-input"
                        prop:value=move || settings.get().privacy_url
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.privacy_url = value);
                        }
                    />
                    <span class="form-hint">"隐私政策页面地址"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn EmailSettingsPanel(
    settings: ReadSignal<EmailSettings>,
    set_settings: WriteSignal<EmailSettings>,
) -> impl IntoView {
    let test_email = move |_| {
        // 模拟发送测试邮件
        web_sys::window()
            .unwrap()
            .alert_with_message("测试邮件已发送，请检查收件箱")
            .unwrap();
    };

    view! {
        <div class="settings-form">
            <h2 class="form-title">"邮件设置"</h2>
            <p class="form-description">"配置SMTP邮件服务器设置"</p>

            <div class="form-grid">
                <div class="form-group">
                    <label class="form-label">"SMTP服务器"</label>
                    <input
                        type="text"
                        class="form-input"
                        prop:value=move || settings.get().smtp_host
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.smtp_host = value);
                        }
                    />
                    <span class="form-hint">"邮件服务器地址，如 smtp.gmail.com"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"SMTP端口"</label>
                    <input
                        type="number"
                        class="form-input"
                        prop:value=move || settings.get().smtp_port.to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(port) = value.parse::<u16>() {
                                set_settings.update(|s| s.smtp_port = port);
                            }
                        }
                    />
                    <span class="form-hint">"常用端口：25, 465, 587"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"用户名"</label>
                    <input
                        type="text"
                        class="form-input"
                        prop:value=move || settings.get().smtp_username
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.smtp_username = value);
                        }
                    />
                    <span class="form-hint">"SMTP认证用户名"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"密码"</label>
                    <input
                        type="password"
                        class="form-input"
                        prop:value=move || settings.get().smtp_password
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.smtp_password = value);
                        }
                    />
                    <span class="form-hint">"SMTP认证密码或应用专用密码"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"加密方式"</label>
                    <select
                        class="form-select"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.smtp_encryption = value);
                        }
                    >
                        <option value="none" selected=move || settings.get().smtp_encryption == "none">"无加密"</option>
                        <option value="ssl" selected=move || settings.get().smtp_encryption == "ssl">"SSL"</option>
                        <option value="tls" selected=move || settings.get().smtp_encryption == "tls">"TLS"</option>
                    </select>
                    <span class="form-hint">"选择SMTP连接加密方式"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"发件人邮箱"</label>
                    <input
                        type="email"
                        class="form-input"
                        prop:value=move || settings.get().from_email
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.from_email = value);
                        }
                    />
                    <span class="form-hint">"系统邮件的发件人地址"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"发件人名称"</label>
                    <input
                        type="text"
                        class="form-input"
                        prop:value=move || settings.get().from_name
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.from_name = value);
                        }
                    />
                    <span class="form-hint">"系统邮件的发件人显示名称"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"测试邮箱"</label>
                    <div class="input-group">
                        <input
                            type="email"
                            class="form-input"
                            placeholder="输入测试邮箱地址"
                            prop:value=move || settings.get().test_email
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_settings.update(|s| s.test_email = value);
                            }
                        />
                        <button
                            class="btn btn-secondary"
                            on:click=test_email
                        >
                            "发送测试"
                        </button>
                    </div>
                    <span class="form-hint">"输入邮箱地址并发送测试邮件"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PaymentSettingsPanel(
    settings: ReadSignal<PaymentSettings>,
    set_settings: WriteSignal<PaymentSettings>,
) -> impl IntoView {
    view! {
        <div class="settings-form">
            <h2 class="form-title">"支付设置"</h2>
            <p class="form-description">"配置支付网关和货币设置"</p>

            <div class="form-grid">
                <div class="form-group">
                    <label class="form-label">"默认货币"</label>
                    <select
                        class="form-select"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.currency = value);
                        }
                    >
                        <option value="CNY" selected=move || settings.get().currency == "CNY">"人民币 (CNY)"</option>
                        <option value="USD" selected=move || settings.get().currency == "USD">"美元 (USD)"</option>
                        <option value="EUR" selected=move || settings.get().currency == "EUR">"欧元 (EUR)"</option>
                        <option value="HKD" selected=move || settings.get().currency == "HKD">"港币 (HKD)"</option>
                    </select>
                    <span class="form-hint">"网站使用的默认货币"</span>
                </div>

                // Stripe 设置
                <div class="form-section span-2">
                    <h3 class="section-title">
                        <span>"Stripe 支付"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().stripe_enabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.stripe_enabled = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </h3>

                    <div class="form-grid" class:disabled=move || !settings.get().stripe_enabled>
                        <div class="form-group">
                            <label class="form-label">"Stripe 公钥"</label>
                            <input
                                type="text"
                                class="form-input"
                                prop:value=move || settings.get().stripe_public_key
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.stripe_public_key = value);
                                }
                                disabled=move || !settings.get().stripe_enabled
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label">"Stripe 私钥"</label>
                            <input
                                type="password"
                                class="form-input"
                                prop:value=move || settings.get().stripe_secret_key
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.stripe_secret_key = value);
                                }
                                disabled=move || !settings.get().stripe_enabled
                            />
                        </div>
                    </div>
                </div>

                // 支付宝设置
                <div class="form-section span-2">
                    <h3 class="section-title">
                        <span>"支付宝"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().alipay_enabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.alipay_enabled = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </h3>

                    <div class="form-grid" class:disabled=move || !settings.get().alipay_enabled>
                        <div class="form-group">
                            <label class="form-label">"应用ID"</label>
                            <input
                                type="text"
                                class="form-input"
                                prop:value=move || settings.get().alipay_app_id
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.alipay_app_id = value);
                                }
                                disabled=move || !settings.get().alipay_enabled
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label">"私钥"</label>
                            <textarea
                                class="form-textarea"
                                rows="3"
                                prop:value=move || settings.get().alipay_private_key
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.alipay_private_key = value);
                                }
                                disabled=move || !settings.get().alipay_enabled
                            ></textarea>
                        </div>
                    </div>
                </div>

                // 微信支付设置
                <div class="form-section span-2">
                    <h3 class="section-title">
                        <span>"微信支付"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().wechat_enabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.wechat_enabled = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </h3>

                    <div class="form-grid" class:disabled=move || !settings.get().wechat_enabled>
                        <div class="form-group">
                            <label class="form-label">"应用ID"</label>
                            <input
                                type="text"
                                class="form-input"
                                prop:value=move || settings.get().wechat_app_id
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.wechat_app_id = value);
                                }
                                disabled=move || !settings.get().wechat_enabled
                            />
                        </div>

                        <div class="form-group">
                            <label class="form-label">"应用密钥"</label>
                            <input
                                type="password"
                                class="form-input"
                                prop:value=move || settings.get().wechat_secret
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_settings.update(|s| s.wechat_secret = value);
                                }
                                disabled=move || !settings.get().wechat_enabled
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SecuritySettingsPanel(
    settings: ReadSignal<SecuritySettings>,
    set_settings: WriteSignal<SecuritySettings>,
) -> impl IntoView {
    view! {
        <div class="settings-form">
            <h2 class="form-title">"安全设置"</h2>
            <p class="form-description">"配置用户注册、验证和安全策略"</p>

            <div class="form-grid">
                <div class="form-group">
                    <label class="form-label switch-label">
                        <span>"允许用户注册"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().register_enabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.register_enabled = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </label>
                    <span class="form-hint">"关闭后新用户无法注册账户"</span>
                </div>

                <div class="form-group">
                    <label class="form-label switch-label">
                        <span>"邮箱验证"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().email_verification
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.email_verification = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </label>
                    <span class="form-hint">"新用户注册后需要验证邮箱"</span>
                </div>

                <div class="form-group">
                    <label class="form-label switch-label">
                        <span>"启用验证码"</span>
                        <label class="switch">
                            <input
                                type="checkbox"
                                prop:checked=move || settings.get().captcha_enabled
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_settings.update(|s| s.captcha_enabled = checked);
                                }
                            />
                            <span class="slider"></span>
                        </label>
                    </label>
                    <span class="form-hint">"在登录和注册页面显示验证码"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"验证码类型"</label>
                    <select
                        class="form-select"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.captcha_provider = value);
                        }
                        disabled=move || !settings.get().captcha_enabled
                    >
                        <option value="recaptcha" selected=move || settings.get().captcha_provider == "recaptcha">"Google reCAPTCHA"</option>
                        <option value="hcaptcha" selected=move || settings.get().captcha_provider == "hcaptcha">"hCaptcha"</option>
                    </select>
                </div>

                <div class="form-group">
                    <label class="form-label">"验证码站点密钥"</label>
                    <input
                        type="text"
                        class="form-input"
                        prop:value=move || settings.get().captcha_site_key
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.captcha_site_key = value);
                        }
                        disabled=move || !settings.get().captcha_enabled
                    />
                </div>

                <div class="form-group">
                    <label class="form-label">"验证码私钥"</label>
                    <input
                        type="password"
                        class="form-input"
                        prop:value=move || settings.get().captcha_secret_key
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_settings.update(|s| s.captcha_secret_key = value);
                        }
                        disabled=move || !settings.get().captcha_enabled
                    />
                </div>

                <div class="form-group">
                    <label class="form-label">"密码最小长度"</label>
                    <input
                        type="number"
                        class="form-input"
                        min="6"
                        max="32"
                        prop:value=move || settings.get().password_min_length.to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(length) = value.parse::<u8>() {
                                set_settings.update(|s| s.password_min_length = length);
                            }
                        }
                    />
                    <span class="form-hint">"用户密码的最小长度要求"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"会话超时时间（分钟）"</label>
                    <input
                        type="number"
                        class="form-input"
                        min="30"
                        max="10080"
                        prop:value=move || settings.get().session_timeout.to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(timeout) = value.parse::<u32>() {
                                set_settings.update(|s| s.session_timeout = timeout);
                            }
                        }
                    />
                    <span class="form-hint">"用户无操作后自动退出的时间"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"最大登录尝试次数"</label>
                    <input
                        type="number"
                        class="form-input"
                        min="3"
                        max="10"
                        prop:value=move || settings.get().max_login_attempts.to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(attempts) = value.parse::<u8>() {
                                set_settings.update(|s| s.max_login_attempts = attempts);
                            }
                        }
                    />
                    <span class="form-hint">"连续登录失败后锁定账户"</span>
                </div>

                <div class="form-group">
                    <label class="form-label">"锁定持续时间（分钟）"</label>
                    <input
                        type="number"
                        class="form-input"
                        min="5"
                        max="1440"
                        prop:value=move || settings.get().lockout_duration.to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(duration) = value.parse::<u32>() {
                                set_settings.update(|s| s.lockout_duration = duration);
                            }
                        }
                    />
                    <span class="form-hint">"账户锁定后多长时间自动解锁"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SystemInfoPanel(system_info: ReadSignal<SystemInfo>) -> impl IntoView {
    view! {
        <div class="settings-form">
            <h2 class="form-title">"系统信息"</h2>
            <p class="form-description">"查看系统版本和运行状态"</p>

            <div class="info-grid">
                <div class="info-card">
                    <div class="info-icon">"🚀"</div>
                    <div class="info-content">
                        <h3 class="info-title">"系统版本"</h3>
                        <p class="info-value">{move || system_info.get().version}</p>
                        <span class="info-label">"构建日期: "{move || system_info.get().build_date}</span>
                    </div>
                </div>

                <div class="info-card">
                    <div class="info-icon">"⚡"</div>
                    <div class="info-content">
                        <h3 class="info-title">"运行环境"</h3>
                        <p class="info-value">{move || system_info.get().php_version}</p>
                        <span class="info-label">"数据库: "{move || system_info.get().database_version}</span>
                    </div>
                </div>

                <div class="info-card">
                    <div class="info-icon">"🖥️"</div>
                    <div class="info-content">
                        <h3 class="info-title">"服务器信息"</h3>
                        <p class="info-value">{move || system_info.get().server_info}</p>
                    </div>
                </div>

                <div class="info-card">
                    <div class="info-icon">"💾"</div>
                    <div class="info-content">
                        <h3 class="info-title">"磁盘使用率"</h3>
                        <p class="info-value">{move || system_info.get().disk_usage}</p>
                    </div>
                </div>

                <div class="info-card">
                    <div class="info-icon">"🧠"</div>
                    <div class="info-content">
                        <h3 class="info-title">"内存使用率"</h3>
                        <p class="info-value">{move || system_info.get().memory_usage}</p>
                    </div>
                </div>

                <div class="info-card">
                    <div class="info-icon">"📊"</div>
                    <div class="info-content">
                        <h3 class="info-title">"系统状态"</h3>
                        <p class="info-value status-good">"运行正常"</p>
                        <span class="info-label">"所有服务正常运行"</span>
                    </div>
                </div>
            </div>

            <div class="system-actions">
                <button class="btn btn-secondary">"清理缓存"</button>
                <button class="btn btn-secondary">"优化数据库"</button>
                <button class="btn btn-secondary">"检查更新"</button>
                <button class="btn btn-warning">"重启应用"</button>
            </div>
        </div>
    }
}
