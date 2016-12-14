//! A definition of the hidden subset strategy.

use itertools::Itertools;

use grid::{Grid, LARGE_SIZE};
use grid::cell::Cell;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a hidden subset.
///
/// A hidden subset is when, in a particular region, n values can only appear in n cells. Then
/// other values can be eliminated from those cells.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {

    macro_rules! find_subsets {
        ($d: expr, $x: ident) => {
            let deductions = find_with_degree($x, $d);
            if deductions.is_some() {
                return deductions;
            }
        }
    }

    find_subsets!(2, grid);
    find_subsets!(3, grid);
    find_subsets!(4, grid);

    None
}

/// Find a hidden subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Vec<Deduction>> {

    // Iterate over all regions of the grid and all tuples of values.
    for region in grid.regions() {

        // Find the values which are missing from the region.
        let mut missing_vals = vec![true; LARGE_SIZE];
        for cell in region.cells().iter() {
            if let Some(val) = cell.value() {
                missing_vals[val - 1] = false;
            }
        }

        // Iterate over tuples of the missing values.
        for tuple in missing_vals.iter()
            .enumerate()
            .filter(|&(_, &x)| x)
            .map(|(idx, _)| idx + 1)
            .combinations(degree) {

            // Take the union of the cells which contain these candidates.
            let mut cells = Vec::new();
            for &candidate in tuple.iter() {
                for cell in region.potential_cells(candidate) {
                    if cells.iter().find(|&&x| x == cell).is_none() {
                        cells.push(cell);
                    }
                }
            }

            // Check if the candidates appear in the right number of cells.
            if cells.len() == degree {
                let deductions = get_deductions(tuple, cells);
                if !deductions.is_empty() {
                    return Some(deductions);
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a hidden subset.
fn get_deductions(tuple: Vec<usize>, cells: Vec<&Cell>) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // Eliminate all other candidates from this group of cells.
    for cell in cells {
        for value in cell.candidates()
            .iter()
            .filter(|&d| tuple.iter().find(|&&x| x == d).is_none()) {

            let deduction = Deduction::Elimination(cell.idx(), value);
            deductions.push(deduction);
        }
    }

    deductions
}