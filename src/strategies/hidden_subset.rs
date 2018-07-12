//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::Grid;
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};

/// Return, if one exists, a hidden subset of the given degree.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Step> {

    // Iterate over all regions of the grid and all tuples of values.
    for region in Grid::regions() {
        for candidates in grid.missing_values_from_region(region).iter().combinations(degree).map(CandidateSet::from_candidates) {

            // Take the collection of cells which contain these candidates.
            let cells = grid.cells_with_candidates_in_region(&candidates, region);

            // Check if the candidates appear in the right number of cells and if any eliminations will occur.
            if cells.len() == degree {
                if cells.iter().any(|cell| !(grid.candidates(cell) & !candidates).is_empty()) {
                    return Some(Step::HiddenSubset { region: *region, cells: cells, values: candidates });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the hidden subset on the given grid.
pub fn get_deductions(grid: &Grid, hidden_subset: &Step) -> Vec<Deduction> {
    match *hidden_subset {
        Step::HiddenSubset { region: _, cells, values } => _get_deductions(grid, &cells, &values),
        _ => unreachable!(),
    }
}


fn _get_deductions(grid: &Grid, cells: &CellSet, candidates: &CandidateSet) -> Vec<Deduction> {

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