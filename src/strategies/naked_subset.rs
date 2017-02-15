//! A definition of the naked subset strategy.

use itertools::Itertools;

use grid::{CellIdx, Grid};
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a naked subset.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
pub fn find(grid: &Grid) -> Option<Move> {

    macro_rules! find_subsets {
        ($d: expr, $x: ident) => {
            let mov = find_with_degree($x, $d);
            if mov.is_some() {
                return mov;
            }
        }
    }

    find_subsets!(2, grid);
    find_subsets!(3, grid);
    find_subsets!(4, grid);

    None
}

/// Find a naked subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Move> {

    // Iterate over all tuples of empty cells from regions of the grid.
    for region in Grid::regions() {
        for cells in grid.empty_cells_in_region(region).iter().combinations(degree).map(CellSet::from_cells) {

            // Take the union of the candidates found in these cells.
            let candidates = grid.all_candidates_from_region(&cells);

            // Check if the right number of candidates appear.
            if candidates.len() as usize == degree {
                let deductions = get_deductions(grid, &cells, &candidates);
                if !deductions.is_empty() {
                    return Some(Move { deductions: deductions });
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a naked subset.
fn get_deductions(grid: &Grid, cells: &CellSet, mut candidates: &CandidateSet) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    for cell in Grid::common_neighbours(cells).iter() {
        for val in candidates.iter() {
            if grid.has_candidate(cell, val) {
                deductions.push(Deduction::Elimination(cell, val));
            }
        }
    }

    deductions
}