//! A definition of the W-wing strategy.

use itertools::Itertools;

use grid::CellIdx;
use grid::{Grid, GridSize};
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the W-wings that exist in the grid.
///
/// A W-wing is a pattern comprising two bivalue cells with the same candidates XY. If there is a
/// region (row, column, block) of the grid such that all occurrences of X in that region are seen
/// by one of the two cells, then one of the cells must contain Y, not X, or else the region would
/// have no place left for the candidate X.
///
/// As a consequence, all common neighbours of the two original cells cannot contain Y.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // Iterate over pairs of bivalue cells with the same candidates.
        for (cell1, cell2) in bivalue_pairs(grid) {
            let common_neighbours = grid.neighbours(cell1) & grid.neighbours(cell2);
            let candidates: Vec<_> = grid.candidates(cell1).iter().collect();
            let (&candidate1, &candidate2) = (candidates.first().unwrap(), candidates.last().unwrap());

            // Iterate over regions of the grid, checking for a W-wing.
            for region in grid.all_regions() {
                if region.contains(cell1) || region.contains(cell2) { continue; }
                let unseen_cells = region & !(grid.neighbours(cell1) | grid.neighbours(cell2));

                // Check if the cells interact with the region in such a way that eliminations occur.
                if !grid.value_placed_in_region(candidate1, &unseen_cells) && !grid.candidate_in_region(candidate1, &unseen_cells) {
                    if !grid.cells_with_candidate_in_region(candidate2, &common_neighbours).is_empty() {
                        yield Step::WWing { pincer1: cell1, pincer2: cell2, region: region.clone(), covered_value: candidate1, eliminated_value: candidate2 };
                    }
                }

                if !grid.value_placed_in_region(candidate2, &unseen_cells) && !grid.candidate_in_region(candidate2, &unseen_cells) {
                    if !grid.cells_with_candidate_in_region(candidate1, &common_neighbours).is_empty() {
                        yield Step::WWing { pincer1: cell1, pincer2: cell2, region: region.clone(), covered_value: candidate2, eliminated_value: candidate1 };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the W-wing on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, w_wing: &Step<T>) -> Vec<Deduction> {
    match w_wing {
        Step::WWing { pincer1, pincer2, eliminated_value, .. } => grid
            .cells_with_candidate_in_region(*eliminated_value, &(grid.neighbours(*pincer1) & grid.neighbours(*pincer2)))
            .map(|cell| Deduction::Elimination(cell, *eliminated_value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, w_wing: &Step<T>) -> String {
    match w_wing {
        Step::WWing { pincer1, pincer2, region, covered_value, eliminated_value } => format!(
            "W-Wing - pincers ({}, {}) cover {} in {}, and so eliminate {} from common neighbours",
            grid.cell_name(*pincer1), grid.cell_name(*pincer2), covered_value, grid.region_name(&region), eliminated_value,
        ),
        _ => unreachable!(),
    }
}

/// Returns pairs of bivalue cells which have the same candidates.
fn bivalue_pairs<T: GridSize>(grid: &Grid<T>) -> Vec<(CellIdx, CellIdx)> {
    grid.cells_with_n_candidates(2).iter().combinations(2)
        .map(|pair| (*pair.first().unwrap(), *pair.last().unwrap()))
        .filter(|&(ix, jx)| grid.candidates(ix) == grid.candidates(jx))
        .collect()
}
