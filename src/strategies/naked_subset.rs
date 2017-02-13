//! A definition of the naked subset strategy.

use itertools::Itertools;

use grid::{CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a naked subset.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
pub fn find(grid: &Grid) -> Option<Move> {

    macro_rules! find_subsets {
        ($d: expr, $x: ident) => {
            let mov = find_with_degree($x, $d);
            if mov.is_some() {
                return mov;
            }
        }
    }

    find_subsets!(2, grid);
    find_subsets!(3, grid);
    find_subsets!(4, grid);

    None
}

/// Find a naked subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Move> {

    // Iterate over all tuples of empty cells from regions of the grid.
    for region in Grid::regions() {
        for tuple in region.iter()
            .filter(|&ix| grid.is_empty(ix))
            .combinations(degree)
        {
            // Take the union of the candidates found in these cells.
            let candidates = tuple.iter().fold(0, |acc, &x| acc | grid.candidates(x));

            // Check if the right number of candidates appear.
            if candidates.count_ones() as usize == degree {
                let deductions = get_deductions(grid, &tuple, candidates);
                if !deductions.is_empty() {
                    return Some(Move { deductions: deductions });
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a naked subset.
fn get_deductions(grid: &Grid, tuple: &[CellIdx], mut candidates: usize) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Get a list of the candidates represented by the candidates mask.
    let mut values = Vec::new();
    while candidates != 0 {
        values.push(candidates.trailing_zeros() as usize + 1);
        candidates &= candidates - 1;
    }

    // Eliminate candidates that can see all the cells in the tuple.
    let common_cells = tuple.iter()
        .map(|&ix| Grid::neighbours(ix))
        .fold(!CellSet::empty(), |acc, x| acc & x);

    for cell in common_cells.iter() {
        for &val in &values {
            if grid.has_candidate(cell, val) {
                deductions.push(Deduction::Elimination(cell, val));
            }
        }
    }

    deductions
}