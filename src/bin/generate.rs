//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::analyser;
use rustdoku::analyser::{Criteria, Constraint};
use rustdoku::generator;
use rustdoku::grid::Grid;
use rustdoku::solver::SolveConfiguration;

fn main() {
    let criteria = Criteria::with_configuration(SolveConfiguration::with_all_strategies())
        .with(Constraint::Unsolvable);

    loop {
        let clues = generator::generate_minimal_symmetric_puzzle();
        let grid = Grid::from_clues(&clues).unwrap();
        if analyser::meets_criteria(&grid, &criteria) {
            println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
