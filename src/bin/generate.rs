//! A tool to generate Sudoku puzzles

extern crate itertools;
extern crate rustdoku;

use itertools::Itertools;

use std::io::{stdin, BufRead};
use std::collections::HashMap;

use rustdoku::analyser::steps_to_solve;
use rustdoku::grid::variants::{empty_nonconsecutive, nonconsecutive_from_clues, nonconsecutive_from_string};
use rustdoku::generator;
use rustdoku::strategies::Strategy::*;

fn main() {

    let pattern = vec![10, 13, 23, 33, 37, 43, 47, 57, 67, 70];
    let competition_steps = vec![vec![HiddenSingle, NakedSingle], vec![BoxLine, CellInteraction], vec![HiddenSubset(2), NakedSubset(2)], vec![HiddenSubset(3), NakedSubset(3)], vec![HiddenSubset(4), NakedSubset(4)], vec![Fish(2)]];

    let empty_grid = empty_nonconsecutive();

    for clues in generator::generate_puzzles_for_grid_with_pattern(empty_grid, pattern) {
        eprintln!("{}", clues.iter().join(""));
        let grid = nonconsecutive_from_clues(&clues).unwrap();
        if let Some(steps) = steps_to_solve(&grid, &competition_steps) {
            println!("{} - {}", steps.iter().rev().join(" "), clues.iter().join(""));
        }
    }
}
