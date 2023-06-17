use crate::{fleet::Fleet, home::Home, my_agent::MyAgent, nav::Nav, system::System};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="h-full flex flex-col">
            <MyAgent />
            <div class="flex grow">
                <Router>
                    <Nav/>
                    <div class="grow">
                        <Routes>
                            <Route path="/" view= move |cx| view! { cx, <Home/> }/>
                            <Route path="/system" view= move |cx| view! { cx, <System/> }/>
                            <Route path="/fleet" view= move |cx| view! { cx, <Fleet/> }/>
                        </Routes>
                    </div>
                </Router>
            </div>
        </div>
    }
}
