use api::contracts::list_contracts;
use leptos::*;
use wasm_bindgen::JsValue;
use crate::js_bind::{js_console};

async fn send_new_todo_to_api(task: String) -> String {
    // do something...
    // return a task id
    "42".to_string()
}

#[component]
pub fn System(cx: Scope) -> impl IntoView {
    let (_, _) = create_signal(cx, "0".to_string());
    let get_contracts = create_action(cx, |page: &String| {
        send_new_todo_to_api(page.clone())
    });
    let datas = get_contracts.value();
    let res = datas.get_untracked();
    match res {
        Some(v) => js_console(&serde_wasm_bindgen::to_value(&v).unwrap()),
        _ => js_console(&JsValue::from_str("no data")),
    }
    view! {
        cx,
        <div>
        "this is system"
            <button on:click= move |_| {get_contracts.dispatch("1".to_string())}>"点击获取所有合同"</button>
        </div>
    }
}
