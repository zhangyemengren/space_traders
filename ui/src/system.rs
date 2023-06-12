use api::contracts::list_contracts;
use leptos::*;
use wasm_bindgen::JsValue;
use crate::js_bind::{log};


#[component]
pub fn System(cx: Scope) -> impl IntoView {
    let (_, _) = create_signal(cx, "0".to_string());
    let get_contracts = create_action(cx, |page: &String| {
        let page = page.clone();
        async move { list_contracts(page).await.ok() }
    });
    let datas = get_contracts.value();
    create_effect(cx, move |_| {
        let datas = datas.get();
        match datas {
            Some(v) => log(&serde_wasm_bindgen::to_value(&v).unwrap()),
            _ => log(&JsValue::from_str("no data")),
        }
    });

    view! {
        cx,
        <div>
        "this is system"
            <button on:click= move |_| {get_contracts.dispatch("1".to_string())}>"点击获取所有合同"</button>
        </div>
    }
}
