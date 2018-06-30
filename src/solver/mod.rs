//! A logical solver for Sudoku

use grid::Grid;

use strategies;
use strategies::Deduction::*;
use strategies::Strategy::*;
use strategies::Move;

#[derive(PartialEq, Eq, Debug)]
pub enum SolveResult {
    Solved,
    Contradiction,
    InsufficientStrategies,
}

pub fn solve(grid: &mut Grid) -> SolveResult {
    while !grid.is_solved() {
        if let Some(mov) = find_move(grid) {
            for deduction in mov.deductions {
                if let Contradiction = deduction {
                    return SolveResult::Contradiction;
                } else {
                    grid.apply_deduction(deduction);
                }
            }
        } else {
            return SolveResult::InsufficientStrategies;
        }
    }

    SolveResult::Solved
}

fn find_move(grid: &Grid) -> Option<Move> {

    macro_rules! search {
        ($strat: expr) => {
            let mov = strategies::find_move(grid, $strat);
            if mov.is_some() {
                return mov;
            }
        }
    }

    search!(FullHouse);
    search!(HiddenSingle);
    search!(NakedSingle);
    search!(Pointing);
    search!(Claiming);
    search!(NakedSubset(2));
    search!(HiddenSubset(2));
    search!(NakedSubset(3));
    search!(HiddenSubset(3));
    search!(NakedSubset(4));
    search!(HiddenSubset(4));
    search!(Fish(2));
    search!(Fish(3));
    search!(FinnedFish(2));
    search!(FinnedFish(3));
    search!(Fish(4));
    search!(FinnedFish(4));
    search!(XYWing);
    search!(XYZWing);
    search!(WWing);

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
                assert_eq!(solve(&mut grid), SolveResult::Solved);
                check_grid(&grid);
            }
        }
    }
}