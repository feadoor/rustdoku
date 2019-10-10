//! A definition of the naked single strategy.

use grid::{Grid, GridSize};
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the naked singles that appear in the grid.
///
/// A naked single is when a given cell has only one candidate that it can contain. Then that value
/// can be placed in the cell.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Scan each cell and check if it is a naked single.
        for cell in grid.cells().iter() {

            // Also put in a check for cells that have no candidates remaining.
            if grid.num_candidates(cell) == 0 && grid.is_empty(cell) {
                yield Step::NoCandidatesForCell{ cell: cell };
            }

            // Check for a naked single deduction.
            if grid.num_candidates(cell) == 1 {
                let val = grid.first_candidate(cell).unwrap();
                yield Step::NakedSingle{ cell: cell, value: val };
            }
        }
    })
}

/// Get the deductions arising from the naked single on the given grid.
pub fn get_deductions<T: GridSize>(_grid: &Grid<T>, naked_single: &Step<T>) -> Vec<Deduction> {
    match *naked_single {
        Step::NakedSingle { cell, value } => vec![ Deduction::Placement(cell, value) ],
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, naked_single: &Step<T>) -> String {
    match *naked_single {
        Step::NakedSingle { cell, value } => format!(
            "Naked Single - {} can only contain {}",
            grid.cell_name(cell), value,
        ),
        _ => unreachable!(),
    }
}
