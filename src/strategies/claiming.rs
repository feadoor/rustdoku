//! A definition of the claiming strategy.

use grid::Grid;
use strategies::{Deduction, Move};
use strategies::outputs::Claiming;

/// Return, if one exists, a deduction based on claiming.
///
/// Pointing occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each row / column, and for each value, check if the positions are limited to a
    // particular block.

    // Rows
    for row in Grid::rows() {
        for &val in Grid::values() {
            let cells = row.filter(|&ix| grid.has_candidate(ix, val));
            if cells.len() >= 2 && Grid::same_block(&cells) {
                let cell = cells.first().unwrap();
                let block = Grid::block(cell);
                let eliminations = (block & !Grid::row(cell))
                    .iter()
                    .filter(|&ix| grid.has_candidate(ix, val))
                    .map(|ix| Deduction::Elimination(ix, val))
                    .collect::<Vec<_>>();

                if !eliminations.is_empty() {

                    // Get a human-readable description of the deduction and return it.
                    let reason = Claiming {
                        region: row.clone(),
                        value: val,
                        block: block.clone()
                    };
                    return Some(Move {
                        deductions: eliminations,
                        reason: Box::new(reason),
                    });
                }
            }
        }
    }

    // Columns
    for col in Grid::columns() {
        for &val in Grid::values() {
            let cells = col.filter(|&ix| grid.has_candidate(ix, val));
            if cells.len() >= 2 && Grid::same_block(&cells) {
                let cell = cells.first().unwrap();
                let block = Grid::block(cell);
                let eliminations = (Grid::block(cell) & !Grid::column(cell))
                    .iter()
                    .filter(|&ix| grid.has_candidate(ix, val))
                    .map(|ix| Deduction::Elimination(ix, val))
                    .collect::<Vec<_>>();

                if !eliminations.is_empty() {

                    // Get a human-readable description of the deduction and return it.
                    let reason = Claiming {
                        region: col.clone(),
                        value: val,
                        block: block.clone()
                    };
                    return Some(Move {
                        deductions: eliminations,
                        reason: Box::new(reason),
                    });
                }
            }
        }
    }

    None
}