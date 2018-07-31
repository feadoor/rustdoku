//! A definition of the naked single strategy.

use grid::Grid;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the naked singles that appear in the grid.
///
/// A naked single is when a given cell has only one candidate that it can contain. Then that value
/// can be placed in the cell.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {
        // Scan each cell and check if it is a naked single.
        for cell_idx in Grid::cells().iter() {

            // Also put in a check for cells that have no candidates remaining.
            if grid.num_candidates(cell_idx) == 0 && grid.is_empty(cell_idx) {
                yield Step::NoCandidatesForCell{ cell: cell_idx };
            }

            // Check for a naked single deduction.
            if grid.num_candidates(cell_idx) == 1 {
                let val = grid.first_candidate(cell_idx).unwrap();
                yield Step::NakedSingle{ cell: cell_idx, value: val };
            }
        }
    })
}

/// Get the deductions arising from the naked single on the given grid.
pub fn get_deductions(_grid: &Grid, naked_single: &Step) -> Vec<Deduction> {
    match *naked_single {
        Step::NakedSingle { cell, value } => vec![ Deduction::Placement(cell, value) ],
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(naked_single: &Step) -> String {
    match *naked_single {
        Step::NakedSingle { cell, value } => format!(
            "Naked Single - {} can only contain {}",
            Grid::cell_name(cell), value,
        ),
        _ => unreachable!(),
    }
}