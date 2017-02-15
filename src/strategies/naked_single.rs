//! A definition of the naked single strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a naked single.
///
/// A naked single is when a given cell has only one candidate that it can contain. Then that value
/// can be placed in the cell.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each cell and check if it is a naked single.
    for cell_idx in Grid::cells().iter() {

        // Also put in a check for cells that have no candidates remaining.
        if grid.num_candidates(cell_idx) == 0 && grid.is_empty(cell_idx) {
            return Some(Move { deductions: vec![Deduction::Contradiction] })
        }

        // Check for a naked single deduction.
        if grid.num_candidates(cell_idx) == 1 {
            let val = grid.first_candidate(cell_idx).unwrap();
            let deduction = Deduction::Placement(cell_idx, val);
            return Some(Move { deductions: vec![deduction] });
        }
    }

    None
}