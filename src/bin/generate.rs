//! A tool to generate Sudoku puzzles

extern crate itertools;
extern crate rustdoku;

use itertools::Itertools;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::variants::{empty_disjoint_groups, disjoint_groups_from_clues};
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![20, 21, 22, 23, 24, 29, 33, 38, 42, 47, 51, 56, 57, 58, 59, 60];
    let competition_steps = vec![vec![HiddenSingle, NakedSingle], vec![BoxLine], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)], vec![HiddenSubset(4), NakedSubset(4)], vec![Fish(2)]];

    let empty_grid = empty_disjoint_groups();

    for clues in generator::generate_puzzles_for_grid_with_pattern(empty_grid, pattern) {
        let grid = disjoint_groups_from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{} - {}", steps.iter().rev().join(" "), clues.iter().join(""));
        }
    }
}
