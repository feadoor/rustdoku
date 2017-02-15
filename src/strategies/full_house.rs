//! A definition of the full house strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a full house.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each region and check if it has only one empty cell.
    for region in Grid::regions() {
        let empty_cells = grid.empty_cells_in_region(region);
        if empty_cells.len() == 1 {
            let cell_idx = empty_cells.first().unwrap();

            // If the puzzle was invalid, then we could have a cell with no candidates. Check.
            if grid.num_candidates(cell_idx) == 0 {
                return Some(Move { deductions: vec![Deduction::Contradiction] });
            }

            // Return the deduction arising from the full house.
            let val = grid.first_candidate(cell_idx).unwrap();
            let deduction = Deduction::Placement(cell_idx, val);
            return Some(Move { deductions: vec![deduction] });
        }
    }

    None
}