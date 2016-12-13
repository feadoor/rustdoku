//! A definition of the pointing strategy.

use grid::{Grid, LARGE_SIZE};
use strategies::Deduction;

/// Return, if one exists, a deduction based on pointing.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each block, and for each value, check if the positions are limited to a row or column.
    for block in grid.blocks() {
        for value in 1..LARGE_SIZE + 1 {
            let cells = block.potential_cells(value);

            // Check if the cells all appear in a single row or column.
            if cells.len() >= 2 {

                // Rows
                if cells.iter().all(|x| x.row() == cells[0].row()) {
                    let eliminations: Vec<Deduction> = grid.row_from_cell(cells[0].idx())
                        .cells()
                        .iter()
                        .filter(|c| c.has_candidate(value) && c.block() != cells[0].block())
                        .map(|c| Deduction::Elimination(c.idx(), value))
                        .collect();
                    if eliminations.len() > 0 {
                        return Some(eliminations);
                    }
                }

                // Columns
                if cells.iter().all(|x| x.column() == cells[0].column()) {
                    let eliminations: Vec<Deduction> = grid.column_from_cell(cells[0].idx())
                        .cells()
                        .iter()
                        .filter(|c| c.has_candidate(value) && c.block() != cells[0].block())
                        .map(|c| Deduction::Elimination(c.idx(), value))
                        .collect();
                    if eliminations.len() > 0 {
                        return Some(eliminations);
                    }
                }
            }
        }
    }

    None
}