//! A definition of the claiming strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on claiming.
///
/// Pointing occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each row / column, and for each value, check if the positions are limited to a
    // particular block.

    // Rows
    for row in Grid::rows() {
        for &val in Grid::values() {
            let cells: Vec<_> = row.iter().filter(|&&ix| grid.has_candidate(ix, val)).collect();
            if cells.len() >= 2 && Grid::same_block(&cells) {
                let eliminations = (Grid::blockset(*cells[0]) & !Grid::rowset(*cells[0]))
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

    // Columns
    for col in Grid::columns() {
        for &val in Grid::values() {
            let cells: Vec<_> = col.iter().filter(|&&ix| grid.has_candidate(ix, val)).collect();
            if cells.len() >= 2 && Grid::same_block(&cells) {
                let eliminations = (Grid::blockset(*cells[0]) & !Grid::colset(*cells[0]))
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

    None
}