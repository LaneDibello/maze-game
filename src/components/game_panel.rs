use dioxus::prelude::*;

use crate::logic::{board::{generate_board, Board}, tile::Tile};
pub fn game_panel() -> Element {
    let mut board = Board::new(51, 51);
    generate_board(&mut board);
    
    rsx! {
        div {
            class: "panel",
            div {
                class: "board",
                for y in 0..board.size.y {
                    div {
                        class: "board-row",
                        for x in 0..board.size.x {
                            match *board.get(x, y) {
                                Tile::empty => rsx!(div {class: "tile-empty"}),
                                Tile::wall => rsx!(div {class: "tile-wall"}),
                                Tile::exit => rsx!(div {class: "tile-exit"}),
                            }   
                        }
                    }
                }
            }
        }
    }
}