use leptos::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Kind {
	Women,
	Men,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Flyout {
	pub kind: Kind,
	pub categories: Vec<Category>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Category {
	// TODO: Find better solution for rendering list with unique keys
	pub id: u32,
	pub name: String,
	pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Item {
	pub icon: Option<String>,
	pub name: String,
}

impl IntoView for Kind {
	fn into_view(self, cx: Scope) -> View {
		match self {
			Kind::Women => "Women".into_view(cx),
			Kind::Men => "Men".into_view(cx),
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
		let icon = move || {
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

#[component]
pub fn FlyoutButton(
	cx: Scope,
	flyout: Flyout,
	flyout_signal: RwSignal<Flyout>,
	active: RwSignal<bool>,
) -> impl IntoView {
	let button_header = flyout.kind.into_view(cx);
	let active = Signal::derive(cx, move || {
		if active() && flyout.kind == flyout_signal().kind {
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
						"border-purple-800 text-purple-800"
					} else {
						"border-transparent"
					}
				)
			}
			on:mouseenter=move |_| flyout_signal.set(flyout.clone())
		>
			{button_header}
		</div>
	}
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
	let women_flyout = Flyout {
		kind: Kind::Women,
		categories: vec![
			Category {
				id: 100,
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
				id: 101,
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
		],
	};
	let men_flyout = Flyout {
		kind: Kind::Men,
		categories: vec![
			Category {
				id: 200,
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
				id: 201,
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
		],
	};

	let show_flyout = create_rw_signal(cx, true);
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
							<For
								each=move || flyout().categories
								key=|category| category.id
								view=move |_, category: Category| category
							/>
						</div>
					</div>
				</div>
			</nav>
		</div>
	}
}
