//! A definition of the claiming strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on claiming.
///
/// Pointing occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each row / column, and for each value, check if the positions are limited to a
    // particular block.
    for region in Grid::rows().iter().chain(Grid::columns().iter()) {
        for &val in Grid::values() {
            let cells = grid.cells_with_candidate_in_region(val, region);
            if cells.len() < 2 { continue; }

            if let Some(block) = Grid::block_containing(&cells) {
                let eliminations = grid.cells_with_candidate_in_region(val, &(block & !region))
                    .map(|ix| Deduction::Elimination(ix, val));

                if !eliminations.is_empty() {
                    return Some(Move {
                        deductions: eliminations,
                        description: format!(
                            "Claiming {}s in {} eliminate further {}s in {}",
                            val, Grid::region_name(region), val, Grid::region_name(block),
                        )
                    });
                }
            }
        }
    }

    None
}