use leptos::*;

#[derive(Clone)]
pub struct CartItem {
	pub name: String,
	pub amount: usize,
	pub price: f64,
}

#[derive(Copy, Clone)]
pub struct ShoppingCart(pub RwSignal<Vec<CartItem>>);

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
