//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod naked_single;
mod hidden_single;

use grid::{CellIdx, Grid};

/// The different types of deduction that can be made on a grid.
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
}

/// Find the simplest deduction that can be applied to the grid.
fn find_deduction(grid: &Grid) -> Option<Vec<Deduction>> {

    // Full House
    if let Some(deductions) = full_house::find(grid) {
        return Some(deductions);
    }

    // Naked Single
    if let Some(deductions) = naked_single::find(grid) {
        return Some(deductions);
    }

    // Hidden Single
    if let Some(deductions) = hidden_single::find(grid) {
        return Some(deductions);
    }

    None
}

/// Decide if the grid is solved or not.
fn is_solved(grid: &Grid) -> bool {
    grid.cells().iter().all(|x| !x.is_empty())
}

/// Apply the results of a deduction to the grid.
fn apply_deduction(grid: &mut Grid, deduction: Deduction) {
    match deduction {
        Deduction::Placement(cell_idx, val) => grid.place_value(cell_idx, val),
        Deduction::Elimination(cell_idx, val) => grid.eliminate_value(cell_idx, val),
    }
}

/// Solve the grid using the available strategies.
pub fn solve(grid: &mut Grid) {
    while !is_solved(grid) {
        if let Some(deductions) = find_deduction(grid) {
            for deduction in deductions {
                apply_deduction(grid, deduction);
            }
        } else {
            break;
        }
    }
}
