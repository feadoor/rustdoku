//! A definition of the naked subset strategy.

use itertools::Itertools;

use grid::Grid;
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};

/// Return, if one exists, a naked subset of the given degree.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Step> {

    // Iterate over all tuples of empty cells from regions of the grid.
    for region in Grid::regions() {
        for cells in grid.empty_cells_in_region(region).iter().combinations(degree).map(CellSet::from_cells) {

            // Take the union of the candidates found in these cells.
            let candidates = grid.all_candidates_from_region(&cells);

            // Check if the right number of candidates appear and if any eliminations will occur.
            if candidates.len() == degree {
                if cells.common_neighbours().iter().any(|cell| !(grid.candidates(cell) & candidates).is_empty()) {
                    return Some(Step::NakedSubset { region: *region, cells: cells, values: candidates });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the hidden single on the given grid.
pub fn get_deductions(grid: &Grid, naked_subset: &Step) -> Vec<Deduction> {
    match *naked_subset {
        Step::NakedSubset { region: _, cells, values } => _get_deductions(grid, &cells, &values),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(naked_subset: &Step) -> String {
    match *naked_subset {
        Step::NakedSubset { region, cells, values } => format!(
            "Naked {} - {} in {} ({})",
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


/// Build up the deductions resulting from a naked subset.
fn _get_deductions(grid: &Grid, cells: &CellSet, candidates: &CandidateSet) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    for cell in cells.common_neighbours().iter() {
        for val in candidates.iter() {
            if grid.has_candidate(cell, val) {
                deductions.push(Deduction::Elimination(cell, val));
            }
        }
    }

    deductions
}