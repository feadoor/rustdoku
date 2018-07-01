//! A definition of the pointing strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on pointing.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column. Then other occurrences of that value can be removed from other cells in the row
/// or column.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each block, and for each value, check if the positions are limited to a row or column.
    for block in Grid::blocks() {
        for &val in Grid::values() {
            let cells = grid.cells_with_candidate_in_region(val, block);
            if cells.len() < 2 { continue; }

            let opt_int = Grid::row_containing(&cells).or(Grid::column_containing(&cells));
            if opt_int.is_some() {
                let intersection = opt_int.unwrap();
                let eliminations = grid.cells_with_candidate_in_region(val, &(intersection & !block))
                    .map(|ix| Deduction::Elimination(ix, val));

                if !eliminations.is_empty() {
                    return Some(Move {
                        deductions: eliminations,
                        description: format!(
                            "Pointing {}s in {} eliminate further {}s in {}",
                            val, Grid::region_name(block), val, Grid::region_name(intersection),
                        )
                    });
                }
            }
        }
    }

    None
}