//! A tool to generate Sudoku puzzles

extern crate itertools;
extern crate rustdoku;

use itertools::Itertools;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::variants::{empty_classic, classic_from_clues};
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![3, 11, 13, 19, 21, 23, 27, 29, 31, 33, 37, 39, 41, 43, 47, 49, 51, 53, 57, 59, 61, 67, 69, 77];
    let competition_steps = vec![vec![FullHouse, HiddenSingle, NakedSingle], vec![BoxLine], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)], vec![HiddenSubset(4), NakedSubset(4)], vec![Fish(2)]];

    let empty_grid = empty_classic();

    for clues in generator::generate_puzzles_on_empty_grid_with_pattern(empty_grid, pattern) {
        let grid = classic_from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{} - {}", steps.iter().rev().join(" "), clues.iter().join(""));
        }
    }
}
