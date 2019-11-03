//! A definition of the box-line interactions strategy.

use grid::{Grid, GridSize};
use grid::placementset::PlacementSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the instances of box-line interactions that appear in the grid.
///
/// A box-line interaction occurs when all occurrences of a given value within a region have other
/// common neighbours outside of that region. In this case, all instances of the candidate in the
/// common neighbours can be eliminated.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Scan each region, and for each value, check if the common neighbours allow for eliminations.
        for region in grid.all_regions() {
            for val in grid.values_missing_from_region(region).iter() {
                let cells = grid.cells_with_candidate_in_region(val, region);

                // Grab the common neighbours and look for instances of the target candidate
                let neighbours = cells.map(|c| grid.neighbours(c, val));
                let common_neighbours = PlacementSet::intersection(&neighbours);
                let eliminations = common_neighbours.filter(|p| grid.has_candidate(p.cell, p.candidate));
                if !eliminations.is_empty() {
                    yield Step::BoxLine { region: region.clone(), eliminations: eliminations, value: val };
                }
            }
        }
    })
}

/// Get the deductions arising from the box-line interactions on the given grid.
pub fn get_deductions<T: GridSize>(_grid: &Grid<T>, box_line: &Step<T>) -> Vec<Deduction> {
    match box_line {
        Step::BoxLine { eliminations, .. } =>
            eliminations.map(|p| Deduction::Elimination(p.cell, p.candidate)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, box_line: &Step<T>) -> String {
    match box_line {
        Step::BoxLine { region, value, .. } => format!(
            "Box-line interaction - the {}s in {} eliminate common neighbours",
            value, grid.region_name(region),
        ),
        _ => unreachable!(),
    }
}
