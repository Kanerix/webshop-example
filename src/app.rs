use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
	components::navbar::Navbar,
	context::cart::CartProvider,
	pages::{checkout::Checkout, home::Home, not_found::NotFound, shop::Shop},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	provide_meta_context(cx);

	view! { cx,
		<Title text="Webshop Example" />
		<Link rel="icon" type_="image/png" href="./assets/favicon.png" />
		<CartProvider>
			<Router>
				<Navbar />
				<main>
					<Routes>
						<Route path="/" view=Home />
						<Route path="/shop" view=Shop />
						<Route path="/checkout" view=Checkout />
						<Route path="/*any" view=NotFound />
					</Routes>
				</main>
			</Router>
		</CartProvider>
	}
}
