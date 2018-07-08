//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::generator;
use rustdoku::grid::Grid;

fn main() {
    let cells = vec![0; 81];
    let solution = generator::brute_force::get_random_solution(&cells).unwrap();
    println!("{}", Grid::from_clues(&solution).unwrap());
}
