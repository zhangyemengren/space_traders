use crate::{fleet::Fleet, home::Home, my_agent::MyAgent, nav::Nav, system::System};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <MyAgent />
            <div class="h-full flex">
                <Router>
                    <Nav/>
                    <Routes>
                        <Route path="/" view= move |cx| view! { cx, <Home/> }/>
                        <Route path="/system" view= move |cx| view! { cx, <System/> }/>
                        <Route path="/fleet" view= move |cx| view! { cx, <Fleet/> }/>
                    </Routes>
                </Router>
            </div>
        </div>
    }
}
