//! A definition of the naked subset strategy.

use itertools::Itertools;

use grid::{CellIdx, Grid};
use strategies::Deduction;

/// Return, if one exists, a deduction based on a naked subset.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
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

/// Find a naked subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Vec<Deduction>> {

    // Iterate over all tuples of empty cells from regions of the grid.
    for region in Grid::regions() {
        for tuple in region.iter()
            .filter(|&&ix| grid.is_empty(ix))
            .combinations(degree) {

            // Take the union of the candidates found in these cells.
            let candidates = tuple.iter().fold(0, |acc, &&x| acc | grid.candidates(x));

            // Check if the right number of candidates appear.
            if candidates.count_ones() as usize == degree {
                let deductions = get_deductions(grid, &tuple, candidates);
                if !deductions.is_empty() {
                    return Some(deductions);
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a naked subset.
fn get_deductions(grid: &Grid, tuple: &[&CellIdx], mut candidates: usize) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Get a list of the candidates represented by the candidates mask.
    let mut values = Vec::new();
    while candidates != 0 {
        values.push(candidates.trailing_zeros() as usize + 1);
        candidates &= candidates - 1;
    }

    // If the cells are all in the same row, then eliminate the candidates from other cells in
    // that row.
    if Grid::same_row(tuple) {
        for &cell in Grid::row(*tuple[0]) {
            for &val in &values {
                if grid.has_candidate(cell, val) {
                    if tuple.iter().find(|&&&x| x == cell).is_none() {
                        deductions.push(Deduction::Elimination(cell, val));
                    }
                }
            }
        }
    }

    // If the cells are all in the same column, then eliminate the candidates from other cells in
    // that column.
    if Grid::same_column(tuple) {
        for &cell in Grid::column(*tuple[0]) {
            for &val in &values {
                if grid.has_candidate(cell, val) {
                    if tuple.iter().find(|&&&x| x == cell).is_none() {
                        deductions.push(Deduction::Elimination(cell, val));
                    }
                }
            }
        }
    }

    // If the cells are all in the same block, then eliminate the candidates from other cells in
    // that block.
    if Grid::same_block(tuple) {
        for &cell in Grid::block(*tuple[0]) {
            for &val in &values {
                if grid.has_candidate(cell, val) {
                    if tuple.iter().find(|&&&x| x == cell).is_none() {
                        deductions.push(Deduction::Elimination(cell, val));
                    }
                }
            }
        }
    }

    deductions
}