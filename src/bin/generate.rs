//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::Grid;
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![0, 1, 3, 4, 9, 10, 12, 13, 27, 28, 36, 37, 43, 44, 52, 53, 67, 68, 70, 71, 76, 77, 79, 80];
    let competition_steps = vec![vec![FullHouse, HiddenSingle, NakedSingle], vec![Pointing, Claiming], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)]];

    for clues in generator::generate_puzzles_with_pattern(pattern.clone()) {
        let grid = Grid::from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{:?} - {}", steps, clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
