//! Gameboard controller.

use piston::input::GenericEvent;
use taquin::state::State;
use taquin::reducer::Reducer;
use std::collections::VecDeque;
use time::PreciseTime;


/// Handles events for Fifteen puzzle game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: State,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Position of the cursor
    cursor_pos: [f64; 2],
    /// Tells if the game needs to solve.
    solving: bool,
    /// Time since last move was made.
    time_since_last_move: f64,
    /// A vector containing the moves needed to finish the game.
    calculated_moves: VecDeque<(i32, i32)>,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(state: State) -> GameboardController {
        GameboardController {
            gameboard: state,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            solving: false,
            time_since_last_move: 0.0,
            calculated_moves: VecDeque::new(),
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(idle) = e.idle_args() {
            self.time_since_last_move += idle.dt;
        }
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= size && y >= 0.0 && y <= size {
                // Compute the cell position.
                let cell_x = (x / size * ::SIZE.1 as f64) as usize;
                let cell_y = (y / size * ::SIZE.0 as f64) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        let mut mv = match e.press_args() {
            Some(Button::Keyboard(key)) => {
                match key {
                    Key::Down => (1, 0),
                    Key::Up => (-1, 0),
                    Key::Left => (0, -1),
                    Key::Right => (0, 1),
                    Key::Space => {
                        self.solve();
                        self.solving = !self.solving;
                        (0, 0)
                    }
                    Key::S => {
                        self.gameboard.shuffle(5000);
                        (0, 0)
                    }
                    _ => (0, 0),
                }
            }
            _ => (0, 0),
        };
        if self.solving {
            mv = self.get_next_solved();
            if mv == (0,0) {
                self.solving != self.solving;
            }
        }
        if self.gameboard.validate(mv) {
            self.gameboard.modify(mv);
        }
    }
    fn get_next_solved(&mut self) -> (i32, i32) {
        if self.time_since_last_move < 0.0 {
            return (0, 0);
        } else {
            self.time_since_last_move -= 0.0;
            self.calculated_moves.pop_front().unwrap_or((0, 0))
        }
    }
    fn solve(&mut self) {
        let start = PreciseTime::now();
        self.calculated_moves = Reducer::new(self.gameboard.clone()).reduce().unwrap_or(VecDeque::new());

        let end = PreciseTime::now();
        let diff = start.to(end).num_microseconds();
        println!("Found solution of {:?} moves in {:?} us", self.calculated_moves.len(),diff.unwrap());
    }
}
