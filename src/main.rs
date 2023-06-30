mod app;
mod components;
mod contexts;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
