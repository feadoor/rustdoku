//! A definition of the naked single strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a naked single.
///
/// A naked single is when a given cell has only one candidate that it can contain.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each cell and check if it is a naked single.
    for cell in grid.cells().iter() {
        if cell.candidates().len() == 1 {
            let val = cell.candidates().iter().next().unwrap();
            let deduction = Deduction::Placement(cell.idx(), val);
            return Some(vec![deduction]);
        }
    }

    None
}