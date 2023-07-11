use std::fmt::Display;

use leptos::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Flyout {
	Women(Vec<Category>),
	Men(Vec<Category>),
	About(Vec<Information>),
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
pub struct Information {
	pub icon: String,
	pub name: String,
	pub description: String,
}

impl Flyout {
	fn get_button_view(&self, cx: Scope) -> View {
		match self {
			Self::Women(_) => "Women".into_view(cx),
			Self::Men(_) => "Men".into_view(cx),
			Self::About(_) => "About".into_view(cx),
		}
	}
}

impl Display for Flyout {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Women(_) => write!(f, "Women"),
			Self::Men(_) => write!(f, "Men"),
			Self::About(_) => write!(f, "About"),
		}
	}
}

impl IntoView for Flyout {
	fn into_view(self, cx: Scope) -> View {
		match self {
			Flyout::Women(categories) => categories.into_view(cx),
			Flyout::Men(categories) => categories.into_view(cx),
			Flyout::About(information) => information.into_view(cx),
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
				<div class="my-2">
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
					<div class="grid w-6 h-6 bg-white rounded-md">
						<i
							class=format!("{icon}
							place-self-center
							text-xs text-slate-400
							group-hover:text-purple-500")
						/>
					</div>
				})
			} else {
				None
			}
		};

		view! { cx,
			<div
				class="group grid grid-cols-6 py-2 text-slate-600
				transition-all ease-in-out duration-200
				cursor-pointer rounded-md 
				hover:bg-slate-200 hover:pl-2"
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

impl IntoView for Information {
	fn into_view(self, cx: Scope) -> View {
		view! { cx,
			<div
				class="group py-4 px-6 rounded-md cursor-pointer
				transition-colors ease-in-out duration-200
				hover:bg-slate-100"
			>
				<div
					class="w-fit p-2 my-4 rounded-md text-left bg-slate-100 
					transition-colors ease-in-out duration-200
					group-hover:bg-white"
				>
					<i
						class=format!("{}
						text-2xl text-slate-400 
						transition-colors ease-in-out duration-200
						group-hover:text-purple-500",
						self.icon) 
					/>
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
					icon: Some("fa-solid fa-vest".into()),
					name: "Vests".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "T-shirts".into(),
				},
				Item {
					icon: Some("fa-solid fa-mitten".into()),
					name: "Gloves".into(),
				},
				Item {
					icon: Some("fa-solid fa-socks".into()),
					name: "Underwear".into(),
				},
				Item {
					icon: Some("fa-solid fa-glasses".into()),
					name: "Accessories".into(),
				},
				Item {
					icon: Some("fa-solid fa-hat-wizard".into()),
					name: "Costumes".into(),
				},

			],
		},
		Category {
			name: "Brands".into(),
			items: vec![
				Item {
					icon: None,
					name: "Zara".into(),
				},
				Item {
					icon: None,
					name: "H&M".into(),
				},
				Item {
					icon: None,
					name: "Forever 21".into(),
				},
				Item {
					icon: None,
					name: "Prada".into(),
				},
				Item {
					icon: None,
					name: "Chanel".into(),
				},
			],
		},
	]);
	let men_flyout = Flyout::Men(vec![
		Category {
			name: "Clothing".into(),
			items: vec![
				Item {
					icon: Some("fa-solid fa-vest".into()),
					name: "Vests".into(),
				},
				Item {
					icon: Some("fa-solid fa-shirt".into()),
					name: "T-shirts".into(),
				},
				Item {
					icon: Some("fa-solid fa-user-tie".into()),
					name: "Suits".into(),
				},
				Item {
					icon: Some("fa-solid fa-socks".into()),
					name: "Underwear".into(),
				},
				Item {
					icon: Some("fa-solid fa-glasses".into()),
					name: "Accessories".into(),
				},
				Item {
					icon: Some("fa-solid fa-crown".into()),
					name: "Costumes".into(),
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
				Item {
					icon: None,
					name: "Off-White".into(),
				},
			],
		},
	]);
	let about_flyout = Flyout::About(vec![
		Information {
			icon: "fa-solid fa-truck-fast".into(),
			name: "Shipping".into(),
			description: "Learn everything about how our shipping works and how we try to create the best customer expericence.".into(),
		},
		Information {
			icon: "fa-solid fa-money-bill".into(),
			name: "Pricing".into(),
			description: "Find out how our pricing works and learn how you as a customer can earn discounts while shopping on our website.".into(),
		},
		Information {
			icon: "fa-solid fa-chart-pie".into(),
			name: "Analytics".into(),
			description: "Learn what data we collect and how we use it to try and improve our customer experince on our webshop.".into(),
		},
		Information {
			icon: "fa-solid fa-headset".into(),
			name: "Support".into(),
			description: "Look through our customer service to see if we can solve an issue of yours or contact an employee.".into(),
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
		<nav on:mouseleave=close_flyout class="text-center border-b border-b-slate-300">
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
						flyout={about_flyout}
						flyout_signal=flyout
						active=show_flyout
					/>
				</div>
				<div class="place-self-center text-lg">
					<img src="assets/full-logo.svg" class="w-28" />
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
	}
}
