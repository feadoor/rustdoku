//! Analyse the solution path of a particular Sudoku, and check constraints.

use grid::Grid;
use solver;
use solver::{SolveConfiguration, SolveDetails};
use solver::SolveResult::Solved;

/// A group of constraints that must be met by a puzzle
pub struct Criteria<'a> {
    configuration: SolveConfiguration,
    constraints: Vec<&'a Fn(&SolveDetails) -> bool>,
}

impl <'a> Criteria<'a> {

    pub fn solvable_with(configuration: SolveConfiguration) -> Criteria<'a> {
        Criteria { configuration, constraints: vec![&solvable], }
    }

    pub fn not_solvable_with(configuration: SolveConfiguration) -> Criteria<'a> {
        Criteria { configuration, constraints: vec![&unsolvable], }
    }
}

pub fn meets_criteria(grid: &Grid, criteria: &Criteria) -> bool {
    let solve_details = solver::solve(&mut grid.clone(), &criteria.configuration);
    criteria.constraints.iter().all(|con| con(&solve_details))
}

fn solvable(solve_details: &SolveDetails) -> bool {
    solve_details.result == Solved
}

fn unsolvable(solve_details: &SolveDetails) -> bool {
    solve_details.result != Solved
}