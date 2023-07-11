use leptos::*;
use leptos_router::A;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="flex justify-between px-64 py-8 bg-blue-600 text-xl text-white">
			<div class="flex flex-col justify-between">
				<div>
					<h1 class="font-semibold">
						"Discounts up to 70% on all brands!"
					</h1>
					<h2 class="mt-1">
						"Get a new refreshing look for the season!"
					</h2>
				</div>
				<div>
					<A
						class="group justify-self-end font-medium"
						href="/shop"
					>
						"SHOP NOW"
						<i
							class="fa-solid fa-arrow-right ml-2 group-hover:ml-3
							transition-all ease-in-out duration-200"
						/>
					</A>
				</div>
			</div>
			<img src="assets/250x400.png" />
		</div>
	}
}
