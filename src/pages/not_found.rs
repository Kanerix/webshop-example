use leptos::*;
use leptos_router::A;

use crate::context::cart::{ShoppingCart, CartItem};

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
	let cart = use_context::<ShoppingCart>(cx).unwrap().0;
	let add_item = move |_| {
		cart.update(move |cart| {
			cart.push(CartItem {
				name: "test".into(),
				amount: 10,
				price: 10.,
			});
		});
	};

	view! { cx,
		<div class="px-64 py-8 bg-red-500 text-white">
			<h1 class="text-center text-xl font-bold">"404"</h1>
			<p class="text-center text-xl">"Page not found"</p>
			<div class="flex justify-center items-center mt-16">
				<A
					href="/"
					class="px-4 py-2 rounded-md bg-slate-800
					text-center text-md font-semibold" 
				>
					"BACK HOME"
				</A>
			</div>
			// Test button for shopping cart
			<div class="flex justify-center items-center mt-16">
				<button on:click=add_item class="mt-2 p-2 rounded-md bg-slate-800">
					"Cart"
				</button>
			</div>
		</div>
	}
}
