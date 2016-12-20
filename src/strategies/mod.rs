//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod hidden_single;
mod naked_single;
mod pointing;
mod claiming;
mod naked_subset;
mod hidden_subset;
mod basic_fish;
mod xy_wing;
mod xyz_wing;

mod outputs;

use std::fmt;

use grid::{CellIdx, Grid};

/// The different types of deduction that can be made on a grid.
#[derive(Debug)]
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
    /// Indicates that the grid was found to be in contradiction.
    Contradiction,
}

/// A trait which is used to describe a logical deduction.
pub trait Reason {
    /// A description of the logic behind some deductions.
    fn description(&self) -> Result<String, fmt::Error>;
}

/// A structure representing the inference that the grid is in contradiction.
pub struct Contradiction { }

impl Reason for Contradiction {
    fn description(&self) -> Result<String, fmt::Error> {
        Ok("The grid is in contradiction".to_string())
    }
}

/// A move that can be made on the grid.
pub struct Move {
    /// The placements or eliminations resulting from this move.
    deductions: Vec<Deduction>,
    /// The reason for the deductions.
    reason: Box<Reason>,
}

impl Reason for Move {
    fn description(&self) -> Result<String, fmt::Error> {
        self.reason.description()
    }
}

/// Find the simplest deduction that can be applied to the grid.
fn find_move(grid: &Grid) -> Option<Move> {

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
    search!(xy_wing, grid);
    search!(xyz_wing, grid);

    None
}

/// Decide if the grid is solved or not.
fn is_solved(grid: &Grid) -> bool {
    Grid::cells().iter().all(|ix| !grid.is_empty(ix))
}

/// Apply the results of a deduction to the grid.
fn apply_deduction(grid: &mut Grid, deduction: Deduction) -> bool {
    match deduction {
        Deduction::Contradiction => return false,
        Deduction::Placement(cell_idx, val) => grid.place_value(cell_idx, val),
        Deduction::Elimination(cell_idx, val) => grid.eliminate_value(cell_idx, val),
    }

    true
}

/// Solve the grid using the available strategies.
pub fn solve(grid: &mut Grid, verbose: bool) {
    while !is_solved(grid) {
        if let Some(mov) = find_move(grid) {
            if verbose {
                println!("\n{}", mov.description().unwrap());
            }
            for deduction in mov.deductions {
                if !apply_deduction(grid, deduction) {
                    return;
                }
            }
            if verbose {
                println!("\n{}", grid);
            }
        } else {
            break;
        }
    }
}
