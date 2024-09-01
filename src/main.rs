mod components;
mod logic;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use logic::board::{generate_board, Board};
use crate::components::game_panel::game_panel;

fn main() {
    let mut board = Board::new(50, 50);
    generate_board(&mut board);

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
