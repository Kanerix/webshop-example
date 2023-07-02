use leptos::*;

use crate::components::logo::Logo;

#[component]
pub fn navbar_button(cx: Scope) -> impl IntoView {
	view! { cx,
		<a>
			{"test"}
		</a>
	}
}

#[component]
pub fn navbar(cx: Scope) -> impl IntoView {
	view! { cx,
		<nav class="text-center border-b border-b-slate-300">
			<div class="px-32 py-2 flex justify-between w-full bg-slate-100">
				<div class="text-xs font-bold text-slate-600">"ALL THE BEST BRANDS"</div>
				<div class="text-xs font-bold text-slate-600">"FREE SHIPPING FOR ORDERS OVER 50$"</div>
				<div class="text-xs font-bold text-slate-600">"30 DAYS RIGHT OF RETURN"</div>
			</div>
			<div class="px-32 flex justify-center">
				<div class="pt-4 w-32">
					<Logo />
				</div>
			</div>
			<div class="px-32 pt-4 w-full text-left">
				<div class="px-4 flex items-center border border-slate-600 rounded-full">
					<i class="fa-solid fa-magnifying-glass mr-4 text-slate-600"></i>
					<input class="py-2 self-stretch w-full outline-none" placeholder="Search..." />
				</div>
			</div>
			<div class="px-32 pt-4">
				"Elements!"
			</div>
		</nav>
	}
}
