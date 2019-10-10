//! A logical solver for Sudoku

mod solve_configuration;

use grid::{Grid, GridSize};

use strategies::{Step, Deduction};
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
pub struct SolveDetails<T: GridSize> {
    /// The result of the solve.
    pub result: SolveResult,
    /// The path taken through the solve.
    pub steps: Vec<Step<T>>,
}

/// Solve, as far as possible, the grid, using the allowed strategies.
pub fn solve<T: GridSize>(grid: &mut Grid<T>, config: &SolveConfiguration) -> SolveDetails<T> {
    let mut steps = Vec::new();
    while !grid.is_solved() {
        if let Some((step, deductions)) = find_step(grid, config) {
            for deduction in deductions {
                if let Contradiction = deduction {
                    return SolveDetails { result: SolveResult::Contradiction, steps };
                } else {
                    grid.apply_deduction(deduction);
                }
            }
            steps.push(step);
        } else {
            return SolveDetails { result: SolveResult::InsufficientStrategies, steps };
        }
    }

    SolveDetails { result: SolveResult::Solved, steps }
}

/// Find the next step using the allowed set of strategies.
fn find_step<T: GridSize>(grid: &Grid<T>, config: &SolveConfiguration) -> Option<(Step<T>, Vec<Deduction>)> {

    for &strategy in config.strategies() {
        for step in strategy.find_steps(&grid) {
            let deductions = step.get_deductions(grid);
            if deductions.len() > 0 { return Some((step, deductions)); }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use crate::define_grid_size;
    use grid::cellset::CellSet;

    use super::*;

    define_grid_size!(Grid9, 9);

    fn parse_grid(string: &str) -> Grid<Grid9> {

        let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

        let clues: Vec<usize> = string.bytes().map(|byte| match byte {
            b'1'..=b'9' => (byte - b'0') as usize,
            _ => 0,
        }).collect();

        let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 81]);
        for (idx, clue) in clues.iter().enumerate() {
            if *clue > 0 { grid.place_value(idx, *clue); }
        }

        grid
    }

    fn check_grid<T: GridSize>(grid: &Grid<T>) {
        // Check that each value appears in every region in the grid.
        for region in grid.all_regions() {
            for value in grid.values() {
                assert!(region.iter().any(|x| grid.value(x) == Some(value)));
            }
        }
    }

    #[test]
    fn test_classic_solves() {
        let file = File::open(&Path::new("classic_grids.txt")).unwrap();
        let reader = BufReader::new(file);
        for line_it in reader.lines() {
            let line = line_it.unwrap();
            if !line.is_empty() && !line.starts_with("//") {                
                let mut grid = parse_grid(&line);
                assert_eq!(
                    solve(&mut grid, &SolveConfiguration::with_all_strategies()).result,
                    SolveResult::Solved
                );
                check_grid(&grid);
            }
        }
    }
}
