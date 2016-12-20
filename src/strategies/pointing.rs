//! A definition of the pointing strategy.

use grid::Grid;
use strategies::{Deduction, Move};
use strategies::outputs::Pointing;

/// Return, if one exists, a deduction based on pointing.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column. Then other occurrences of that value can be removed from other cells in the row
/// or column.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each block, and for each value, check if the positions are limited to a row or column.
    for block in Grid::blocks() {
        for &val in Grid::values() {
            let cells = block.filter(|&ix| grid.has_candidate(ix, val));
            if cells.len() >= 2 {

                // Rows
                if Grid::same_row(&cells) {
                    let cell = cells.first().unwrap();
                    let row = Grid::row(cell);
                    let eliminations = (row & !Grid::block(cell))
                        .iter()
                        .filter(|&ix| grid.has_candidate(ix, val))
                        .map(|ix| Deduction::Elimination(ix, val))
                        .collect::<Vec<_>>();

                    if !eliminations.is_empty() {

                        // Get a human-readable description of the deduction and return it.
                        let reason = Pointing {
                            block: block.clone(),
                            value: val,
                            region: row.clone()
                        };
                        return Some(Move {
                            deductions: eliminations,
                            reason: Box::new(reason),
                        });
                    }
                }

                // Columns
                if Grid::same_column(&cells) {
                    let cell = cells.first().unwrap();
                    let col = Grid::column(cell);
                    let eliminations = (col & !Grid::block(cell))
                        .iter()
                        .filter(|&ix| grid.has_candidate(ix, val))
                        .map(|ix| Deduction::Elimination(ix, val))
                        .collect::<Vec<_>>();

                    if !eliminations.is_empty() {

                        // Get a human-readable description of the deduction and return it.
                        let reason = Pointing {
                            block: block.clone(),
                            value: val,
                            region: col.clone()
                        };
                        return Some(Move {
                            deductions: eliminations,
                            reason: Box::new(reason),
                        });
                    }
                }
            }
        }
    }

    None
}