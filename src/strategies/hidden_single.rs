//! A definition of the naked single strategy.

use grid::{Grid, LARGE_SIZE};
use strategies::Deduction;

/// Return, if one exists, a deduction based on a hidden single.
///
/// A hidden single is when a given region has only one spot for a particular value.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each region, and check if any value has only one position.
    for region in grid.regions() {
        for value in 1..LARGE_SIZE + 1 {
            let cells = region.potential_cells(value);
            if cells.len() == 1 {
                let deduction = Deduction::Placement(cells[0].idx(), value);
                return Some(vec![deduction]);
            }
        }
    }

    None
}