//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::analyser;
use rustdoku::analyser::Criteria;
use rustdoku::generator;
use rustdoku::grid::Grid;
use rustdoku::solver::SolveConfiguration;

fn main() {

    let unsolvable = Criteria::not_solvable_with(SolveConfiguration::with_all_strategies());

    loop {
        let clues = generator::generate_minimal_symmetric_puzzle();
        let grid = Grid::from_clues(&clues).unwrap();
        if analyser::meets_criteria(&grid, &unsolvable) {
            println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
