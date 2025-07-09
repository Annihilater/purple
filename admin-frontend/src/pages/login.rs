use crate::services::auth::AuthService;
use leptos::*;
use purple_shared::{LoginRequest, RegisterRequest};

#[derive(Clone, Copy, PartialEq)]
enum AuthMode {
    Login,
    Register,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (mode, set_mode) = create_signal(AuthMode::Login);
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (confirm_password, set_confirm_password) = create_signal("".to_string());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some("请填写邮箱和密码".to_string()));
            return;
        }

        // 注册模式下验证确认密码
        if mode.get() == AuthMode::Register {
            if password.get() != confirm_password.get() {
                set_error.set(Some("两次输入的密码不一致".to_string()));
                return;
            }
            if password.get().len() < 6 {
                set_error.set(Some("密码长度至少6位".to_string()));
                return;
            }
        }

        set_loading.set(true);
        set_error.set(None);

        match mode.get() {
            AuthMode::Login => {
                let login_request = LoginRequest {
                    email: email.get(),
                    password: password.get(),
                };

                spawn_local(async move {
                    match AuthService::login(login_request).await {
                        Ok(response) => {
                            // 保存token
                            AuthService::save_token(&response.token);
                            // 跳转到管理员区域
                            leptos_router::use_navigate()("/admin/dashboard", Default::default());
                        }
                        Err(err) => {
                            set_error.set(Some(format!("登录失败: {}", err)));
                        }
                    }
                    set_loading.set(false);
                });
            }
            AuthMode::Register => {
                let register_request = RegisterRequest {
                    email: email.get(),
                    password: password.get(),
                    username: None,
                };

                spawn_local(async move {
                    match AuthService::register(register_request).await {
                        Ok(_) => {
                            set_mode.set(AuthMode::Login);
                            set_error.set(None);
                            // 清空表单
                            set_email.set("".to_string());
                            set_password.set("".to_string());
                            set_confirm_password.set("".to_string());
                            // 显示成功消息
                            set_error.set(Some("注册成功！请登录".to_string()));
                        }
                        Err(err) => {
                            set_error.set(Some(format!("注册失败: {}", err)));
                        }
                    }
                    set_loading.set(false);
                });
            }
        }
    };

    view! {
        <div class="login-container fade-in">
            <div class="login-form">
                <h1 class="login-title">
                    {move || if mode.get() == AuthMode::Login { "欢迎回来" } else { "创建账户" }}
                </h1>
                <p class="login-subtitle">
                    {move || if mode.get() == AuthMode::Login {
                        "登录您的 Purple 管理账户"
                    } else {
                        "注册新的 Purple 管理账户"
                    }}
                </p>

                // 切换按钮
                <div class="toggle-container">
                    <button
                        type="button"
                        class=move || if mode.get() == AuthMode::Login { "toggle-btn active" } else { "toggle-btn" }
                        on:click=move |_| {
                            set_mode.set(AuthMode::Login);
                            set_error.set(None);
                        }
                    >
                        "登录"
                    </button>
                    <button
                        type="button"
                        class=move || if mode.get() == AuthMode::Register { "toggle-btn active" } else { "toggle-btn" }
                        on:click=move |_| {
                            set_mode.set(AuthMode::Register);
                            set_error.set(None);
                        }
                    >
                        "注册"
                    </button>
                </div>

                <form on:submit=on_submit>
                    <div class="form-group">
                        <input
                            type="email"
                            class="form-input"
                            placeholder="请输入邮箱地址"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            required
                        />
                    </div>

                    <div class="form-group">
                        <input
                            type="password"
                            class="form-input"
                            placeholder={move || if mode.get() == AuthMode::Register { "请输入密码（至少6位）" } else { "请输入密码" }}
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            required
                        />
                    </div>

                    // 注册模式下显示确认密码字段
                    {move || if mode.get() == AuthMode::Register {
                        view! {
                            <div class="form-group">
                                <input
                                    type="password"
                                    class="form-input"
                                    placeholder="请确认密码"
                                    prop:value=confirm_password
                                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                    required
                                />
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }}

                    {move || error.get().map(|err| {
                        let is_success = err.contains("注册成功");
                        view! {
                            <div class={if is_success { "success-message" } else { "error-message" }}>
                                {err}
                            </div>
                        }
                    })}

                    <button
                        type="submit"
                        class="btn"
                        style="width: 100%; margin-top: 1rem;"
                        disabled=loading
                    >
                        {move || if loading.get() {
                            if mode.get() == AuthMode::Login { "登录中..." } else { "注册中..." }
                        } else {
                            if mode.get() == AuthMode::Login { "立即登录" } else { "立即注册" }
                        }}
                    </button>
                </form>

                <div style="text-align: center; margin-top: 2rem; padding-top: 2rem; border-top: 1px solid rgba(102, 126, 234, 0.2);">
                    <p style="color: #718096; font-size: 0.875rem;">
                        {move || if mode.get() == AuthMode::Login {
                            "还没有账户？点击上方注册按钮"
                        } else {
                            "已有账户？点击上方登录按钮"
                        }}
                    </p>
                </div>
            </div>
        </div>
    }
}
