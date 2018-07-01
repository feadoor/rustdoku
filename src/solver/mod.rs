//! A logical solver for Sudoku

mod solve_configuration;

use grid::Grid;

use strategies;
use strategies::AnnotatedMove;
use strategies::Deduction::*;

pub use self::solve_configuration::SolveConfiguration;

/// Represents the result of an attempted solve.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SolveResult {
    Solved,
    Contradiction,
    InsufficientStrategies,
}

/// Stores details about the path taken during a solve.
pub struct SolveDetails {
    /// The result of the solve.
    pub result: SolveResult,
    /// The path taken through the solve.
    pub moves: Vec<AnnotatedMove>,
}

/// Solve, as far as possible, the grid, using the allowed strategies.
pub fn solve(grid: &mut Grid, config: &SolveConfiguration) -> SolveDetails {
    let mut moves = Vec::new();
    while !grid.is_solved() {
        if let Some(mov) = find_move(grid, config) {
            moves.push(mov.clone());
            for deduction in mov.mov.deductions {
                if let Contradiction = deduction {
                    return SolveDetails { result: SolveResult::Contradiction, moves };
                } else {
                    grid.apply_deduction(deduction);
                }
            }
        } else {
            return SolveDetails { result: SolveResult::InsufficientStrategies, moves };
        }
    }

    SolveDetails { result: SolveResult::Solved, moves }
}

/// Find the next move using the allowed set of strategies.
fn find_move(grid: &Grid, config: &SolveConfiguration) -> Option<AnnotatedMove> {

    for &strategy in config.strategies() {
        let mov = strategies::find_move(grid, strategy);
        if mov.is_some() {
            return mov;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use super::*;

    fn check_grid(grid: &Grid) {
        // Check that each value appears in every region in the grid.
        for region in Grid::regions() {
            for &value in Grid::values() {
                assert!(region.iter().any(|x| grid.value(x) == Some(value)));
            }
        }
    }

    #[test]
    fn test_solves() {
        let file = File::open(&Path::new("grids.txt")).unwrap();
        let reader = BufReader::new(file);
        for line_it in reader.lines() {
            let line = line_it.unwrap();
            if !line.is_empty() && !line.starts_with("//") {
                let mut grid = Grid::from_str(&line).unwrap();
                assert_eq!(
                    solve(&mut grid, &SolveConfiguration::with_all_strategies()).result,
                    SolveResult::Solved
                );
                check_grid(&grid);
            }
        }
    }
}
