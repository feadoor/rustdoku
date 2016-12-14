//! A definition of the full house strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a full house.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each region and check if it has only one empty cell.
    for region in grid.regions() {
        let empty_cells = region.empty_cells();
        if empty_cells.len() == 1 {
            let cell = empty_cells[0];
            let val = cell.candidates().iter().next().unwrap();
            let deduction = Deduction::Placement(cell.idx(), val);
            return Some(vec![deduction]);
        }
    }

    None
}