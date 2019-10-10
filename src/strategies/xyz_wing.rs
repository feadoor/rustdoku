//! A definition of the XYZ-wing strategy.

use grid::CellIdx;
use grid::{Grid, GridSize};
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find XYZ-wings that exist in the grid.
///
/// An XYZ-wing is a pattern comprising a tri-value cell and two bi-value cells. Suppose we have a
/// cell, called the pivot, with three candidates XYZ. Suppose that there are two cells within
/// sight of the pivot, called the pincers, which have candidates XZ and YZ. Then Z can be
/// eliminated from all cells which can see the pivot and both pincers.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Iterate over tri-value cells of the grid as the pivot and look for pincer cells.
        for pivot in grid.cells_with_n_candidates(3).iter() {
            for pincer1 in first_pincers(grid, pivot).iter() {
                for pincer2 in second_pincers(grid, pivot, pincer1).iter() {

                    // Check for eliminations coming from this wing.
                    let ex_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2)).first().unwrap();
                    let elim_region = grid.neighbours(pincer1) & grid.neighbours(pincer2) & grid.neighbours(pivot);
                    if !grid.cells_with_candidate_in_region(ex_candidate, &elim_region).is_empty() {
                        yield Step::XYZWing { pivot, pincer1, pincer2, value: ex_candidate };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the XYZ-wing on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, xyz_wing: &Step<T>) -> Vec<Deduction> {
    match *xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } =>
            grid.cells_with_candidate_in_region(value, &(grid.neighbours(pincer1) & grid.neighbours(pincer2) & grid.neighbours(pivot)))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, xyz_wing: &Step<T>) -> String {
    match *xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } => format!(
            "XYZ-Wing - pivot {} and pincers ({}, {}) eliminate {} from common neighbours",
            grid.cell_name(pivot), grid.cell_name(pincer1), grid.cell_name(pincer2), value,
        ),
        _ => unreachable!(),
    }
}

/// Return a `CellSet` consisting of possible pincer cells for the given pivot - that is, bivalue
/// cells which can see the pivot and which have two candidates in common with it.
fn first_pincers<T: GridSize>(grid: &Grid<T>, pivot: CellIdx) -> CellSet<T> {
    grid.cells_with_n_candidates_in_region(2, grid.neighbours(pivot))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 2)
}

/// Return a `CellSet` consisting of possible second pincer cells for the given pivot and first
/// pincer - that is, bivalue cells which can see the pivot, cannot see the first pincer, and which
/// are bivalue with both candidates in common with the pivot and one candidate in common with the
/// other pincer.
fn second_pincers<T: GridSize>(grid: &Grid<T>, pivot: CellIdx, pincer: CellIdx) -> CellSet<T> {
    grid.cells_with_n_candidates_in_region(2, &(grid.neighbours(pivot) & !grid.neighbours(pincer)))
        .filter(|&ix| (grid.candidates(ix) & grid.candidates(pivot)).len() == 2)
        .filter(|&ix| (grid.candidates(ix) != grid.candidates(pincer)))
}
