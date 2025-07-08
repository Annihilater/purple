use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
            <span class="ml-2 text-gray-600">"加载中..."</span>
        </div>
    }
}

#[component]
pub fn ErrorMessage(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="bg-red-50 border border-red-200 rounded-md p-4">
            <div class="text-red-800">{message}</div>
        </div>
    }
}
