use crate::components::{auth_guard::AuthGuard, header::Header, sidebar::Sidebar};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <AuthGuard>
            <div style="min-height: 100vh;">
                <Header/>
                <div style="display: flex;">
                    <Sidebar/>
                    <main class="main-content" style="flex: 1; padding: 1.5rem;">
                        <Outlet/>
                    </main>
                </div>
            </div>
        </AuthGuard>
    }
}
