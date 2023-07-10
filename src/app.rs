use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
	components::navbar::Navbar,
	pages::{home::Home, not_found::NotFound, shop::Shop},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		<Title text="Artilun Webshop" />
		<Link rel="icon" type_="image/png" href="./assets/favicon.png" />
		<Router>
			<Navbar />
			<main>
				<Routes>
					<Route path="/" view=Home />
					<Route path="/shop/:product_id/view" view=Shop />
					<Route path="/*any" view=NotFound />
				</Routes>
			</main>
		</Router>
	}
}
