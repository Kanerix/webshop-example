use std::{rc::Rc, collections::VecDeque};

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{pages::shop::Shop, components::navbar::Navbar, contexts::cart::Cart};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/shop")]
    Shop,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Shop => html! { <Shop /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
	let cart = use_memo((), |_| Cart::new());

    html! {
		<ContextProvider<Rc<Cart>> context={cart}>
			<Navbar />
			<BrowserRouter>
				<Switch<Route> render={switch} />
			</BrowserRouter>
		</ContextProvider<Rc<Cart>>>
    }
}
