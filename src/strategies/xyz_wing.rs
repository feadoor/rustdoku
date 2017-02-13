//! A definition of the XYZ-wing strategy.

use grid::Grid;
use strategies::{Deduction, Move};

/// Return, if one exists, an elimination based on an XYZ-wing.
///
/// An XYZ-wing is a pattern comprising a tri-value cell and two bi-value cells. Suppose we have a
/// cell, called the pivot, with three candidates XYZ. Suppose that there are two cells within
/// sight of the pivot, called the pincers, which have candidates XZ and YZ. Then Z can be
/// eliminated from all cells which can see the pivot and both pincers.
pub fn find(grid: &Grid) -> Option<Move> {
    // Iterate over tri-value cells of the grid as the pivot.
    for pivot in Grid::cells().iter().filter(|&ix| grid.num_candidates(ix) == 3) {
        // Look for possible choices for the first pincer cell.
        for pincer1 in Grid::neighbours(pivot).iter()
            .filter(|&ix| grid.num_candidates(ix) == 2 &&
                          grid.candidates(ix) & grid.candidates(pivot) == grid.candidates(ix))
        {
            // Now get a cell that can act as the second pincer.
            for pincer2 in (Grid::neighbours(pivot) & !Grid::neighbours(pincer1)).iter()
                .filter(|&ix| grid.num_candidates(ix) == 2 &&
                              grid.candidates(ix) != grid.candidates(pincer1) &&
                              grid.candidates(ix) & grid.candidates(pivot) == grid.candidates(ix))
            {
                // Check for eliminations coming from this wing.
                let ex_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2))
                    .trailing_zeros() as usize + 1;
                let deductions = (Grid::neighbours(pincer1) &
                                  Grid::neighbours(pincer2) &
                                  Grid::neighbours(pivot))
                    .iter()
                    .filter(|&ix| grid.has_candidate(ix, ex_candidate))
                    .map(|ix| Deduction::Elimination(ix, ex_candidate))
                    .collect::<Vec<_>>();

                if ! deductions.is_empty() {
                    return Some(Move { deductions: deductions });
                }
            }
        }

    }

    None
}
