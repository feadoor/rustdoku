//! Analyse the solution path of a particular Sudoku, and check constraints.

use grid::{Grid, GridSize};
use solver;
use solver::{SolveConfiguration, SolveDetails};
use solver::SolveResult::Solved;

/// A group of constraints that must be met by a puzzle
pub struct Criteria<'a, T: GridSize> {
    configuration: SolveConfiguration,
    constraints: Vec<&'a dyn Fn(&SolveDetails<T>) -> bool>,
}

impl <'a, T: GridSize> Criteria<'a, T> {

    pub fn solvable_with(configuration: SolveConfiguration) -> Criteria<'a, T> {
        Criteria { configuration, constraints: vec![&solvable], }
    }

    pub fn not_solvable_with(configuration: SolveConfiguration) -> Criteria<'a, T> {
        Criteria { configuration, constraints: vec![&unsolvable], }
    }
}

pub fn meets_criteria<T: GridSize>(grid: &Grid<T>, criteria: &Criteria<T>) -> bool {
    let solve_details = solver::solve(&mut grid.clone(), &criteria.configuration);
    criteria.constraints.iter().all(|con| con(&solve_details))
}

fn solvable<T: GridSize>(solve_details: &SolveDetails<T>) -> bool {
    solve_details.result == Solved
}

fn unsolvable<T: GridSize>(solve_details: &SolveDetails<T>) -> bool {
    solve_details.result != Solved
}