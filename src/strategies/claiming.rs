//! A definition of the claiming strategy.

use grid::Grid;
use strategies::{Deduction, Step};

/// Return, if one exists, a claiming.
///
/// Claiming occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find(grid: &Grid) -> Option<Step> {

    // Scan each row / column, and for each value, check if the positions are limited to a
    // particular block.
    for region in Grid::rows().iter().chain(Grid::columns().iter()) {
        for &val in Grid::values() {
            let cells = grid.cells_with_candidate_in_region(val, region);
            if cells.len() < 2 { continue; }

            if let Some(block) = Grid::block_containing(&cells) {
                if !grid.cells_with_candidate_in_region(val, &(block & !region)).is_empty() {
                    return Some(Step::Claiming { region: *region, block: *block, value: val });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the claiming on the given grid.
pub fn get_deductions(grid: &Grid, claiming: &Step) -> Vec<Deduction> {
    match *claiming {
        Step::Claiming { region, block, value } => grid
            .cells_with_candidate_in_region(value, &(block & !region))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}
