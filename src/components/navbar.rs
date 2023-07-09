use std::fmt::Display;

use leptos::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Flyout {
	Women(Vec<Category>),
	Men(Vec<Category>),
	Solutions(Vec<Solution>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Category {
	pub name: String,
	pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Item {
	pub icon: Option<String>,
	pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Solution {
	pub icon: String,
	pub name: String,
	pub description: String,
}

impl Flyout {
	fn get_button_view(&self, cx: Scope) -> View {
		match self {
			Self::Women(_) => "Women".into_view(cx),
			Self::Men(_) => "Men".into_view(cx),
			Self::Solutions(_) => "Solutions".into_view(cx),
		}
	}
}

impl Display for Flyout {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Women(_) => write!(f, "Women"),
			Self::Men(_) => write!(f, "Men"),
			Self::Solutions(_) => write!(f, "Solutions"),
		}
	}
}

impl IntoView for Flyout {
	fn into_view(self, cx: Scope) -> View {
		match self {
			Flyout::Women(categories) => categories.into_view(cx),
			Flyout::Men(categories) => categories.into_view(cx),
			Flyout::Solutions(solutions) => solutions.into_view(cx),
		}
	}
}

impl IntoView for Category {
	fn into_view(self, cx: Scope) -> View {
		view! { cx,
			<div>
				<div class="text-sm font-bold text-left text-slate-400">
					{self.name}
				</div>
				<div class="py-2">
					{self.items}
				</div>
			</div>
		}
		.into_view(cx)
	}
}

impl IntoView for Item {
	fn into_view(self, cx: Scope) -> View {
		let icon = || {
			if let Some(icon) = self.icon {
				Some(view! { cx,
					<i class=format!("{icon} self-center justify-self-start text-xs text-slate-400")></i>
				})
			} else {
				None
			}
		};

		view! { cx,
			<div
				class="grid grid-cols-6 pt-3 pb-2 text-slate-600
				border-b border-transparent
				hover:border-slate-300 hover:cursor-pointer"
			>
				{icon()}
				<div class="col-span-4 self-center justify-self-start text-xs font-bold">
					{self.name}
				</div>
			</div>
		}
		.into_view(cx)
	}
}

impl IntoView for Solution {
	fn into_view(self, cx: Scope) -> View {
		view! { cx,
			<div class="group py-4 px-6 rounded-md cursor-pointer hover:bg-slate-100">
				<div class="w-fit p-2 my-4 rounded-md text-left bg-slate-100 group-hover:bg-white">
					<i class=format!("{} text-2xl text-slate-400 group-hover:text-purple-500", self.icon)></i>
				</div>
				<div class="my-2">
					<div class="mb-2 text-sm text-left text-slate-800 font-bold">
						{self.name}
					</div>
					<div class="text-xs text-left text-slate-600 leading-5">
						{self.description}
					</div>
				</div>
			</div>
		}
		.into_view(cx)
	}
}

#[component]
pub fn FlyoutButton(
	cx: Scope,
	flyout: Flyout,
	flyout_signal: RwSignal<Flyout>,
	active: RwSignal<bool>,
) -> impl IntoView {
	let flyout_kind = flyout.to_string();
	let flyout_element = flyout.get_button_view(cx);
	let active = Signal::derive(cx, move || {
		if active() && flyout_kind == flyout_signal().to_string() {
			true
		} else {
			false
		}
	});

	view! { cx,
		<div
			class=move || {
				format!("py-4 mr-8 text-md font-medium border-b-2 {}",
					if active() {
						"border-purple-500 text-purple-500"
					} else {
						"border-transparent"
					}
				)
			}
			on:mouseenter=move |_| flyout_signal.set(flyout.clone())
		>
			{flyout_element}
		</div>
	}
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
	let women_flyout = Flyout::Women(vec![
		Category {
			name: "Clothing".into(),
			items: vec![
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "T-shirts".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Dresses".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Cardigans".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Jeans".into(),
				},
				Item {
					icon: Some("fa-solid fa-vest".into()),
					name: "Vests".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Coats".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Skirts".into(),
				},
				Item {
					icon: Some("fa-solid fa-socks".into()),
					name: "Underwear".into(),
				},
			],
		},
		Category {
			name: "Brands".into(),
			items: vec![
				Item {
					icon: None,
					name: "Nike".into(),
				},
				Item {
					icon: None,
					name: "Adidas".into(),
				},
				Item {
					icon: None,
					name: "Carhartt".into(),
				},
			],
		},
	]);
	let men_flyout = Flyout::Men(vec![
		Category {
			name: "Clothing".into(),
			items: vec![
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "T-shirts".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Sweaters".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Hoodies".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Jeans".into(),
				},
				Item {
					icon: Some("fa-solid fa-vest".into()),
					name: "Vests".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Tracksuits".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "Shorts".into(),
				},
				Item {
					icon: Some("fa-solid fa-socks".into()),
					name: "Underwear".into(),
				},
			],
		},
		Category {
			name: "Brands".into(),
			items: vec![
				Item {
					icon: None,
					name: "Nike".into(),
				},
				Item {
					icon: None,
					name: "Adidas".into(),
				},
				Item {
					icon: None,
					name: "Carhartt".into(),
				},
				Item {
					icon: None,
					name: "Wood Wood".into(),
				},
				Item {
					icon: None,
					name: "Calvin Klein".into(),
				},
			],
		},
	]);
	let solution_flyout = Flyout::Solutions(vec![
		Solution {
			icon: "fa-solid fa-truck-fast".into(),
			name: "Shipping".into(),
			description: "Learn everything about how our shipping solutions works for the best customer expericence.".into(),
		},
		Solution {
			icon: "fa-solid fa-money-bill".into(),
			name: "Pricing".into(),
			description: "Find out how our pricing works and learn how you as a customer can impact the price of a product.".into(),
		},
		Solution {
			icon: "fa-solid fa-chart-pie".into(),
			name: "Analytics".into(),
			description: "Learn how we found a solution improve our customer experince using collected data.".into(),
		},
		Solution {
			icon: "fa-solid fa-address-book".into(),
			name: "Contact".into(),
			description: "If you have any problems you are always free to contact one of our emplyees.".into(),
		},
	]);

	let show_flyout = create_rw_signal(cx, false);
	let flyout = create_rw_signal::<Flyout>(cx, women_flyout.clone());

	let close_flyout = move |_| {
		show_flyout.set(false);
	};
	let open_flyout = move |_| {
		show_flyout.set(true);
	};

	view! { cx,
		<div on:mouseleave=close_flyout>
			<nav class="text-center border-b border-b-slate-300">
				<div class="px-32 py-2 flex justify-between w-full bg-slate-100">
					<div class="text-xs font-bold text-slate-600">"ALL THE BEST BRANDS"</div>
					<div class="text-xs font-bold text-slate-600">"FREE SHIPPING FOR ORDERS OVER 50$"</div>
					<div class="text-xs font-bold text-slate-600">"30 DAYS RIGHT OF RETURN"</div>
				</div>
				<div class="grid grid-cols-3 px-32">
					<div class="flex w-fit" on:mouseenter=open_flyout>
						<FlyoutButton
							flyout={women_flyout}
							flyout_signal=flyout
							active=show_flyout
						/>
						<FlyoutButton
							flyout={men_flyout}
							flyout_signal=flyout
							active=show_flyout
						/>
						<FlyoutButton
							flyout={solution_flyout}
							flyout_signal=flyout
							active=show_flyout
						/>
					</div>
					<div class="place-self-center text-lg">
						"Artilun"
					</div>
					<div class="justify-self-end self-center">
						<i class="fa-solid fa-shopping-cart text-slate-400 text-2xl"></i>
					</div>
				</div>
				<div class=move || if show_flyout() { "animate-appear" } else { "animate-disappear" }>
					<div class="absolute mt-px px-32 w-full bg-white border-b border-b-slate-300">
						<div class="grid grid-cols-4 gap-x-4 py-4">
							{move || flyout()}
						</div>
					</div>
				</div>
			</nav>
		</div>
	}
}
