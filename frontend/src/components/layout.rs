use crate::components::{header::Header, sidebar::Sidebar};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <Header/>
            <div class="flex">
                <Sidebar/>
                <main class="flex-1 p-6">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}
