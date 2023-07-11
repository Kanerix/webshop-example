use leptos::*;
use leptos_router::A;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="px-64 py-8 bg-red-500 text-white">
			<h1 class="text-center text-xl font-bold">"404"</h1>
			<p class="text-center text-xl">"Page not found"</p>
			<div class="flex justify-center items-center mt-16">
				<A class="px-4 py-2 rounded-md bg-slate-800 text-center text-md font-semibold" href="/">
					"BACK HOME"
				</A>
			</div>
		</div>
	}
}
