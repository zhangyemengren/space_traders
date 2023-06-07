use api::contracts::list_contracts;
use leptos::*;

#[component]
pub fn System(cx: Scope) -> impl IntoView {
    let (_, _) = create_signal(cx, "0".to_string());
    let get_contracts = create_action(cx, |page: &String| {
        list_contracts(page.clone())
    });
    view! {
        cx,
        <div>
        "this is system"
            <button on:click= move |_| {get_contracts.dispatch("1".to_string())}>"点击获取所有合同"</button>
        </div>
    }
}
