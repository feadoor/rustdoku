//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::Grid;
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a hidden subset.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find(grid: &Grid) -> Option<Move> {

    macro_rules! find_subsets {
        ($d: expr, $x: ident) => {
            let mov = find_with_degree($x, $d);
            if mov.is_some() {
                return mov;
            }
        }
    }

    for degree in 2..5 {
        find_subsets!(degree, grid);
    }

    None
}

/// Find a hidden subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Move> {

    // Iterate over all regions of the grid and all tuples of values.
    for region in Grid::regions() {
        for candidates in grid.missing_values_from_region(region).iter().combinations(degree).map(CandidateSet::from_candidates) {

            // Take the collection of cells which contain these candidates.
            let cells = grid.all_cells_with_candidates_in_region(&candidates, region);

            // Check if the candidates appear in the right number of cells.
            if cells.len() == degree {
                let deductions = get_deductions(grid, &candidates, &cells);
                if !deductions.is_empty() {
                    return Some(Move { deductions: deductions });
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a hidden subset.
fn get_deductions(grid: &Grid, candidates: &CandidateSet, cells: &CellSet) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Eliminate all other candidates from this group of cells.
    for cell in cells.iter() {
        let eliminations = grid.candidates(cell) & !candidates;
        for candidate in eliminations.iter() {
            let deduction = Deduction::Elimination(cell, candidate);
            deductions.push(deduction);
        }
    }

    deductions
}