use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">欢迎使用 Purple 用户平台</h1>
            <p>这是用户前端的首页, 您可以在这里查看和管理您的信息.</p>
        </div>
    }
}
