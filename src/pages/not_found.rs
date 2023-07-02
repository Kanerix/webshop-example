use leptos::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
	view! { cx,
		<div>
			<h1>404</h1>
			<p>Page not found</p>
		</div>
	}
}
