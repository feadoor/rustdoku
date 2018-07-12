//! A definition of the full house strategy.

use grid::Grid;
use strategies::{Step, Deduction};

/// Return, if one exists, a full house.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find(grid: &Grid) -> Option<Step> {

    // Scan each region and check if it has only one empty cell.
    for region in Grid::regions() {
        let empty_cells = grid.empty_cells_in_region(region);
        if empty_cells.len() == 1 {
            let cell_idx = empty_cells.first().unwrap();

            // If the puzzle was invalid, then we could have a cell with no candidates. Check.
            if grid.num_candidates(cell_idx) == 0 {
                return Some(Step::NoCandidatesForCell{ cell: cell_idx });
            }

            // Return the deduction arising from the full house.
            let val = grid.first_candidate(cell_idx).unwrap();
            return Some(Step::FullHouse{ region: *region, cell: cell_idx, value: val });
        }
    }

    None
}

/// Get the deductions arising from the full house on the given grid.
pub fn get_deductions(_grid: &Grid, full_house: &Step) -> Vec<Deduction> {
    match *full_house {
        Step::FullHouse { region: _, cell, value } => vec![ Deduction::Placement(cell, value) ],
        _ => unreachable!(),
    }
}