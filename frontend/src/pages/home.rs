use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto">
            <div class="text-center">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">
                    "欢迎使用 Purple 管理系统"
                </h1>
                <p class="text-xl text-gray-600 mb-8">
                    "现代化的 Rust 全栈 Web 应用"
                </p>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-12">
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-2">"用户管理"</h3>
                        <p class="text-gray-600">"管理系统用户，设置权限和状态"</p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-2">"套餐管理"</h3>
                        <p class="text-gray-600">"创建和管理各种服务套餐"</p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow">
                        <h3 class="text-lg font-semibold mb-2">"优惠券管理"</h3>
                        <p class="text-gray-600">"发放和管理优惠券活动"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
