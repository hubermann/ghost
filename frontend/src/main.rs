mod app;
mod components;
mod services;
mod domain;
mod stores;
mod routes;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
