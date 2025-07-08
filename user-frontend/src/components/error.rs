use leptos::*;

#[component]
pub fn ErrorMessage(
    #[prop(into)] message: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        {move || message.get().map(|msg| view! {
            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                {msg}
            </div>
        })}
    }
}