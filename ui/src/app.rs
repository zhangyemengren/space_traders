use leptos::*;
use leptos_router::*;
use crate::{nav::Nav, home::Home, system::System, my_agent::MyAgent};

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
                    </Routes>
                </Router>
            </div>
        </div>
    }
}

