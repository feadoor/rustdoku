//! A definition of the pointing strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on pointing.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column. Then other occurrences of that value can be removed from other cells in the row
/// or column.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each block, and for each value, check if the positions are limited to a row or column.
    for block in Grid::blocks() {
        for &val in Grid::values() {
            let cells: Vec<_> = block.iter().filter(|&&ix| grid.has_candidate(ix, val)).collect();
            if cells.len() >= 2 {

                // Rows
                if Grid::same_row(&cells) {
                    let eliminations = (Grid::rowset(*cells[0]) & !Grid::blockset(*cells[0]))
                        .iter()
                        .filter(|&ix| grid.has_candidate(ix, val))
                        .map(|ix| Deduction::Elimination(ix, val))
                        .collect::<Vec<_>>();

                    if !eliminations.is_empty() {
                        return Some(eliminations);
                    }
                }

                // Columns
                if Grid::same_column(&cells) {
                    let eliminations = (Grid::colset(*cells[0]) & !Grid::blockset(*cells[0]))
                        .iter()
                        .filter(|&ix| grid.has_candidate(ix, val))
                        .map(|ix| Deduction::Elimination(ix, val))
                        .collect::<Vec<_>>();

                    if !eliminations.is_empty() {
                        return Some(eliminations);
                    }
                }
            }
        }
    }

    None
}