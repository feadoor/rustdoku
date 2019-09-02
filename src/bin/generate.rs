//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::generator;

fn main() {

    let pattern = vec![8, 11, 14, 15, 18, 22, 25, 28, 30, 32, 35, 38, 40, 42, 45, 48, 50, 52, 55, 58, 62, 65, 66, 69, 72];
    for clues in generator::generate_puzzles_with_pattern(pattern.clone()) {
        println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
    }
}
