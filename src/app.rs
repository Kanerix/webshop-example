use leptos::*;
use leptos_router::*;

use crate::{
	components::navbar::Navbar,
	pages::{home::Home, not_found::NotFound},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	view! { cx,
		<Routes>
			<Navbar />
			<main>
				<Route path="/" view=|cx| view! { cx, <Home /> } />
				<Route path="/*any" view=|cx| view! { cx, <NotFound /> } />
			</main>
		</Routes>
	}
}
