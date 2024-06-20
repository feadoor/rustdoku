//! A definition of the full house strategy.

use grid::{Grid, GridSize};
use strategies::{Step, Deduction};
use utils::GeneratorAdapter;

/// Find the full houses that occur in the given grid.
///
/// A full house is when a given region has only one empty cell. Then that cell can be filled in
/// with the last remaining value.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // Scan each region and check if it has only one empty cell.
        for region in grid.all_regions() {
            let empty_cells = grid.empty_cells_in_region(region);
            if empty_cells.len() == 1 {
                let cell = empty_cells.first().unwrap();

                // If the puzzle was invalid, then we could have a cell with no candidates. Check.
                if grid.num_candidates(cell) == 0 {
                    yield Step::NoCandidatesForCell{ cell: cell };
                }

                // Return the deduction arising from the full house.
                else {
                    let val = grid.first_candidate(cell).unwrap();
                    yield Step::FullHouse{ region: region.clone(), cell: cell, value: val };
                }
            }
        }
    })
}

/// Get the deductions arising from the full house on the given grid.
pub fn get_deductions<T: GridSize>(_grid: &Grid<T>, full_house: &Step<T>) -> Vec<Deduction> {
    match full_house {
        Step::FullHouse { cell, value, .. } => vec![ Deduction::Placement(*cell, *value) ],
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, full_house: &Step<T>) -> String {
    match full_house {
        Step::FullHouse { region, cell, value } => format!(
            "Full House - {} is the last cell in {}, and must contain {}",
            grid.cell_name(*cell), grid.region_name(region), *value,
        ),
        _ => unreachable!(),
    }
}
