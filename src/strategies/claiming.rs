//! A definition of the claiming strategy.

use grid::{Grid, LARGE_SIZE};
use strategies::Deduction;

/// Return, if one exists, a deduction based on claiming.
///
/// Pointing occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each row / column, and for each value, check if the positions are limited to a
    // particular block.

    // Rows
    for row in grid.rows() {
        for value in 1..LARGE_SIZE + 1 {
            let cells = row.potential_cells(value);

            // Check if the cells all appear in a single block.
            if cells.len() >= 2 && cells.iter().all(|x| x.block() == cells[0].block()) {
                let eliminations: Vec<Deduction> = grid.block_from_cell(cells[0].idx())
                    .cells()
                    .iter()
                    .filter(|c| c.has_candidate(value) && c.row() != cells[0].row())
                    .map(|c| Deduction::Elimination(c.idx(), value))
                    .collect();
                if eliminations.len() > 0 {
                    return Some(eliminations);
                }
            }
        }
    }

    // Columns
    for column in grid.columns() {
        for value in 1..LARGE_SIZE + 1 {
            let cells = column.potential_cells(value);

            // Check if the cells all appear in a single block.
            if cells.len() >= 2 && cells.iter().all(|x| x.block() == cells[0].block()) {
                let eliminations: Vec<Deduction> = grid.block_from_cell(cells[0].idx())
                    .cells()
                    .iter()
                    .filter(|c| c.has_candidate(value) && c.column() != cells[0].column())
                    .map(|c| Deduction::Elimination(c.idx(), value))
                    .collect();
                if eliminations.len() > 0 {
                    return Some(eliminations);
                }
            }
        }
    }

    None
}