//! A definition of the XYZ-wing strategy.

use grid::CellIdx;
use grid::Grid;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};

/// Return, if one exists, an XYZ-wing.
///
/// An XYZ-wing is a pattern comprising a tri-value cell and two bi-value cells. Suppose we have a
/// cell, called the pivot, with three candidates XYZ. Suppose that there are two cells within
/// sight of the pivot, called the pincers, which have candidates XZ and YZ. Then Z can be
/// eliminated from all cells which can see the pivot and both pincers.
pub fn find(grid: &Grid) -> Option<Step> {
    // Iterate over tri-value cells of the grid as the pivot and look for pincer cells.
    for pivot in grid.cells_with_n_candidates(3).iter() {
        for pincer1 in first_pincers(grid, pivot).iter() {
            for pincer2 in second_pincers(grid, pivot, pincer1).iter() {

                // Check for eliminations coming from this wing.
                let ex_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2)).first().unwrap();
                let elim_region = Grid::neighbours(pincer1) & Grid::neighbours(pincer2) & Grid::neighbours(pivot);
                if !grid.cells_with_candidate_in_region(ex_candidate, &elim_region).is_empty() {
                    return Some(Step::XYZWing { pivot, pincer1, pincer2, value: ex_candidate });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the XYZ-wing on the given grid.
pub fn get_deductions(grid: &Grid, xyz_wing: &Step) -> Vec<Deduction> {
    match *xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } =>
            grid.cells_with_candidate_in_region(value, &(Grid::neighbours(pincer1) & Grid::neighbours(pincer2) & Grid::neighbours(pivot)))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(xyz_wing: &Step) -> String {
    match *xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } => format!(
            "XYZ-Wing - pivot {} and pincers ({}, {}) eliminate {} from common neighbours",
            Grid::cell_name(pivot), Grid::cell_name(pincer1), Grid::cell_name(pincer2), value,
        ),
        _ => unreachable!(),
    }
}

/// Return a `CellSet` consisting of possible pincer cells for the given pivot - that is, bivalue
/// cells which can see the pivot and which have two candidates in common with it.
fn first_pincers(grid: &Grid, pivot: CellIdx) -> CellSet {
    grid.cells_with_n_candidates_in_region(2, Grid::neighbours(pivot))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 2)
}

/// Return a `CellSet` consisting of possible second pincer cells for the given pivot and first
/// pincer - that is, bivalue cells which can see the pivot, cannot see the first pincer, and which
/// are bivalue with both candidates in common with the pivot and one candidate in common with the
/// other pincer.
fn second_pincers(grid: &Grid, pivot: CellIdx, pincer: CellIdx) -> CellSet {
    grid.cells_with_n_candidates_in_region(2, &(Grid::neighbours(pivot) & !Grid::neighbours(pincer)))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 2)
        .filter(|&ix| (grid.candidates(ix) != grid.candidates(pincer)))
}