//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::{CellIdx, Grid};
use strategies::Deduction;

/// Return, if one exists, a deduction based on a hidden subset.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    macro_rules! find_subsets {
        ($d: expr, $x: ident) => {
            let deductions = find_with_degree($x, $d);
            if deductions.is_some() {
                return deductions;
            }
        }
    }

    for degree in 2..5 {
        find_subsets!(degree, grid);
    }

    None
}

/// Find a hidden subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Vec<Deduction>> {

    // Iterate over all regions of the grid and all tuples of values.
    for region in Grid::regions() {

        // Find the values which are missing from the region.
        let mut missing_vals = [true; 9];
        for &cell in region.iter() {
            if let Some(val) = grid.value(cell) {
                missing_vals[val - 1] = false;
            }
        }

        // Iterate over tuples of the missing values.
        for tuple in missing_vals.iter()
            .enumerate()
            .filter(|&(_, &x)| x)
            .map(|(idx, _)| idx + 1)
            .combinations(degree) {

            // Take the collection of cells which contain these candidates.
            let cells = region.iter()
                .filter(|&&ix| tuple.iter().any(|&val| grid.has_candidate(ix, val)))
                .collect::<Vec<_>>();

            // Check if the candidates appear in the right number of cells.
            if cells.len() == degree {
                let deductions = get_deductions(grid, &tuple, &cells);
                if !deductions.is_empty() {
                    return Some(deductions);
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a hidden subset.
fn get_deductions(grid: &Grid, tuple: &[usize], cells: &[&CellIdx]) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Eliminate all other candidates from this group of cells.
    for &&cell in cells {
        let mut candidates = grid.candidates(cell);
        while candidates != 0 {
            let val = candidates.trailing_zeros() as usize + 1;
            if tuple.iter().find(|&&x| x == val).is_none() {
                let deduction = Deduction::Elimination(cell, val);
                deductions.push(deduction);
            }
            candidates &= candidates - 1;
        }
    }

    deductions
}