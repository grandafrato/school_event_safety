use leptos::{mount_to_body, view};
use school_event_safety::app::{App, AppProps};

fn main() {
    mount_to_body(|cx| view! { cx, <App />})
}
