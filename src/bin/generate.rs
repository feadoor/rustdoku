//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::generator;

fn main() {
    let clues = generator::generate_puzzle();
    println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
}
