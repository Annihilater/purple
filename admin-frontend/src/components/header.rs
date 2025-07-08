use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-sm border-b border-gray-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex items-center">
                        <A href="/" class="text-xl font-bold text-indigo-600">
                            "Purple"
                        </A>
                    </div>
                    <div class="flex items-center space-x-4">
                        <span class="text-sm text-gray-700">"管理员"</span>
                        <button class="bg-indigo-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-indigo-700">
                            "退出"
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}
