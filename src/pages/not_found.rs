use leptos::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="px-64 py-8 bg-red-500 text-white">
			<h1 class="text-center text-xl font-bold">"404"</h1>
			<p class="text-center text-xl">"Page not found"</p>
		</div>
	}
}
