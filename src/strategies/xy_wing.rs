//! A definition of the XY-wing strategy.

use grid::CellIdx;
use grid::Grid;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, an elimination based on an XY-wing.
///
/// An XY-wing is a pattern comprising 3 bi-value cells. Suppose we have a cell, called the pivot,
/// with two candidates XY. Suppose that there are two cells within sight of the pivot, called the
/// pincers, which have candidates XZ and YZ. Then Z can be eliminated from all cells which can see
/// both pincers.
pub fn find(grid: &Grid) -> Option<Move> {

    // Iterate over bi-value cells of the grid as the pivot and look for pairs of pincer cells.
    for pivot in grid.cells_with_n_candidates(2).iter() {
        for pincer1 in pincers(grid, pivot).iter() {
            let candidates = grid.candidates(pincer1) ^ grid.candidates(pivot);
            for pincer2 in grid.cells_with_exact_candidates_in_region(&candidates, Grid::neighbours(pivot)).iter() {

                // Check for eliminations coming from this wing.
                let ex_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2)).first().unwrap();
                let elim_region = Grid::neighbours(pincer1) & Grid::neighbours(pincer2);
                let deductions = grid.cells_with_candidate_in_region(ex_candidate, &elim_region)
                    .map(|ix| Deduction::Elimination(ix, ex_candidate));
                if ! deductions.is_empty() {
                    return Some(Move { deductions: deductions });
                }
            }
        }
    }

    None
}

/// Return a `CellSet` consisting of possible pincer cells for the given pivot - that is, bivalue
/// cells which can see the pivot and which have a candidate in common with it.
fn pincers(grid: &Grid, pivot: CellIdx) -> CellSet {
    grid.cells_with_n_candidates_in_region(2, Grid::neighbours(pivot))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 1)
}