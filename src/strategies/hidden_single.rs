//! A definition of the hidden single strategy.

use grid::Grid;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the hidden singles that appear in the grid.
///
/// A hidden single is when a given region has only one spot for a particular value. Then that
/// value can be placed in that location.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {
        // Scan each region, and check if any value has only one position.
        for region in Grid::regions() {
            for val in grid.missing_values_from_region(region).iter() {
                let cells = grid.cells_with_candidate_in_region(val, region);

                // There might be no place for this value, which is a contradiction. Check.
                if cells.len() == 0 {
                    yield Step::NoPlaceForCandidateInRegion { region: *region, value: val};
                }

                // Otherwise check for a hidden single deduction.
                if cells.len() == 1 {
                    let cell_idx = cells.first().unwrap();
                    yield Step::HiddenSingle { region: *region, cell: cell_idx, value: val };
                }
            }
        }
    })
}

/// Get the deductions arising from the hidden single on the given grid.
pub fn get_deductions(_grid: &Grid, hidden_single: &Step) -> Vec<Deduction> {
    match *hidden_single {
        Step::HiddenSingle { cell, value, .. } => vec![ Deduction::Placement(cell, value) ],
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(hidden_single: &Step) -> String {
    match *hidden_single {
        Step::HiddenSingle { region, cell, value } => format!(
            "Hidden Single - {} is the only place for {} in {}",
            Grid::cell_name(cell), value, Grid::region_name(&region),
        ),
        _ => unreachable!(),
    }
}