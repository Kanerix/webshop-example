use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="px-64 py-8 bg-blue-600 text-xl text-white">
			<h1 class="font-semibold">
				"Discounts up to 70% on all brands!"
			</h1>
			<h2 class="mt-1">
				"Get a new refreshing look for the season!"
			</h2>
		</div>
	}
}
