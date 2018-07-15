//! Analyse the solution path of a particular Sudoku, and check constraints.

use grid::Grid;
use solver::{solve, SolveConfiguration, SolveResult};
use strategies::Strategy;

/// A group of constraints that must be met by a puzzle
pub struct Criteria {
    solve_configuration: SolveConfiguration,
    constraints: Vec<Constraint>,
}

/// A single constraint that must be met by a puzzle
pub enum Constraint {
    /// The puzzle must be solvable
    Solvable,
    /// The puzzle must be unsolvable
    Unsolvable,
    /// The puzzle cannot be solved without the use of at least one of the given strategies
    Requires(Vec<Strategy>),
}

impl Criteria {

    pub fn with_configuration(configuration: SolveConfiguration) -> Criteria {
        Criteria { solve_configuration: configuration, constraints: Vec::new(), }
    }

    pub fn with(self, constraint: Constraint) -> Criteria {
        let mut criteria = Criteria {
            solve_configuration: self.solve_configuration,
            constraints: self.constraints,
        };
        criteria.constraints.push(constraint);
        criteria
    }

}

pub fn meets_criteria(grid: &Grid, criteria: &Criteria) -> bool {
    let solve_details = solve(&mut grid.clone(), &criteria.solve_configuration);
    for constraint in &criteria.constraints {
        let meets_constraint = match *constraint {
            Constraint::Solvable => solve_details.result == SolveResult::Solved,
            Constraint::Unsolvable => solve_details.result == SolveResult::InsufficientStrategies,
            Constraint::Requires(ref strats) => {
                let configuration = criteria.solve_configuration.without_strategies(&strats);
                solve(&mut grid.clone(), &configuration).result == SolveResult::InsufficientStrategies
            },
        };
        if !meets_constraint { return false; }
    }
    true
}
