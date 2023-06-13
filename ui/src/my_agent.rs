use crate::js_bind::log;
use api::{
    agent::{my_agent_details, Data},
    common::Response,
};
use leptos::*;

#[component]
pub fn MyAgent(cx: Scope) -> impl IntoView {
    let result = create_local_resource(
        cx,
        move || (),
        |_| async move { my_agent_details().await.ok() },
    );
    let data = move || {
        let data = result.read(cx);
        match data {
            Some(Some(Response::Success(success))) => {
                log(&wasm_bindgen::JsValue::from_str("success"));
                let Data {
                    account_id,
                    symbol,
                    headquarters,
                    credits,
                    starting_faction,
                } = success.data;
                (account_id, symbol, headquarters, credits, starting_faction)
            }
            _ => (
                "".to_string(),
                "".to_string(),
                "".to_string(),
                0,
                "".to_string(),
            ),
        }
    };
    view! {
        cx,
        <div>
            "个人信息"
            <ul>
                <li>"ID " {move || data().0}</li>
                <li>"name " {move || data().1}</li>
                <li>"headquarters " {move || data().2}</li>
                <li>"credits " {move || data().3}</li>
                <li>"startingFaction " {move || data().4}</li>
            </ul>
        </div>
    }
}
