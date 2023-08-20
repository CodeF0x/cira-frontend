mod components;
mod models;
mod router;

use crate::components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
