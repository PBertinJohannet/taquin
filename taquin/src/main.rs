//! # taquin
//!
//! Command line utility to solve fifteen puzzle instances.
//!
//! ---
//!
//! Uses BFS.
//!

#![deny(missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications)]

extern crate time;
extern crate rand;
extern crate clap;

mod state;
pub mod reducer;
pub mod astar;

use astar::{DistNode, AStar};
use reducer::Reducer;
use clap::{Arg, App};
use state::State;
use std::num::ParseIntError;

macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}


/// Creates the app, checks for errors and prints the steps.
///
pub fn main() {
    let matches = App::new("fifteen puzzle")
        .version("0.1")
        .author("P Bertin-Johannet")
        .about(
            "slowly solves the given fifteen puzzle game\n\
                Example usage :\
                    $ taquin 2.3.4:7.1.6:0.8.5",
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help("The size of a column")
                .takes_value(true),
        )
        .arg(Arg::with_name("grid").index(1).required(true).takes_value(
            true,
        ))
        .get_matches();

    let size = matches.value_of("size").unwrap_or("3").to_string();

    let grid = matches.value_of("grid").unwrap().to_string();

    match create_state(size, grid) {
        Ok(state) => find(state, false),
        Err(e) => eprintln!("{}", e),
    };
}

/// Creates a state and checks that there are no errors.
/// takes the strings passed as config.
pub fn create_state(s_size: String, s_state: String) -> Result<State, String> {
    let mut size = 3;
    match s_size.parse() {
        Ok(s) => size = s,
        Err(_) => return Err("Please input the size as an integer".to_string()),
    }
    let state = match parse_grid(s_state, size) {
        Ok((grid, pos)) => State::new(pos.0, pos.1, grid, size),
        Err(e) => return Err(e),
    };
    Ok(state)
}

/// Creates a grid/position and checks that there are no errors.
/// takes the strings passed as config.
pub fn parse_grid(
    s_size: String,
    size: usize,
) -> Result<(Vec<Vec<usize>>, (usize, usize)), String> {
    // parse a grid of results.
    let p_grid = s_size
        .split(":")
        .map(|line| {
            line.split(".").map(|e| e.parse()).collect::<Vec<
                Result<
                    usize,
                    ParseIntError,
                >,
            >>()
        })
        .collect::<Vec<_>>();



    // check that only integers were given in the grid.
    let mut grid = vec![];
    let mut zero = None;

    for (id_x, p_line) in p_grid.iter().enumerate() {
        let mut line = vec![];
        for (id_y, j) in p_line.iter().enumerate() {
            match j {
                &Ok(val) => {
                    if val == 0 {
                        zero = Some((id_x, id_y));
                    }
                    line.push(val)
                }
                &Err(_) => {
                    return Err(format!(
                        "error at position : {}, {} cannot parse integer",
                        id_x,
                        id_y
                    ))
                }
            }
        }
        grid.push(line);
    }


    // Checks that the columns/lines have the right number of elements.
    if grid.len() != size {
        return Err("Wrong number of lines".to_string());
    }
    if grid.iter().any(|line| line.len() != size) {
        return Err("Some lines does not have the specified length".to_string());
    }
    if zero.is_none() {
        return Err("Please set the empty cell to zero".to_string());
    }
    Ok((grid, zero.unwrap()))
}


/// Finds the result and prints it.
fn find(base: State, weird: bool) {
    match astar::bfs(base, true) {
        Some(hist) => print_hist(hist),
        None => println!("Sorry\nNo solution could be found"),
    };
}


/// Prints an history of moves in readable format.
pub fn print_hist(hist: Vec<(i32, i32)>) {
    println!("Solution in {} moves", hist.len());
    let mut nb = 0;
    for i in hist {
        nb += 1;
        if i.0 == 1 {
            println!("{} : DOWN", nb);
        } else if i.0 == -1 {
            println!("{} : UP", nb);
        } else if i.1 == 1 {
            println!("{} : RIGHT", nb);
        } else if i.1 == -1 {
            println!("{} : LEFT", nb);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn main() {
        let mut state = State::new_random(4);
        //find(state, false);
        print_hist(astar::AStar::new(state).solve().unwrap());
    }
}
