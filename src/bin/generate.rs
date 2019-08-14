//! A tool to generate Sudoku puzzles.

extern crate rustdoku;

use rustdoku::analyser;
use rustdoku::analyser::Criteria;
use rustdoku::generator;
use rustdoku::grid::Grid;
use rustdoku::solver::SolveConfiguration;
use rustdoku::strategies::Strategy::*;

fn main() {

    let not_trivial = Criteria::not_solvable_with(SolveConfiguration::with_strategies(vec![
        FullHouse, NakedSingle, HiddenSingle, Pointing, Claiming, HiddenSubset(2), HiddenSubset(3), 
        HiddenSubset(4), NakedSubset(2), NakedSubset(3), NakedSubset(4), Fish(2), Fish(3), Fish(4),
        FinnedFish(2), FinnedFish(3),
    ]));

    let solvable_with_advanced_technique = Criteria::solvable_with(SolveConfiguration::with_strategies(vec![
        FullHouse, NakedSingle, HiddenSingle, FinnedFish(4),
    ]));

    loop {
        let clues = generator::generate_minimal_symmetric_puzzle();
        let grid = Grid::from_clues(&clues).unwrap();
        if analyser::meets_criteria(&grid, &solvable_with_advanced_technique) && analyser::meets_criteria(&grid, &not_trivial) {
            println!("{}", clues.iter().map(|x| x.to_string()).collect::<String>());
        }
    }
}
