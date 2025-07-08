use leptos::*;
use leptos_router::*;

use crate::components::header::Header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <Header/>
            <main class="flex-grow container mx-auto p-4">
                {children()}
            </main>
            <footer class="bg-gray-100 p-4 text-center text-sm text-gray-600">
                <p>"Copyright 2023 Purple 用户平台"</p>
            </footer>
        </div>
    }
}
