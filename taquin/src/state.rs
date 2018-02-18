//! The state containing the Fifteen puzzle.


use time::PreciseTime;
use std::collections::{HashSet, VecDeque, HashMap};
use rand::XorShiftRng;
use rand::Rng;
use astar::DistNode;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]

/// The struct representing the state of the game
///
pub struct State {
    /// the x position of the empty cell
    pub x: usize,
    /// the y position of the empty cell
    pub y: usize,
    /// the array of cells.
    table: Vec<Vec<usize>>,
    /// the array's size.
    pub size: usize,
}


impl State {
    /// Creates a new state with the given positions.
    pub fn new(x: usize, y: usize, table: Vec<Vec<usize>>, size: usize) -> Self {
        State {
            x: x,
            y: y,
            table: table,
            size: size,
        }
    }
    /// Creates a new state with the given positions.
    pub fn new_random(size: usize) -> Self {
        let mut state = State::new_perfect(size);
        state.shuffle(100);
        state
    }
    /// Creates a new completed state
    pub fn new_perfect(size: usize) -> Self {
        let mut table = vec![];
        let mut val = 0;
        for i in 0..size {
            table.push(vec![]);
            for j in 0..size {
                val += 1;
                table[i].push(val);
                if val == size * size {
                    table[i][j] = 0;
                }
            }
        }
        State {
            x: size - 1,
            y: size - 1,
            table: table,
            size: size,
        }
    }
    /// Search for the given value in the table and returns its coordinates.
    pub fn search(&self, target: usize) -> (usize, usize) {
        for (id_x, i) in (0..self.size).enumerate() {
            for (id_y, j) in (0..self.size).enumerate() {
                if self.table[i][j] == target {
                    return (i, j);
                }
            }
        }
        panic!(format!("error, coult not find : {}", target))
    }
    /// Returns a tuple containing the position of the white cell
    pub fn white_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Checks if the state is complete.
    pub fn is_final(&self, from_start: bool) -> bool {
        let mut val = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if !from_start {
                    val += 1;
                    if val == self.size * self.size {
                        return true;
                    }
                }
                if self.table[i][j] != val {
                    return false;
                }
                if from_start {
                    val += 1;
                }
            }
        }
        return true;
    }
    /// Get all possible moves at the current state.
    pub fn moves(&self) -> Vec<(i32, i32)> {
        let mut moves = vec![];
        if self.x > 0 {
            moves.push((-1, 0));
        }
        if self.x < self.size - 1 {
            moves.push((1, 0));
        }
        if self.y > 0 {
            moves.push((0, -1));
        }
        if self.y < self.size - 1 {
            moves.push((0, 1));
        }
        moves
    }
    /// Takes a position and returns the value in the table at this position.
    pub fn at(&self, (i, j): (usize, usize)) -> Option<usize> {
        match self.table[j][i] {
            0 => None,
            x => Some(x),
        }
    }
    /// Takes a move and checks if it is a legal move.
    pub fn validate(&mut self, mv: (i32, i32)) -> bool {
        self.moves().contains(&mv)
    }
    /// Takes a move and apply it to the state.
    pub fn modify(&mut self, mv: (i32, i32)) {
        let (new_x, new_y) = (
            (self.x as i32 + mv.0) as usize,
            (self.y as i32 + mv.1) as usize,
        );
        self.table[self.x][self.y] = self.table[new_x][new_y];
        self.x = new_x;
        self.y = new_y;
        self.table[self.x][self.y] = 0;
    }
    /// Shuffles the state by playing a certain number of random moves.
    pub fn shuffle(&mut self, times: usize) {
        let mut my_rand = XorShiftRng::new_unseeded();
        for _ in 0..times {
            let mov = self.moves();
            let mv = mov.as_slice();
            self.modify(my_rand.choose(mv).unwrap().clone());
        }
    }
    /// Returns the bottom left square composed of 3x3 grid.
    pub fn get_bottom_left(&self) -> State {
        let mut new_table = vec![];
        for col in self.table[self.size - 3..self.size].iter() {
            new_table.push(col[self.size - 3..self.size].to_vec());
        }
        println!(
            "return : {:?}",
            State::new(
                self.x - self.size + 3,
                self.y - self.size + 3,
                new_table.clone(),
                self.size,
            )
        );
        return State::new(
            self.x - self.size + 3,
            self.y - self.size + 3,
            new_table,
            self.size,
        );
    }
}
impl DistNode<(i32, i32)> for State {
    fn dist_from_end(&self) -> i32 {
        let mut sm = 10;
        for x in 0..self.size {
            for y in 0..self.size {
                if y*self.size+x+1 == self.table[x][y]{
                    sm-=1;
                }
            }
        }
        sm
    }

    fn moves(&self) -> Vec<(i32, i32)> {
        self.moves()
    }

    fn modify(&mut self, mv: &(i32, i32)) {
        self.modify(*mv);
    }
    fn end(&self) -> bool {
        self.is_final(false)
    }
    fn cost_to(&self, target: &Self) -> i32 {
        1
    }
}
