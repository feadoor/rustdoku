//! A definition of the XY-wing strategy.

use grid::{Grid, CellIdx};
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the XY-wings that exist in the grid.
///
/// An XY-wing is a pattern comprising 3 bi-value cells. Suppose we have a cell, called the pivot,
/// with two candidates XY. Suppose that there are two cells within sight of the pivot, called the
/// pincers, which have candidates XZ and YZ. Then Z can be eliminated from all cells which can see
/// both pincers.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move ||{
        // Iterate over bi-value cells of the grid as the pivot and look for pairs of pincer cells.
        for pivot in grid.cells_with_n_candidates(2).iter() {
            for pincer1 in pincers(grid, pivot).iter() {
                let candidates = grid.candidates(pincer1) ^ grid.candidates(pivot);
                for pincer2 in grid.cells_with_exact_candidates_in_region(&candidates, Grid::neighbours(pivot)).iter() {

                    // Check for eliminations coming from this wing.
                    let ex_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2)).first().unwrap();
                    let elim_region = Grid::neighbours(pincer1) & Grid::neighbours(pincer2);
                    if !grid.cells_with_candidate_in_region(ex_candidate, &elim_region).is_empty() {
                        yield Step::XYWing { pivot, pincer1, pincer2, value: ex_candidate };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the XY-wing on the given grid.
pub fn get_deductions(grid: &Grid, xy_wing: &Step) -> Vec<Deduction> {
    match *xy_wing {
        Step::XYWing { pincer1, pincer2, value, .. } => grid
            .cells_with_candidate_in_region(value, &(Grid::neighbours(pincer1) & Grid::neighbours(pincer2)))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(xy_wing: &Step) -> String {
    match *xy_wing {
        Step::XYWing { pivot, pincer1, pincer2, value } => format!(
            "XY-Wing - pivot {} and pincers ({}, {}) eliminate {} from common neighbours",
            Grid::cell_name(pivot), Grid::cell_name(pincer1), Grid::cell_name(pincer2), value,
        ),
        _ => unreachable!(),
    }
}

/// Return a `CellSet` consisting of possible pincer cells for the given pivot - that is, bivalue
/// cells which can see the pivot and which have a candidate in common with it.
fn pincers(grid: &Grid, pivot: CellIdx) -> CellSet {
    grid.cells_with_n_candidates_in_region(2, Grid::neighbours(pivot))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 1)
}
