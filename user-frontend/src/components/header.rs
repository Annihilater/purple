use leptos::*;
use leptos_router::*;

use crate::services::auth;

#[component]
pub fn Header() -> impl IntoView {
    let is_logged_in = create_rw_signal(auth::is_authenticated());
    
    let logout = move |_| {
        auth::logout();
        is_logged_in.set(false);
        
        // 重定向到登录页
        let navigate = use_navigate();
        navigate("/login", NavigateOptions::default());
    };
    
    view! {
        <header class="bg-purple-700 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <A href="/" class="text-xl font-bold">Purple 用户平台</A>
                
                <nav>
                    <ul class="flex space-x-4">
                        <li><A href="/" class="hover:underline">首页</A></li>
                        
                        {move || {
                            if is_logged_in.get() {
                                view! {
                                    <Fragment>
                                        <li><A href="/profile" class="hover:underline">个人资料</A></li>
                                        <li><button on:click=logout class="hover:underline">退出</button></li>
                                    </Fragment>
                                }
                            } else {
                                view! {
                                    <Fragment>
                                        <li><A href="/login" class="hover:underline">登录</A></li>
                                    </Fragment>
                                }
                            }
                        }}
                    </ul>
                </nav>
            </div>
        </header>
    }
} 