use leptos::*;

// TODO: Move this struct to shop page and use real product data
#[derive(Clone)]
pub struct Product {
	_name: String,
	_price: f64,
}

#[derive(Copy, Clone)]
pub struct ShoppingCart(pub RwSignal<Vec<Product>>);

#[component]
pub fn CartProvider(cx: Scope, children: Children) -> impl IntoView {
	let cart = create_rw_signal(cx, vec![]);

	provide_context(cx, ShoppingCart(cart));

	view! { cx,
		<div>
			{children(cx)}
		</div>
	}
}
