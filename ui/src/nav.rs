use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <ul class="menu bg-base-200 w-56 [&_li>*]:rounded-none">
          <li><a href="/">"to home"</a></li>
          <li><a href="/system">"to system"</a></li>
          <li><a href="/fleet">"to fleet"</a></li>
        </ul>
    }
}