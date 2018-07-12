//! A definition of the pointing strategy.

use grid::Grid;
use strategies::{Deduction, Step};

/// Return, if one exists, a pointing.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column. Then other occurrences of that value can be removed from other cells in the row
/// or column.
pub fn find(grid: &Grid) -> Option<Step> {

    // Scan each block, and for each value, check if the positions are limited to a row or column.
    for block in Grid::blocks() {
        for &val in Grid::values() {
            let cells = grid.cells_with_candidate_in_region(val, block);
            if cells.len() < 2 { continue; }

            if let Some(intersection) = Grid::row_containing(&cells).or(Grid::column_containing(&cells)) {
                if !grid.cells_with_candidate_in_region(val, &(intersection & !block)).is_empty() {
                    return Some(Step::Pointing { block: *block, region: *intersection, value: val });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the pointing on the given grid.
pub fn get_deductions(grid: &Grid, pointing: &Step) -> Vec<Deduction> {
    match *pointing {
        Step::Pointing { block, region, value } => grid
            .cells_with_candidate_in_region(value, &(region & !block))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}
