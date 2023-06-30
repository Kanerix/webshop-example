use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! {
		<nav class="text-center bg-red-400">
			<div class="px-20 py-1 flex justify-between w-full bg-slate-100">
				<div class="text-sm">{ "ALL THE BEST BRANDS" }</div>
				<div class="text-sm">{ "FREE SHIPPING FOR ORDERS OVER 50$" }</div>
				<div class="text-sm">{ "30 DAYS RIGHT OF RETURN" }</div>
			</div>
			<div class="py-2 w-full text-xl font-semibold">{ "Artilun" }</div>
		</nav>
	}
}