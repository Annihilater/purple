use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-purple-500"></div>
        </div>
    }
}