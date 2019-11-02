//! A definition of the cell interactions strategy.

use grid::{Grid, GridSize};
use grid::placementset::PlacementSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the instances of cell interactions that appear in the grid.
///
/// A cell interaction occurs when all possible values for a cell have a common elimination. Since
/// once of the cell's candidates must be true, the common elimination can be made immediately.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Scan each cell, and check for common eliminations of its candidates.
        for cell in grid.empty_cells().iter() {
            let neighbours = grid.candidates(cell).map(|val| grid.neighbours(cell, val));
            let common_neighbours = PlacementSet::intersection(&neighbours);
            let eliminations = common_neighbours.filter(|p| grid.has_candidate(p.cell, p.candidate));
            if !eliminations.is_empty() {
                yield Step::CellInteraction { cell: cell, eliminations: eliminations };
            }
        }
    })
}

/// Get the deductions arising from the box-line interactions on the given grid.
pub fn get_deductions<T: GridSize>(_grid: &Grid<T>, cell_interaction: &Step<T>) -> Vec<Deduction> {
    match cell_interaction {
        Step::CellInteraction { eliminations, .. } =>
            eliminations.map(|p| Deduction::Elimination(p.cell, p.candidate)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, cell_interaction: &Step<T>) -> String {
    match cell_interaction {
        Step::CellInteraction { cell, .. } => format!(
            "Cell interaction - the candidates in {} lead to a common elimination",
            grid.cell_name(*cell),
        ),
        _ => unreachable!(),
    }
}
