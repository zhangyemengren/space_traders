mod app;

use app::*;
use leptos::*;

pub fn main() {

    println!("csr mode - mounting to body");

    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}