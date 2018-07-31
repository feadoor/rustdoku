//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::Grid;
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the hidden subsets, of the given degree, in the grid.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find_with_degree<'a>(grid: &'a Grid, degree: usize) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {
        // Iterate over all regions of the grid and all tuples of values.
        for region in Grid::regions() {
            for candidates in grid.missing_values_from_region(region).iter().combinations(degree).map(CandidateSet::from_candidates) {

                // Take the collection of cells which contain these candidates.
                let cells = grid.cells_with_candidates_in_region(&candidates, region);

                // Check if the candidates appear in the right number of cells and if any eliminations will occur.
                if cells.len() == degree {
                    if cells.iter().any(|cell| !(grid.candidates(cell) & !candidates).is_empty()) {
                        yield Step::HiddenSubset { region: *region, cells: cells, values: candidates };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the hidden subset on the given grid.
pub fn get_deductions(grid: &Grid, hidden_subset: &Step) -> Vec<Deduction> {
    match *hidden_subset {
        Step::HiddenSubset { region: _, cells, values } => _get_deductions(grid, &cells, &values),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(hidden_subset: &Step) -> String {
    match *hidden_subset {
        Step::HiddenSubset { region, cells, values } => format!(
            "Hidden {} - {} in {} {}",
            get_subset_name(cells.len()), values, Grid::region_name(&region), cells,
        ),
        _ => unreachable!(),
    }
}

fn get_subset_name<'a>(size: usize) -> &'a str {
    match size {
        2 => "Pair",
        3 => "Triple",
        4 => "Quad",
        _ => unreachable!(),
    }
}

fn _get_deductions(grid: &Grid, cells: &CellSet, candidates: &CandidateSet) -> Vec<Deduction> {

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