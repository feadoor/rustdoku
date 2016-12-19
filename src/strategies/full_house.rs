//! A definition of the full house strategy.

use grid::Grid;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a full house.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    // Scan each region and check if it has only one empty cell.
    for region in Grid::regions() {
        let empty_cells: Vec<_> = region.iter().filter(|&&ix| grid.is_empty(ix)).collect();
        if empty_cells.len() == 1 {
            let &cell_idx = empty_cells[0];
            let val = grid.first_candidate(cell_idx);
            let deduction = Deduction::Placement(cell_idx, val);
            return Some(vec![deduction]);
        }
    }

    None
}