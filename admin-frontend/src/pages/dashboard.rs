use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-2xl font-bold text-gray-900 mb-6">"仪表盘"</h1>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="bg-white p-6 rounded-lg shadow">
                    <div class="text-2xl font-bold text-indigo-600">"123"</div>
                    <div class="text-sm text-gray-600">"总用户数"</div>
                </div>
                <div class="bg-white p-6 rounded-lg shadow">
                    <div class="text-2xl font-bold text-green-600">"45"</div>
                    <div class="text-sm text-gray-600">"活跃用户"</div>
                </div>
                <div class="bg-white p-6 rounded-lg shadow">
                    <div class="text-2xl font-bold text-yellow-600">"12"</div>
                    <div class="text-sm text-gray-600">"套餐数量"</div>
                </div>
                <div class="bg-white p-6 rounded-lg shadow">
                    <div class="text-2xl font-bold text-purple-600">"8"</div>
                    <div class="text-sm text-gray-600">"活动优惠券"</div>
                </div>
            </div>
        </div>
    }
}
