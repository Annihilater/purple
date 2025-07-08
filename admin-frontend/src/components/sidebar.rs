use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="w-64 bg-white shadow-sm">
            <nav class="mt-5 px-2">
                <ul class="space-y-1">
                    <li>
                        <A href="/dashboard"
                           class="group flex items-center px-2 py-2 text-sm font-medium rounded-md hover:bg-gray-50 hover:text-gray-900">
                            "📊 仪表盘"
                        </A>
                    </li>
                    <li>
                        <A href="/users"
                           class="group flex items-center px-2 py-2 text-sm font-medium rounded-md hover:bg-gray-50 hover:text-gray-900">
                            "👥 用户管理"
                        </A>
                    </li>
                    <li>
                        <A href="/plans"
                           class="group flex items-center px-2 py-2 text-sm font-medium rounded-md hover:bg-gray-50 hover:text-gray-900">
                            "📦 套餐管理"
                        </A>
                    </li>
                    <li>
                        <A href="/coupons"
                           class="group flex items-center px-2 py-2 text-sm font-medium rounded-md hover:bg-gray-50 hover:text-gray-900">
                            "🎫 优惠券管理"
                        </A>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
