//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::analyser;
use rustdoku::analyser::Criteria;
use rustdoku::generator;
use rustdoku::grid::Grid;
use rustdoku::solver::SolveConfiguration;
use rustdoku::strategies::Strategy::*;

fn main() {

    let solvable = Criteria::solvable_with(SolveConfiguration::with_all_strategies());
    let needs_als_forcing_chains = Criteria::not_solvable_with(SolveConfiguration::without_strategies(vec![
        AlsForcingChain,
    ]));

    loop {
        let clues = generator::generate_minimal_symmetric_puzzle();
        let grid = Grid::from_clues(&clues).unwrap();
        if analyser::meets_criteria(&grid, &needs_als_forcing_chains) && analyser::meets_criteria(&grid, &solvable) {
            println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
