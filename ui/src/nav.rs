use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <div class="w-[100px]">
            <a href="/">"to home"</a>
            <a href="/system">"to system"</a>
        </div>
    }
}