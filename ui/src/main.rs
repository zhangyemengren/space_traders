mod app;
mod js_bind;
mod nav;
mod home;
mod system;

use app::*;
use leptos::*;

pub fn main() {

    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}