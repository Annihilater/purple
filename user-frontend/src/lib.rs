use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;
mod services;
mod utils;

use components::layout::Layout;
use pages::{home::HomePage, login::LoginPage};

#[component]
pub fn App() -> impl IntoView {
    // 提供元数据
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/purple-user-frontend.css"/>

        <Title text="Purple - 用户平台"/>

        <Router>
            <main>
                <Routes>
                    // 不需要认证的页面
                    <Route path="/login" view=LoginPage/>
                    
                    // 需要认证的页面，包装在Layout中
                    <Route path="" view=Layout>
                        <Route path="" view=HomePage/>
                        // 此处可添加更多用户相关页面
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App)
} 