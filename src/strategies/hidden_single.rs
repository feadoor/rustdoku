//! A definition of the hidden single strategy.

use grid::{Grid, LARGE_SIZE};
use strategies::Deduction;

/// Return, if one exists, a deduction based on a hidden single.
///
/// A hidden single is when a given region has only one spot for a particular value. Then that
/// value can be placed in that location.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each region, and check if any value has only one position.
    for region in grid.regions() {
        for value in 1..LARGE_SIZE + 1 {
            let cells = region.potential_cells(value);
            if cells.len() == 1 {
                let cell = cells[0];
                let deduction = Deduction::Placement(cell.idx(), value);
                return Some(vec![deduction]);
            }
        }
    }

    None
}