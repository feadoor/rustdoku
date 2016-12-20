//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod naked_single;
mod hidden_single;
mod pointing;
mod claiming;
mod naked_subset;
mod hidden_subset;
mod basic_fish;

use grid::{CellIdx, Grid};

/// The different types of deduction that can be made on a grid.
#[derive(Debug)]
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
}

/// Find the simplest deduction that can be applied to the grid.
fn find_deduction(grid: &Grid) -> Option<Vec<Deduction>> {

    macro_rules! search {
        ($e: ident, $x: ident) => {
            let deductions = $e::find($x);
            if deductions.is_some() {
                return deductions;
            }
        }
    }

    search!(full_house, grid);
    search!(hidden_single, grid);
    search!(naked_single, grid);
    search!(pointing, grid);
    search!(claiming, grid);
    search!(naked_subset, grid);
    search!(hidden_subset, grid);
    search!(basic_fish, grid);

    None
}

/// Decide if the grid is solved or not.
fn is_solved(grid: &Grid) -> bool {
    Grid::cells().iter().all(|&ix| !grid.is_empty(ix))
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
