use crate::components::{header::Header, sidebar::Sidebar};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div style="min-height: 100vh; background-color: #f7fafc;">
            <Header/>
            <div style="display: flex;">
                <Sidebar/>
                <main style="flex: 1; padding: 1.5rem;">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}
