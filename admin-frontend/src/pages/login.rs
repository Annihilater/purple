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
                            // 跳转到仪表盘
                            leptos_router::use_navigate()("/dashboard", Default::default());
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
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {move || if mode.get() == AuthMode::Login { "登录到 Purple" } else { "注册 Purple 账户" }}
                    </h2>
                </div>
                
                // 切换按钮
                <div class="flex justify-center space-x-4">
                    <button
                        type="button"
                        class=move || if mode.get() == AuthMode::Login { 
                            "px-4 py-2 text-sm font-medium text-indigo-600 bg-indigo-50 border border-indigo-200 rounded-md" 
                        } else { 
                            "px-4 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50" 
                        }
                        on:click=move |_| {
                            set_mode.set(AuthMode::Login);
                            set_error.set(None);
                        }
                    >
                        "登录"
                    </button>
                    <button
                        type="button"
                        class=move || if mode.get() == AuthMode::Register { 
                            "px-4 py-2 text-sm font-medium text-indigo-600 bg-indigo-50 border border-indigo-200 rounded-md" 
                        } else { 
                            "px-4 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50" 
                        }
                        on:click=move |_| {
                            set_mode.set(AuthMode::Register);
                            set_error.set(None);
                        }
                    >
                        "注册"
                    </button>
                </div>

                <form class="mt-8 space-y-6" on:submit=on_submit>
                    <div>
                        <label for="email" class="sr-only">"邮箱地址"</label>
                        <input
                            id="email"
                            name="email"
                            type="email"
                            required
                            class="appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                            placeholder="邮箱地址"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                    </div>
                    <div>
                        <label for="password" class="sr-only">"密码"</label>
                        <input
                            id="password"
                            name="password"
                            type="password"
                            required
                            class="appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                            placeholder={move || if mode.get() == AuthMode::Register { "密码（至少6位）" } else { "密码" }}
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    // 注册模式下显示确认密码字段
                    {move || if mode.get() == AuthMode::Register {
                        view! {
                            <div>
                                <label for="confirm_password" class="sr-only">"确认密码"</label>
                                <input
                                    id="confirm_password"
                                    name="confirm_password"
                                    type="password"
                                    required
                                    class="appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                    placeholder="确认密码"
                                    prop:value=confirm_password
                                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                />
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }}

                    {move || error.get().map(|err| {
                        let is_success = err.contains("注册成功");
                        view! {
                            <div class={if is_success { "bg-green-50 border border-green-200 rounded-md p-3" } else { "bg-red-50 border border-red-200 rounded-md p-3" }}>
                                <div class={if is_success { "text-green-800 text-sm" } else { "text-red-800 text-sm" }}>{err}</div>
                            </div>
                        }
                    })}

                    <div>
                        <button
                            type="submit"
                            disabled=loading
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
                        >
                            {move || if loading.get() { 
                                if mode.get() == AuthMode::Login { "登录中..." } else { "注册中..." }
                            } else { 
                                if mode.get() == AuthMode::Login { "登录" } else { "注册" }
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
