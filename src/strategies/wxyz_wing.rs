//! A definition of the WXYZ-wing strategy.

use grid::CellIdx;
use grid::{Grid, GridSize};
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find WXYZ-wings that exist in the grid.
///
/// A WXYZ-wing is a pattern comprising of four cells anywhere in the grid. These four cells must,
/// between them, contain a total of four candidate digits. If exactly one of those digits is not
/// restricted common, then it can be eliminated from all cells that can see all instances of that
/// digit inside the wing.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // We are concerned only with empty cells in the grid.
        let empty_cells = grid.empty_cells();

        // Iterate over combinations of four cells which between them have at most four candidates.
        for first_cell in empty_cells.iter() {
            for second_cell in continuation_cells(grid, &empty_cells, &[first_cell]).iter() {
                for third_cell in continuation_cells(grid, &empty_cells, &[first_cell, second_cell]).iter() {
                    for fourth_cell in continuation_cells(grid, &empty_cells, &[first_cell, second_cell, third_cell]).iter() {

                        // Check whether this wing provides any deductions.
                        let all_cells = CellSet::from_cells([first_cell, second_cell, third_cell, fourth_cell].iter().map(|x| *x));
                        for candidate in get_necessary_digits(grid, &all_cells).iter() {
                            if has_eliminations(grid, &all_cells, candidate) {
                                yield Step::WXYZWing { cells: all_cells.clone(), value: candidate};
                            }
                        }
                    }
                }
            }
        }

    })
}



/// Get the deductions arising from the WXYZ-wing on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, wxyz_wing: &Step<T>) -> Vec<Deduction> {
    match wxyz_wing {
        Step::WXYZWing { cells, value } =>
            get_elimination_cells(grid, cells, *value).map(|cell| Deduction::Elimination(cell, *value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, wxyz_wing: &Step<T>) -> String {
    match wxyz_wing {
        Step::WXYZWing { cells, value } => format!(
            "WXYZ-Wing - cells {} eliminate {} from common neighbours",
            grid.region_name(cells), value,
        ),
        _ => unreachable!(),
    }
}

/// Return a `CellSet` consisting of possible continuation cells for the WXYZ-Wing - that is, cells
/// which, in combination with the other cells already in use, do not contain more than 4 candidates.
fn continuation_cells<T: GridSize>(grid: &Grid<T>, empty_cells: &CellSet<T>, current_cells: &[CellIdx]) -> CellSet<T> {
    empty_cells
        .filter(|&ix| current_cells.iter().all(|&c| c < ix))
        .filter(|&ix| (current_cells.iter().fold(CandidateSet::empty(), |acc, &cell| acc | grid.candidates(cell)) | grid.candidates(ix)).len() <= 4)
}

/// Get the digits which must appear somewhere in the WXYZ-wing configuration
fn get_necessary_digits<T: GridSize>(grid: &Grid<T>, cells: &CellSet<T>) -> CandidateSet<T> {

    let all_candidates = cells.iter().fold(CandidateSet::empty(), |acc, cell| acc | grid.candidates(cell));
    let mut restricted_candidates = CandidateSet::empty();
    for candidate in all_candidates.iter() {
        let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &cells);
        if grid.all_regions().iter().any(|region| region.contains_all(&cells_with_candidate)) {
            restricted_candidates.add_candidate(candidate);
        }
    }

    let non_restricted_candidates = all_candidates & !restricted_candidates;
    match non_restricted_candidates.len() {
        0 => all_candidates,
        1 => non_restricted_candidates,
        _ => CandidateSet::empty(),
    }
}

/// Check if the given wing cells have any eliminations that can be made
fn get_elimination_cells<T: GridSize>(grid: &Grid<T>, cells: &CellSet<T>, candidate: usize) -> CellSet<T> {
    let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &cells);
    let possible_eliminations = grid.cells_with_candidate(candidate);
    possible_eliminations & CellSet::intersection(&cells_with_candidate.map(|ix| grid.neighbours(ix)))
}

/// Check if the given wing cells have any eliminations that can be made
fn has_eliminations<T: GridSize>(grid: &Grid<T>, cells: &CellSet<T>, candidate: usize) -> bool {
    get_elimination_cells(grid, cells, candidate).len() > 0
}
