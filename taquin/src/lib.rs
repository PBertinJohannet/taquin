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

pub mod state;
pub mod reducer;
pub mod astar;
