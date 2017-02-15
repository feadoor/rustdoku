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

            // Rows
            if let Some(row) = Grid::row_containing(&cells) {
                let eliminations = grid.cells_with_candidate_in_region(val, &(row & !block))
                    .map(|ix| Deduction::Elimination(ix, val));

                if !eliminations.is_empty() {
                    return Some(Move { deductions: eliminations });
                }
            }

            // Columns
            if let Some(column) = Grid::column_containing(&cells) {
                let eliminations = grid.cells_with_candidate_in_region(val, &(column & !block))
                    .map(|ix| Deduction::Elimination(ix, val));

                if !eliminations.is_empty() {
                    return Some(Move { deductions: eliminations });
                }
            }
        }
    }

    None
}