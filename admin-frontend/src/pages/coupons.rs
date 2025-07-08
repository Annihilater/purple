use leptos::*;

#[component]
pub fn CouponsPage() -> impl IntoView {
    view! {
        <div>
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold text-gray-900">"优惠券管理"</h1>
                <button class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
                    "新增优惠券"
                </button>
            </div>
            <div class="bg-white shadow rounded-lg">
                <div class="p-6">
                    <div class="text-center text-gray-500">
                        "优惠券管理功能开发中..."
                    </div>
                </div>
            </div>
        </div>
    }
}
