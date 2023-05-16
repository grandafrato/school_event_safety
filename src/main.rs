use leptos::{mount_to_body, view};
use school_event_safety::app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
