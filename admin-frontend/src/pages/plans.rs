use leptos::*;

#[component]
pub fn PlansPage() -> impl IntoView {
    view! {
        <div>
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold text-gray-900">"套餐管理"</h1>
                <button class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700">
                    "新增套餐"
                </button>
            </div>
            <div class="bg-white shadow rounded-lg">
                <div class="p-6">
                    <div class="text-center text-gray-500">
                        "套餐管理功能开发中..."
                    </div>
                </div>
            </div>
        </div>
    }
}
