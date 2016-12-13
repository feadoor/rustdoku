//! Definitions of solving strategies for Sudoku puzzles.

mod naked_single;

use grid::{CellIdx, Grid};

/// The different types of deduction that can be made on a grid.
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
}

/// Find the simplest deduction that can be applied to the grid.
pub fn find_deduction(grid: &Grid) -> Option<Vec<Deduction>> {
    if let Some(deduction) = naked_single::find(grid) {
        return Some(deduction);
    }

    None
}
