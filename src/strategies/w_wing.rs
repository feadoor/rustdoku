//! A definition of the W-wing strategy.

use itertools::Itertools;

use grid::{Candidate, CellIdx};
use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, an elimination based on a W-wing.
///
/// A W-wing is a pattern comprising two bivalue cells with the same candidates XY. If there is a
/// region (row, column, block) of the grid such that all occurrences of X in that region are seen
/// by one of the two cells, then one of the cells must contain Y, not X, or else the region would
/// have no place left for the candidate X.
///
/// As a consequence, all common neighbours of the two original cells cannot contain Y.
pub fn find(grid: &Grid) -> Option<Move> {

    // Iterate over pairs of bivalue cells and regions of the grid.
    for (cell1, cell2) in bivalue_pairs(grid) {
        let candidates: Vec<_> = grid.candidates(cell1).iter().collect();
        let (&candidate1, &candidate2) = (candidates.first().unwrap(), candidates.last().unwrap());
        for region in Grid::regions() {
            if region.contains(cell1) || region.contains(cell2) { continue; }
            let unseen_cells = region & !(Grid::neighbours(cell1) | Grid::neighbours(cell2));

            // Check if the cells interact with the region in such a way that eliminations occur.
            if !grid.value_placed_in_region(candidate1, &unseen_cells) && !grid.candidate_in_region(candidate1, &unseen_cells) {
                let deductions = get_deductions(grid, cell1, cell2, candidate2);
                if !deductions.is_empty() {
                    return Some(Move{ deductions: deductions });
                }
            }

            if !grid.value_placed_in_region(candidate2, &unseen_cells) && !grid.candidate_in_region(candidate2, &unseen_cells) {
                let deductions = get_deductions(grid, cell1, cell2, candidate1);
                if !deductions.is_empty() {
                    return Some(Move{ deductions: deductions });
                }
            }
        }
    }

    None
}

/// Returns pairs of bivalue cells which have the same candidates.
fn bivalue_pairs(grid: &Grid) -> Vec<(CellIdx, CellIdx)> {
    grid.cells_with_n_candidates(2).iter().combinations(2)
        .map(|pair| (*pair.first().unwrap(), *pair.last().unwrap()))
        .filter(|&(ix, jx)| grid.candidates(ix) == grid.candidates(jx))
        .collect()
}

/// Gets a list of eliminations which arise from removing the given candidate from all common
/// neighbours of the two given cells.
fn get_deductions(grid: &Grid, cell1: CellIdx, cell2: CellIdx, candidate: Candidate) -> Vec<Deduction> {
    grid.cells_with_candidate_in_region(candidate, &(Grid::neighbours(cell1) & Grid::neighbours(cell2)))
        .map(|cell| Deduction::Elimination(cell, candidate))
}