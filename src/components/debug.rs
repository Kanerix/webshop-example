use leptos::*;

#[component]
pub fn Debug(cx: Scope, children: Children) -> impl IntoView {
	// Only display in debug assertions
	#[cfg(debug_assertions)]
	view! { cx,
		<div>
			{children(cx)}
		</div>
	}

	// Don't display anything in release mode
	#[cfg(not(debug_assertions))]
	None::<View>.into_view(cx)
}