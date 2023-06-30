use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
	pub name: String,
	pub price: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cart {
	pub items: VecDeque<Item>,
}

impl Cart {
	pub fn new() -> Self {
		Self { items: VecDeque::new() }
	}

	pub fn add(&mut self, item: Item) {
		self.items.push_front(item);
	}

	pub fn remove(&mut self, index: usize) {
		self.items.remove(index);
	}

	pub fn get_total(&self) -> f64 {
		self.items.iter().map(|item| item.price).sum()
	}
}