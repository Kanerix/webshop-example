pub mod app;
pub mod components;
pub mod pages;

use app::App;
use leptos::*;

fn main() {
	mount_to_body(|cx| view! { cx, <App /> })
}
