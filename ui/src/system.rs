use crate::js_bind::log;
use api::{
    common::{Response, Waypoints},
    contracts::list_contracts,
    system::{get_system, Data},
};
use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub fn System(cx: Scope) -> impl IntoView {
    // 系统名输入框
    let (system_name, set_system_name) = create_signal(cx, "".to_string());
    let get_contracts = create_action(cx, |page: &String| {
        let page = page.clone();
        async move { list_contracts(page).await.ok() }
    });
    let get_system = create_action(cx, |system: &String| {
        let system = system.clone();
        async move { get_system(system).await.ok() }
    });
    let datas = get_contracts.value();
    create_effect(cx, move |_| {
        let datas = datas.get();
        match datas {
            Some(v) => log(&serde_wasm_bindgen::to_value(&v).unwrap()),
            _ => log(&JsValue::from_str("no data")),
        }
    });
    let systems = move || {
        let data = get_system.value().get();
        match data {
            Some(Some(Response::Success(success))) => {
                let Data {
                    waypoints,
                    ..
                } = success.data;
                waypoints
                    .into_iter()
                    .map(|waypoints| {
                        let Waypoints{
                            symbol,
                            the_type,
                            x,
                            y,
                        } = waypoints;
                        view! {
                            cx,
                            <li>
                                <div>"name: "{move || symbol.clone()}</div>
                                <div>"the_type "{move || the_type.clone()}</div>
                                <div>"position-x "{move || x}</div>
                                <div>"position-y "{move || y}</div>
                            </li>
                        }
                    })
                    .collect_view(cx)
            }
            // leptos 已经为()实现了IntoView trait
            _ => ().into_view(cx),
        }
    };

    view! {
        cx,
        <div>
        "this is system"
            <div>
                <button class="block btn" on:click= move |_| {get_contracts.dispatch("1".to_string())}>"点击获取所有合同"</button>
                <button class="block btn" on:click= move |_| {get_system.dispatch(system_name.get())}>"点击获取系统详细信息"</button>
                <input
                    type="text"
                    placeholder="Type here"
                    class="input input-bordered input-primary w-full max-w-xs"
                    on:input= move |e| {
                        set_system_name.set(event_target_value(&e))
                    }
                    prop:value=move || system_name.get().to_string()
                />
            </div>
            <div>
                <ul>{move || systems()}</ul>
            </div>
        </div>
    }
}
