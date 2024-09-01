use super::tile::Tile;
use rand::Rng;

const PASSAGE_LENGTH: usize = 2;

/// Represents an (x, y) coordinate on our game board
/// Also used for storing size
#[derive(Clone, PartialEq, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Creates a new coordinate (x, y)
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Represents the Game Board the maze is built upon
pub struct Board {
    /// True if the game has been won
    pub game_done: bool,
    /// Current location of the player on the board
    pub player_pos: Coord,
    /// Starting position of the player
    start_pos: Coord,
    /// Size of teh board in units (width, height)
    pub size: Coord,
    /// Stores the actual Tile data for the baord
    data: Vec<Tile>,
}

impl Board {
    /// Creates a new game board of size (width, height)
    pub fn new(width: usize, height: usize) -> Self {
        let area: usize = (width * height) as usize;
        Self {
            game_done: false,
            player_pos: Coord::new(0, 0),
            start_pos: Coord::new(0, 0),
            size: Coord::new(width, height),
            data: vec![Tile::Wall; area],
        }
    }

    /// Gets a reference to the tile at (x, y), or a wall if none exists
    pub fn get(&self, x: usize, y: usize) -> &Tile {
        match self.data.get(x + y * self.size.x) {
            Some(tile) => tile,
            None => &Tile::Wall,
        }
    }

    /// True if there is a tile at (x, y) that is a wall
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if (x >= self.size.x) || (y >= self.size.y) {
            return false;
        }
        *self.get(x, y) == Tile::Wall
    }

    /// True if there is a tile at (x, y) that is empty
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        if (x >= self.size.x) || (y >= self.size.y) {
            return false;
        }
        *self.get(x, y) == Tile::Empty
    }

    /// Sets the tile at (x, y) to `tile`, if one exists
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        match self.data.get_mut(x + y * self.size.x) {
            None => (),
            Some(t) => {
                t.clone_from(&tile);
            }
        }
    }

    /// Get all adjacent tiles that are a PASSAGE_LENGTH away and are walls
    pub fn get_adjacent_walls(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut walls = Vec::new();
        if let Some(new_x) = x.checked_add(PASSAGE_LENGTH) {
            if self.is_wall(new_x, y) {
                walls.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(PASSAGE_LENGTH) {
            if self.is_wall(new_x, y) {
                walls.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(PASSAGE_LENGTH) {
            if self.is_wall(x, new_y) {
                walls.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(PASSAGE_LENGTH) {
            if self.is_wall(x, new_y) {
                walls.push(Coord::new(x, new_y));
            }
        }

        walls
    }

    /// Get all adjacent tiles that are a PASSAGE_LENGTH away and are empty
    pub fn get_adjacent_empty(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut empties = Vec::new();
        if let Some(new_x) = x.checked_add(PASSAGE_LENGTH) {
            if self.is_empty(new_x, y) {
                empties.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(PASSAGE_LENGTH) {
            if self.is_empty(new_x, y) {
                empties.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(PASSAGE_LENGTH) {
            if self.is_empty(x, new_y) {
                empties.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(PASSAGE_LENGTH) {
            if self.is_empty(x, new_y) {
                empties.push(Coord::new(x, new_y));
            }
        }

        empties
    }

    /// Get all directly adjacent tiles that are empty (meaning they connect to this tile)
    pub fn get_adjacent_connections(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut connections = Vec::new();
        if let Some(new_x) = x.checked_add(1) {
            if self.is_empty(new_x, y) {
                connections.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(1) {
            if self.is_empty(new_x, y) {
                connections.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(1) {
            if self.is_empty(x, new_y) {
                connections.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(1) {
            if self.is_empty(x, new_y) {
                connections.push(Coord::new(x, new_y));
            }
        }

        connections
    }

    /// Make a passage between two tiles
    /// NOTE: this currenlty only works with a PASSAGE_LENGTH = 2
    pub fn make_passage(&mut self, start: Coord, end: Coord) {
        let dx: isize = (start.x as isize) - (end.x as isize);
        let dy: isize = (start.y as isize) - (end.y as isize);
        if dx == 0 {
            if dy < 0 {
                self.set(start.x, start.y + 1, Tile::Empty);
            }
            if dy > 0 {
                self.set(start.x, start.y - 1, Tile::Empty);
            }
        } else if dy == 0 {
            if dx < 0 {
                self.set(start.x + 1, start.y, Tile::Empty);
            }
            if dx > 0 {
                self.set(start.x - 1, start.y, Tile::Empty);
            }
        }
        self.set(start.x, start.y, Tile::Empty);
        self.set(end.x, end.y, Tile::Empty);
    }

    /// Moves player up if not blocked by walls or edges
    pub fn move_player_up(&mut self) {
        let new_y = match self.player_pos.y.checked_sub(1) {
            Some(u) => u,
            None => return,
        };

        if new_y < self.size.y {
            let new_tile = self.get(self.player_pos.x, new_y).clone();
            if new_tile == Tile::Exit {
                self.game_done = true;
            }
            if new_tile != Tile::Wall {
                self.player_pos.y = new_y;
            }
        }
    }

    /// Moves player down if not blocked by walls or edges
    pub fn move_player_down(&mut self) {
        let new_y = match self.player_pos.y.checked_add(1) {
            Some(u) => u,
            None => return,
        };

        if new_y < self.size.y {
            let new_tile = self.get(self.player_pos.x, new_y).clone();
            if new_tile == Tile::Exit {
                self.game_done = true;
            }
            if new_tile != Tile::Wall {
                self.player_pos.y = new_y;
            }
        }
    }

    /// Moves player left if not blocked by walls or edges
    pub fn move_player_left(&mut self) {
        let new_x = match self.player_pos.x.checked_sub(1) {
            Some(u) => u,
            None => return,
        };

        if new_x < self.size.x {
            let new_tile = self.get(new_x, self.player_pos.y).clone();
            if new_tile == Tile::Exit {
                self.game_done = true;
            }
            if new_tile != Tile::Wall {
                self.player_pos.x = new_x;
            }
        }
    }

    /// Moves player right if not blocked by walls or edges
    pub fn move_player_right(&mut self) {
        let new_x = match self.player_pos.x.checked_add(1) {
            Some(u) => u,
            None => return,
        };

        if new_x < self.size.x {
            let new_tile = self.get(new_x, self.player_pos.y).clone();
            if new_tile == Tile::Exit {
                self.game_done = true;
            }
            if new_tile != Tile::Wall {
                self.player_pos.x = new_x;
            }
        }
    }

    /// Used For debug purposes
    /// Pretty prints the maze as text art
    pub fn pretty_print(&self) -> String {
        let mut output: String = String::new();
        for y in 0..self.size.y {
            let mut row: String = String::new();
            for x in 0..self.size.x {
                match self.get(x, y) {
                    Tile::Wall => row.push('█'),
                    Tile::Empty => row.push(' '),
                    Tile::Exit => row.push('x'),
                }
            }
            row.push('\n');
            output.push_str(&row)
        }
        output
    }
}

/// Use's a ranomized version of Prim's Minimum Spanning Tree Algorithm to generate a maze
pub fn generate_board(board: &mut Board) {
    board.set(board.start_pos.x, board.start_pos.y, Tile::Empty);

    let mut walls: Vec<Coord> = Vec::new();

    walls.append(&mut board.get_adjacent_walls(board.start_pos.x, board.start_pos.y));

    let mut last_wall = Coord::new(board.start_pos.x, board.start_pos.y);

    while walls.len() > 0 {
        let i = rand::thread_rng().gen_range(0..walls.len());
        let wall: Coord = walls.remove(i);

        let cells = board.get_adjacent_empty(wall.x, wall.y);

        if (cells.len() > 0) && (board.get_adjacent_connections(wall.x, wall.y).len() < 1) {
            let i = rand::thread_rng().gen_range(0..cells.len());
            let cell = cells[i];
            board.make_passage(cell, wall);
            last_wall = wall;
            walls.append(&mut board.get_adjacent_walls(wall.x, wall.y));
        }
    }

    board.set(last_wall.x, last_wall.y, Tile::Exit);
}
