use leptos::*;
use leptos_router::*;
use crate::{nav::Nav, home::Home, system::System};

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    view! {
        cx,
        <div class="h-full flex">
            <Router>
                <Nav/>
                <Routes>
                    <Route path="/" view= move |cx| view! { cx, <Home/> }/>
                    <Route path="/system" view= move |cx| view! { cx, <System/> }/>
                </Routes>
            </Router>
        </div>
    }
}

