//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::{Grid, GridSize};
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the hidden subsets, of the given degree, in the grid.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find_with_degree<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // Iterate over all regions of the grid and all tuples of values.
        for region in grid.all_regions() {
            for candidates in grid.values_missing_from_region(region).iter().combinations(degree).map(CandidateSet::from_candidates) {

                // Take the collection of cells which contain these candidates.
                let cells = grid.cells_with_candidates_in_region(&candidates, region);

                // Check if the candidates appear in the right number of cells and if any eliminations will occur.
                if cells.len() == degree {
                    if cells.iter().any(|cell| !(grid.candidates(cell) & !candidates).is_empty()) {
                        yield Step::HiddenSubset { region: region.clone(), cells: cells.clone(), values: candidates };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the hidden subset on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, hidden_subset: &Step<T>) -> Vec<Deduction> {
    match hidden_subset {
        Step::HiddenSubset { region: _, cells, values } => _get_deductions(grid, cells, values),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, hidden_subset: &Step<T>) -> String {
    match hidden_subset {
        Step::HiddenSubset { region, cells, values } => format!(
            "Hidden {} - {} in {} {}",
            get_subset_name(cells.len()), values, grid.region_name(region), grid.region_name(cells),
        ),
        _ => unreachable!(),
    }
}

fn get_subset_name<'a>(size: usize) -> &'a str {
    match size {
        2 => "Pair",
        3 => "Triple",
        4 => "Quad",
        _ => "Subset",
    }
}

fn _get_deductions<T: GridSize>(grid: &Grid<T>, cells: &CellSet<T>, candidates: &CandidateSet<T>) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Eliminate all other candidates from this group of cells.
    for cell in cells.iter() {
        let eliminations = grid.candidates(cell) & !candidates;
        for candidate in eliminations.iter() {
            let deduction = Deduction::Elimination(cell, candidate);
            deductions.push(deduction);
        }
    }

    deductions
}
