use leptos::*;
use leptos_router::*;

use crate::services::auth;

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal(None::<String>);

    let navigate = use_navigate();

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let password = password.get();

        // 简单验证
        if username.is_empty() || password.is_empty() {
            set_error.set(Some("用户名和密码不能为空".to_string()));
            return;
        }

        // 克隆navigate以便在闭包中使用
        let nav = navigate.clone();
        spawn_local(async move {
            match auth::login(&username, &password).await {
                Ok(_) => {
                    // 登录成功, 重定向到首页
                    nav("/", NavigateOptions::default());
                }
                Err(e) => {
                    set_error.set(Some(format!("登录失败: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="flex min-h-screen items-center justify-center">
            <div class="w-full max-w-md p-8 space-y-8 bg-white rounded-lg shadow">
                <div>
                    <h1 class="text-2xl font-bold text-center">用户登录</h1>
                    <p class="text-center text-gray-600">输入您的账号信息登录</p>
                </div>

                {move || error.get().map(|err| view! {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                        {err}
                    </div>
                })}

                <form on:submit=on_submit>
                    <div class="space-y-4">
                        <div>
                            <label for="username" class="block text-sm font-medium">用户名</label>
                            <input
                                id="username"
                                type="text"
                                on:input=move |ev| set_username.set(event_target_value(&ev))
                                prop:value=username
                                class="w-full px-3 py-2 border rounded-md"
                            />
                        </div>

                        <div>
                            <label for="password" class="block text-sm font-medium">密码</label>
                            <input
                                id="password"
                                type="password"
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                prop:value=password
                                class="w-full px-3 py-2 border rounded-md"
                            />
                        </div>

                        <button type="submit" class="w-full py-2 px-4 bg-blue-600 text-white rounded-md">
                            登录
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
