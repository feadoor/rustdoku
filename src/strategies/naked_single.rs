//! A definition of the naked single strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a naked single.
///
/// A naked single is when a given cell has only one candidate that it can contain. Then that value
/// can be placed in the cell.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each cell and check if it is a naked single.
    for &cell_idx in Grid::cells() {
        if grid.num_candidates(cell_idx) == 1 {
            let val = grid.first_candidate(cell_idx);
            let deduction = Deduction::Placement(cell_idx, val);
            return Some(vec![deduction]);
        }
    }

    None
}