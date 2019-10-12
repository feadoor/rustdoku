//! A tool to generate Sudoku puzzles

extern crate rustdoku;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::variants::{empty_diagonal, diagonal_from_clues};
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![2, 3, 4, 12, 13, 25, 34, 35, 36, 37, 43, 44, 45, 46, 55, 67, 68, 76, 77, 78];
    let competition_steps = vec![vec![FullHouse, HiddenSingle, NakedSingle], vec![BoxLine], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)]];
    let empty_classic_grid = empty_diagonal();

    for clues in generator::generate_puzzles_for_grid_with_pattern(empty_classic_grid, pattern) {
        let grid = diagonal_from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{} {} {} {} - {}", steps[3], steps[2], steps[1], steps[0], clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
