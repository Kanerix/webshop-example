pub mod app;
pub mod components;
pub mod context;
pub mod pages;

use app::App;
use leptos::*;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();
	mount_to_body(|cx| view! { cx, <App /> })
}
