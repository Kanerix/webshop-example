use leptos::*;

#[component]
pub fn NavbarButton(cx: Scope) -> impl IntoView {
	view! { cx,
		<a>
			{"test"}
		</a>
	}
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
	let (show, set_show) = create_signal(cx, false);
	let toggle_popover = move |_| set_show(!show());

	let (categories, set_categories) = create_signal(
		cx,
		vec![
			"Hats",
			"Accessories",
			"Jackets",
			"Sweatshirts",
			"Shirts",
			"Pants",
			"Shorts",
			"Shoes",
		]
		.iter()
		.map(|s| s.to_string()),
	);

	view! { cx,
		<div
			on:mouseenter=toggle_popover
			on:mouseleave=toggle_popover
		>
			<nav class="text-center border-b border-b-slate-300">
				<div class="px-32 py-2 flex justify-between w-full bg-slate-100">
					<div class="text-xs font-bold text-slate-600">"ALL THE BEST BRANDS"</div>
					<div class="text-xs font-bold text-slate-600">"FREE SHIPPING FOR ORDERS OVER 50$"</div>
					<div class="text-xs font-bold text-slate-600">"30 DAYS RIGHT OF RETURN"</div>
				</div>
				<div class="px-32 py-4 w-full text-left">
					<div class="px-4 flex items-center border border-slate-300 rounded-full bg-slate-100">
						<i class="fa-solid fa-magnifying-glass mr-4 text-slate-300"></i>
						<input
							class="py-2 self-stretch w-full outline-none bg-slate-100"
							placeholder="Search..."
						/>
					</div>
				</div>
				<Show when=move || show() fallback=|_| ()>
					<div class="animate-slideIn absolute w-full bg-slate-200">
						<For
							each=move || categories()
							// a unique key for each item
							key=|category| category.clone()
							// renders each item to a view
							view=move |cx, category: String| {
								view! { cx,
									<div>{category}</div>
								}
							}
						/>
					</div>
				</Show>
			</nav>
		</div>
	}
}
