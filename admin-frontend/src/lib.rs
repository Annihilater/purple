use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Stylesheet id="leptos" href="/pkg/purple-admin-frontend.css"/>

        <Title text="Purple - 管理员平台"/>

        <Router>
            <main>
                <Routes>
                    // 不需要认证的页面
                    <Route path="/login" view=LoginPage/>

                    // 需要认证的页面，包装在Layout中
                    <Route path="" view=Layout>
                        <Route path="" view=HomePage/>
                        <Route path="/dashboard" view=DashboardPage/>
                        <Route path="/plans" view=PlansPage/>
                        <Route path="/coupons" view=CouponsPage/>
                        <Route path="/users" view=UsersPage/>
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
