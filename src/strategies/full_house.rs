//! A definition of the full house strategy.

use grid::Grid;
use strategies::{Contradiction, Deduction, Move};
use strategies::outputs::FullHouse;

/// Return, if one exists, a deduction based on a full house.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find(grid: &Grid) -> Option<Move> {

    // Scan each region and check if it has only one empty cell.
    for region in Grid::regions() {
        let empty_cells = region.filter(|&ix| grid.is_empty(ix));
        if empty_cells.len() == 1 {
            let cell_idx = empty_cells.first().unwrap();

            // If the puzzle was invalid, then we could have a cell with no candidates. Check.
            if grid.num_candidates(cell_idx) == 0 {
                return Some(Move {
                    deductions: vec![Deduction::Contradiction],
                    reason: Box::new(Contradiction { }),
                });
            }

            // Get a human-readable description of the deduction and return it.
            let val = grid.first_candidate(cell_idx);
            let deduction = Deduction::Placement(cell_idx, val);
            let reason = FullHouse {
                cell: cell_idx,
                region: region.clone()
            };

            return Some(Move {
                deductions: vec![deduction],
                reason: Box::new(reason),
            });
        }
    }

    None
}