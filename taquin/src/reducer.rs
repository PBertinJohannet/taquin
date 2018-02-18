//! The Reducer module : Reduces the size of the grid by filling columns and rows.
use std::collections::{HashSet, VecDeque};
use astar::bfs;
use state::State;
#[derive(Debug)]
/// The reducer struct, contains the informations nececary to perform the algorithm.
pub struct Reducer {
    grid: State,
    availables: Vec<Vec<bool>>,
    curr_x: usize,
    curr_y: usize,
    moves: VecDeque<(i32, i32)>,
}
impl Reducer {
    /// Creates a new reducer from a state.
    pub fn new(state: State) -> Self {
        let size = state.size;
        Reducer {
            grid: state,
            availables: (0..size)
                .map(|_| (0..size).map(|_| true).collect())
                .collect(),
            curr_x: 0,
            curr_y: 0,
            moves: VecDeque::new(),
        }
    }
    /// Reduces the grid and returns the moves needed to do so.
    pub fn reduce(&mut self) -> Option<VecDeque<(i32, i32)>> {
        // rows down to n -2
        let size = self.grid.size;
        for i in 0..size - 3 {
            self.reduce_col(i);
            println!("finished col : {}", i);
            self.reduce_row(i);
        }
        self.reduce_row(size - 3);
        self.moves.append(
            &mut bfs(self.grid.clone(), true)
                .unwrap_or(vec![(0, 0)])
                .iter()
                .map(|x| *x)
                .collect(),
        );
        Some(self.moves.clone())
    }
    /// Reduces one row by completing it.
    pub fn reduce_row(&mut self, row: usize) {
        // rows down to n -2
        let size = self.grid.size;
        for i in 0..size - 1 {
            self.bring_cell((row, i), row * size + 1 + i);
            self.availables[row][i] = false;
        }
        // then for the last cell :
        let last = row * size + size;
        if self.grid.search(last) == (row + 1, size - 1) &&
            self.grid.white_pos() == (row, size - 1)
        {
            self.forward(last);
        } else if self.grid.search(last) != (row, size - 1) {
            self.bring_cell((row + 1, size - 1), last);
            self.bring_white((row + 1, size - 3), (row + 1, size - 1));
            // brings the white 2 up without moving this one
            // then : left down down right up left up right
            let moves = vec![
                (-1, 0),
                (0, 1),
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
                (0, -1),
                (1, 0),
            ];
            for m in moves {
                self.moves.push_back(m);
                self.grid.modify(m);
            }
        }
        self.availables[row][size - 1] = false;
    }
    /// Reduces one column by completing it.
    pub fn reduce_col(&mut self, col: usize) {
        // rows down to n -2
        let size = self.grid.size;
        for i in 0..size - 1 {
            self.bring_cell((i, col), i * size + 1 + col);
            self.availables[i][col] = false;
        }
        // then for the last cell :
        let last = size * size - size + 1 + col;
        if self.grid.search(last) == (size - 1, col + 1) &&
            self.grid.white_pos() == (size - 1, col)
        {
            self.forward(last);
        } else if self.grid.search(last) != (size - 1, col) {
            self.bring_cell((size - 1, col + 1), last);
            self.bring_white((size - 3, col + 1), (size - 1, col + 1));
            // brings the white 2 up without moving this one
            // then : left down down right up left up right
            let moves = vec![
                (0, -1),
                (1, 0),
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (-1, 0),
                (0, 1),
            ];
            for m in moves {
                self.moves.push_back(m);
                self.grid.modify(m);
            }
        }
        self.availables[size - 1][col] = false;
    }
    /// Brings the cell with value "value" to the desired position
    pub fn bring_cell(&mut self, target_pos: (usize, usize), value: usize) {
        let target_val = self.grid.search(value);
        let path = self.path_find(target_val, target_pos, target_val);
        // for each step :
        for i in path {
            let target = self.grid.search(value);
            let next_cell = (
                (target.0 as i32 + i.0) as usize,
                (target.1 as i32 + i.1) as usize,
            );
            // brings the white to the step and avoid the cell.
            self.bring_white(next_cell, target);
            self.forward(value);
        }
    }
    /// Brings the white cell with  to the desired position
    pub fn bring_white(&mut self, next_cell: (usize, usize), avoid: (usize, usize)) {
        let white_to_next = self.path_find(self.grid.white_pos(), next_cell, avoid);
        for mv in white_to_next {
            self.moves.push_back(mv);
            self.grid.modify(mv);
        }
    }

    /// Finds a path for a cell to a specific point.
    pub fn path_find(
        &self,
        from: (usize, usize),
        target: (usize, usize),
        avoid: (usize, usize),
    ) -> Vec<(i32, i32)> {
        let mut visited = HashSet::new();
        let mut f = VecDeque::new();
        f.push_back((from.clone(), vec![]));
        while let Some((nouv, history)) = f.pop_front() {
            if nouv == target {
                return history;
            } else {
                visited.insert(nouv.clone());

                for i in self.get_legal_moves(nouv, avoid) {
                    let mut cop = (
                        (nouv.0 as i32 + i.0) as usize,
                        (nouv.1 as i32 + i.1) as usize,
                    );
                    if !visited.contains(&cop) {
                        let mut new_hist = history.clone();
                        new_hist.push(i);
                        f.push_back((cop, new_hist));
                    }
                }
            }
        }
        println!("fail : {:?}", self.grid);
        panic!("found no way from {:?} to {:?}", from, target);
    }
    /// swap the "target" cell with the empty cell.
    pub fn forward(&mut self, target_val: usize) {
        let empty = self.grid.white_pos();
        let target = self.grid.search(target_val);
        let mut mv = (0, 0);
        if empty.0 > target.0 {
            mv = (-1, 0);
        } else if empty.0 < target.0 {
            mv = (1, 0);
        } else if empty.1 < target.1 {
            mv = (0, 1);
        } else if empty.1 > target.1 {
            mv = (0, -1);
        }
        self.moves.push_back(mv);
        self.grid.modify(mv);
    }
    /// Checks to see if a move is legal.
    pub fn get_legal_moves(&self, pos: (usize, usize), avoid: (usize, usize)) -> Vec<(i32, i32)> {
        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .filter(|&&(x, y)| {
                self.legal((pos.0 + x as usize, pos.1 + y as usize), avoid)
            })
            .map(|&x| x)
            .collect()
    }
    /// Checks to see if a move is legal.
    pub fn legal(&self, target: (usize, usize), avoid: (usize, usize)) -> bool {
        target.0 >= 0 && target.0 < self.grid.size && target.1 >= 0 &&
            target.1 < self.grid.size && self.availables[target.0][target.1] &&
            avoid != target
    }
}
