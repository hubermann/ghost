mod app;
mod api;
mod components;
mod config;
mod services;
mod domain;
mod stores;
mod routes;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
