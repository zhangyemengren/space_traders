use crate::js_bind::log;
use api::agent::my_agent_details;
use leptos::*;

#[component]
pub fn MyAgent(cx: Scope) -> impl IntoView {
    let result = create_local_resource(
        cx,
        move || (),
        |_| async move { my_agent_details().await.ok() },
    );
    let account_id = move || {
        let data = result.read(cx);
        match data {
            None => return "none".to_string(),
            _ => {}
        };
        let data = data.unwrap();
        match data {
            None => return "none".to_string(),
            _ => {}
        };
        let data = data.unwrap();
        match data {
            api::common::Response::Success(success) => success.data.account_id,
            _ => "none".to_string(),
        }
    };
    view! {
        cx,
        <div>
            "个人信息"
            <ul>
                <li>{account_id}</li>
            </ul>
        </div>
    }
}
