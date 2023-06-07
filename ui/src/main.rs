mod app;
mod js_bind;
mod nav;
mod home;
mod system;

use app::*;
use leptos::*;
use js_bind::{js_console};

pub fn main() {

    js_console();

    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}