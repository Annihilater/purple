use leptos::*;

#[component]
pub fn ErrorBoundary(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full bg-white shadow-lg rounded-lg p-6">
                <div class="flex items-center mb-4">
                    <div class="flex-shrink-0">
                        <svg class="h-6 w-6 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                        </svg>
                    </div>
                    <div class="ml-3">
                        <h3 class="text-sm font-medium text-gray-800">"发生错误"</h3>
                    </div>
                </div>
                <div class="text-sm text-gray-600">
                    {message}
                </div>
                <div class="mt-4">
                    <button
                        class="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        on:click=move |_| {
                            web_sys::window().unwrap().location().reload().unwrap();
                        }
                    >
                        "重新加载页面"
                    </button>
                </div>
            </div>
        </div>
    }
}
