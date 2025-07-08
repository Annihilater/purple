use crate::services::auth::AuthService;
use leptos::*;
use purple_shared::LoginRequest;

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if email.get().is_empty() || password.get().is_empty() {
            set_error.set(Some("请填写邮箱和密码".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let login_request = LoginRequest {
            email: email.get(),
            password: password.get(),
        };

        spawn_local(async move {
            match AuthService::login(login_request).await {
                Ok(response) => {
                    // TODO: 保存 token 并跳转到仪表盘
                    leptos_router::use_navigate()("/dashboard", Default::default());
                }
                Err(err) => {
                    set_error.set(Some(format!("登录失败: {}", err)));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "登录到 Purple"
                    </h2>
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
                            placeholder="密码"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    {move || error.get().map(|err| view! {
                        <div class="bg-red-50 border border-red-200 rounded-md p-3">
                            <div class="text-red-800 text-sm">{err}</div>
                        </div>
                    })}

                    <div>
                        <button
                            type="submit"
                            disabled=loading
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
                        >
                            {move || if loading.get() { "登录中..." } else { "登录" }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
