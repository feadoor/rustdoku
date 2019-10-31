//! A tool to generate Sudoku puzzles

extern crate itertools;
extern crate rustdoku;

use itertools::Itertools;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::variants::{empty_untouch, untouch_from_clues};
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![10, 11, 12, 13, 14, 15, 16, 19, 25, 28, 34, 37, 43, 46, 52, 55, 61, 64, 65, 66, 67, 68, 69, 70];
    let competition_steps = vec![vec![FullHouse, HiddenSingle, NakedSingle], vec![BoxLine], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)], vec![HiddenSubset(4), NakedSubset(4)], vec![Fish(2)]];

    let empty_grid = empty_untouch();

    for clues in generator::generate_puzzles_for_grid_with_pattern(empty_grid, pattern) {
        let grid = untouch_from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{} - {}", steps.iter().rev().join(" "), clues.iter().join(""));
        }
    }
}
