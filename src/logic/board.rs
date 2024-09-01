use super::tile::Tile;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
    fn area(&self) -> usize {
        self.x * self.y
    } 
}

pub struct Board {
    game_done: bool,
    player_pos: Coord,
    start_pos: Coord,
    pub size: Coord,
    data: Vec<Tile>
}

// Consolidate this implementation
impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let area: usize = (width * height) as usize;
        Self {
            game_done: false,
            player_pos: Coord::new(0, 0),
            start_pos: Coord::new(0, 0),
            size: Coord::new(width, height),
            data: vec![Tile::wall; area],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Tile {
        match self.data.get(x + y * self.size.x) {
            Some(tile) => tile,
            None => &Tile::wall
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if (x >= self.size.x) || (y >= self.size.y) {
            return false;
        }
        *self.get(x, y) == Tile::wall
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        if (x >= self.size.x) || (y >= self.size.y) {
            return false;
        }
        *self.get(x, y) == Tile::empty
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        match self.data.get_mut(x + y * self.size.x) {
            None => (),
            Some(t) => {
                t.clone_from(&tile);
            }
        }
    }

    pub fn get_adjacent_walls(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut walls = Vec::new();
        if let Some(new_x) = x.checked_add(1) {
            if self.is_wall(new_x, y){
                walls.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(1) {
            if self.is_wall(new_x, y) {
                walls.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(1) {
            if self.is_wall(x, new_y) {
                walls.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(1) {
            if self.is_wall(x, new_y) {
                walls.push(Coord::new(x, new_y));
            }
        }

        walls
    }

    pub fn get_adjacent_empty(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut empties = Vec::new();
        if let Some(new_x) = x.checked_add(1) {
            if self.is_empty(new_x, y){
                empties.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(1) {
            if self.is_empty(new_x, y) {
                empties.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(1) {
            if self.is_empty(x, new_y) {
                empties.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(1) {
            if self.is_empty(x, new_y) {
                empties.push(Coord::new(x, new_y));
            }
        }

        empties
    }

    pub fn get_visited(&self, x: usize, y: usize, visited: &Vec<bool>) -> bool {
        match visited.get(x + y * self.size.x) {
            Some(b) => *b,
            None => false
        }
    }
    
    pub fn get_adjacent_visited(&self, x: usize, y: usize, visited: &Vec<bool>) -> Vec<Coord> {
        let mut tiles = Vec::new();
        if let Some(new_x) = x.checked_add(1) {
            if self.get_visited(new_x, y, visited) {
                tiles.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(1) {
            if self.get_visited(new_x, y, visited) {
                tiles.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(1) {
            if self.get_visited(x, new_y, visited) {
                tiles.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(1) {
            if self.get_visited(x, new_y, visited) {
                tiles.push(Coord::new(x, new_y));
            }
        }
        tiles
    }

    pub fn get_adjacent_unvisited(&self, x: usize, y: usize, visited: &Vec<bool>) -> Vec<Coord> {
        let mut tiles = Vec::new();
        if let Some(new_x) = x.checked_add(1) {
            if new_x < self.size.x && !self.get_visited(new_x, y, visited) {
                tiles.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_x) = x.checked_sub(1) {
            if !self.get_visited(new_x, y, visited) {
                tiles.push(Coord::new(new_x, y));
            }
        }
        if let Some(new_y) = y.checked_add(1) {
            if new_y < self.size.y &&!self.get_visited(x, new_y, visited) {
                tiles.push(Coord::new(x, new_y));
            }
        }
        if let Some(new_y) = y.checked_sub(1) {
            if !self.get_visited(x, new_y, visited) {
                tiles.push(Coord::new(x, new_y));
            }
        }
        tiles
    }

    pub fn move_player_up(&mut self) {
        let new_y = match self.player_pos.y.checked_sub(1) {
            Some(u) => u,
            None => return
        };

        if new_y < self.size.y {
            let new_tile = self.get(self.player_pos.x, new_y).clone();
            if new_tile == Tile::exit {
                self.game_done = true;
            }
            if new_tile != Tile::wall {
                self.player_pos.y = new_y;
            }
        }
    }

    pub fn move_player_down(&mut self) {
        let new_y = match self.player_pos.y.checked_add(1) {
            Some(u) => u,
            None => return
        };

        if new_y < self.size.y {
            let new_tile = self.get(self.player_pos.x, new_y).clone();
            if new_tile == Tile::exit {
                self.game_done = true;
            }
            if new_tile != Tile::wall {
                self.player_pos.y = new_y;
            }
        }
    }

    pub fn move_player_left(&mut self) {
        let new_x = match self.player_pos.x.checked_sub(1) {
            Some(u) => u,
            None => return
        };

        if new_x < self.size.x {
            let new_tile = self.get(new_x, self.player_pos.y).clone();
            if new_tile == Tile::exit {
                self.game_done = true;
            }
            if new_tile != Tile::wall {
                self.player_pos.x = new_x;
            }
        }
    }

    pub fn move_player_right(&mut self) {
        let new_x = match self.player_pos.x.checked_add(1) {
            Some(u) => u,
            None => return
        };

        if new_x < self.size.x {
            let new_tile = self.get(new_x, self.player_pos.y).clone();
            if new_tile == Tile::exit {
                self.game_done = true;
            }
            if new_tile != Tile::wall {
                self.player_pos.x = new_x;
            }
        }
    }

    pub fn pretty_print(&self) -> String {
        let mut output: String = String::new();
        for y in 0..self.size.y {
            let mut row: String = String::new();
            for x in 0..self.size.x {
                match self.get(x, y) {
                    Tile::wall => row.push('█'),
                    Tile::empty => row.push(' '),
                    Tile::exit => row.push('x')
                }
            }
            row.push('\n');
            output.push_str(&row) 
        }
        output
    }

}

pub fn generate_board(board: &mut Board) {
    let mut visited = vec![false; board.size.area()];
    
    board.set(board.start_pos.x, board.start_pos.y, Tile::empty);
    visited[board.start_pos.x + board.start_pos.y * board.size.x] = true;

    let mut walls: Vec<Coord>  = Vec::new();

    walls.append(&mut board.get_adjacent_walls(board.start_pos.x, board.start_pos.y));
    
    println!("Generating Board!");

    // Refien this implementation for nicer mazes
    // Consider a prim's algorythm varient where every other tile is considered a valid cell
    // and the in between tiles are passage ways
    while walls.len() > 0 {
        let i = rand::thread_rng().gen_range(0..walls.len());
        let wall: Coord = walls.remove(i);

        println!("{} walls remain", walls.len());

        if board.get_adjacent_visited(wall.x, wall.y, &visited).len() == 1 {
            let unvisited = board.get_adjacent_unvisited(wall.x, wall.y, &visited);
            let i = rand::thread_rng().gen_range(0..unvisited.len());
            let cell = unvisited[i].clone();
            board.set(wall.x, wall.y, Tile::empty);
            visited[wall.x + wall.y * board.size.x] = true;
            board.set(cell.x, cell.y, Tile::empty);
            visited[cell.x + cell.y * board.size.x] = true;
            walls.append(&mut board.get_adjacent_walls(cell.x, cell.y));
        }
    }
}