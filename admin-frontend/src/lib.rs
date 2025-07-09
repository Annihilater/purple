use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

mod components;
mod pages;
mod services;
mod utils;

use components::layout::Layout;
use pages::{
    coupons::CouponsPage, dashboard::DashboardPage, home::HomePage, login::LoginPage,
    plans::PlansPage, users::UsersPage,
};

#[component]
pub fn App() -> impl IntoView {
    // 提供元数据
    provide_meta_context();

    view! {
        <Title text="Purple - 管理员平台"/>

        <Router>
            <Routes>
                // 公开页面 - 无需认证
                <Route path="/" view=HomePage/>
                <Route path="/login" view=LoginPage/>
                
                // 需要认证的页面，包装在Layout中
                <Route path="/admin" view=Layout>
                    <Route path="/" view=DashboardPage/>
                    <Route path="/dashboard" view=DashboardPage/>
                    <Route path="/plans" view=PlansPage/>
                    <Route path="/coupons" view=CouponsPage/>
                    <Route path="/users" view=UsersPage/>
                </Route>
            </Routes>
        </Router>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    // 初始化日志
    console_log::init_with_level(log::Level::Debug).expect("日志初始化失败");
    console_error_panic_hook::set_once();

    // 添加调试信息
    web_sys::console::log_1(&"开始初始化 Leptos 应用".into());

    // 等待 DOM 就绪
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // 检查 DOM 是否就绪
    if document.ready_state() == "loading" {
        // 如果还在加载，等待 DOMContentLoaded 事件
        let closure = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"DOM 已就绪，开始挂载应用".into());
            leptos::mount_to_body(App);
        }) as Box<dyn Fn()>);

        document
            .add_event_listener_with_callback("DOMContentLoaded", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    } else {
        // DOM 已就绪，直接挂载
        web_sys::console::log_1(&"DOM 已就绪，开始挂载应用".into());
        leptos::mount_to_body(App);
    }

    // 添加调试信息
    web_sys::console::log_1(&"Leptos 应用初始化完成".into());
}
