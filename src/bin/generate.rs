//! A tool to generate Sudoku puzzles

extern crate rustdoku;

use rustdoku::grid::variants::empty_classic;
use rustdoku::generator;

fn main() {

    let pattern = vec![2, 3, 4, 11, 12, 13, 25, 26, 34, 35, 36, 37, 43, 44, 45, 46, 54, 55, 67, 68, 69, 76, 77, 78];
    let empty_classic_grid = empty_classic();

    for clues in generator::generate_puzzles_for_grid_with_pattern(empty_classic_grid, pattern) {
        println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
    }
}
