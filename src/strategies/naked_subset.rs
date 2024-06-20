//! A definition of the naked subset strategy.

use itertools::Itertools;

use grid::{Grid, GridSize};
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the naked subsets, of the given degree, that appear in the grid.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
pub fn find_with_degree<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // Iterate over all tuples of empty cells from regions of the grid.
        for region in grid.all_regions() {
            for cells in grid.empty_cells_in_region(region).iter().combinations(degree).map(CellSet::from_cells) {

                // Take the union of the candidates found in these cells.
                let candidates = grid.all_candidates_from_region(&cells);

                // Check if the right number of candidates appear and if any eliminations will occur.
                if candidates.len() == degree {
                    if grid.common_neighbours(&cells).iter().any(|cell| !(grid.candidates(cell) & candidates).is_empty()) {
                        yield Step::NakedSubset { region: region.clone(), cells: cells.clone(), values: candidates };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the hidden single on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, naked_subset: &Step<T>) -> Vec<Deduction> {
    match naked_subset {
        Step::NakedSubset { cells, values, .. } => _get_deductions(grid, cells, values),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, naked_subset: &Step<T>) -> String {
    match naked_subset {
        Step::NakedSubset { region, cells, values } => format!(
            "Naked {} - {} in {} {}",
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


/// Build up the deductions resulting from a naked subset.
fn _get_deductions<T: GridSize>(grid: &Grid<T>, cells: &CellSet<T>, candidates: &CandidateSet<T>) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    for cell in grid.common_neighbours(cells).iter() {
        for val in candidates.iter() {
            if grid.has_candidate(cell, val) {
                deductions.push(Deduction::Elimination(cell, val));
            }
        }
    }

    deductions
}
