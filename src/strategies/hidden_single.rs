//! A definition of the hidden single strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a hidden single.
///
/// A hidden single is when a given region has only one spot for a particular value. Then that
/// value can be placed in that location.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each region, and check if any value has only one position.
    for region in Grid::regions() {
        for &val in Grid::values() {
            let cells: Vec<_> = region.iter().filter(|&&ix| grid.has_candidate(ix, val)).collect();
            if cells.len() == 1 {
                let &cell_idx = cells[0];
                let deduction = Deduction::Placement(cell_idx, val);
                return Some(vec![deduction]);
            }
        }
    }

    None
}