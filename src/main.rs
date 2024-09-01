mod components;
mod logic;

use crate::components::game_panel::game_panel;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        h1 { "MAZE GAME" }
        game_panel {

        }
    }
}
