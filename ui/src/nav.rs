use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <div class="w-[200px]">
            <a href="/" class="block">"to home"</a>
            <a href="/system" class="block">"to system"</a>
            <a href="/fleet" class="block">"to fleet"</a>
        </div>
    }
}