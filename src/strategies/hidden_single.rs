//! A definition of the hidden single strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a hidden single.
///
/// A hidden single is when a given region has only one spot for a particular value. Then that
/// value can be placed in that location.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each region, and check if any value has only one position.
    for region in Grid::regions() {
        for &val in Grid::values() {
            let cells = region.filter(|&ix| grid.has_candidate(ix, val));

            // There might be no place for this value, which is a contradiction. Check.
            if cells.len() == 0 && !region.iter().any(|ix| grid.value(ix) == Some(val)) {
                return Some(Move { deductions: vec![Deduction::Contradiction] });
            }

            // If we get here then all is well, so continue as normal.
            if cells.len() == 1 {
                // Get a human-readable description of the deduction and return it.
                let cell_idx = cells.first().unwrap();
                let deduction = Deduction::Placement(cell_idx, val);
                return Some(Move { deductions: vec![deduction] });
            }
        }
    }

    None
}