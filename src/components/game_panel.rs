use dioxus::prelude::*;
use crate::logic::{board::{generate_board, Board}, tile::Tile};

const BOARD_WIDTH: usize = 31;
const BOARD_HEIGHT: usize = 31;

fn handle_key_down(event: KeyboardEvent, board: &mut Signal<Board>) {
     match event.key() {
        Key::ArrowUp => board.write().move_player_up(),
        Key::ArrowDown => board.write().move_player_down(),
        Key::ArrowLeft => board.write().move_player_left(),
        Key::ArrowRight=> board.write().move_player_right(),
        _ => {} // Handle other keys if necessary
    }
}

pub fn game_panel() -> Element {
    let mut board = use_signal( || {
        let mut b = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
        generate_board(&mut b);
        b
    });

    rsx! {
        div {
            class: "panel",
            tabindex: "0", 
            onkeydown: move |event| handle_key_down(event, &mut board),
            prevent_default: "onkeydown",
            if board.read().game_done {
                h2 {
                    "You Win!"
                }
            }
            else {
                h2 {
                    "Reach the Exit!"
                }
            }
            div {
                class: "board",
                for y in 0..board.read().size.y {
                    div {
                        class: "board-row",
                        for x in 0..board.read().size.x {
                            if (x == board.read().player_pos.x) && (y == board.read().player_pos.y) {
                                div {class: "tile-player"}
                            }
                            else {
                                match *board.read().get(x, y) {
                                    Tile::Empty => rsx!(div {class: "tile-empty"}),
                                    Tile::Wall => rsx!(div {class: "tile-wall"}),
                                    Tile::Exit => rsx!(div {class: "tile-exit"}),
                                }  
                            }
                             
                        }
                    }
                }
            },
            p {
                "Use Arrow keys to navigate the player (Red Square) about the maze, until you reach the exit (Green Square)"
            }
        }
    }
}