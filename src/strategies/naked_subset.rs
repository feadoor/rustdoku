//! A definition of the naked subset strategy.

use bit_set::BitSet;
use itertools::Itertools;

use grid::Grid;
use grid::cell::Cell;
use strategies::Deduction;

/// Return, if one exists, a deduction based on a naked subset.
///
/// A naked subset is when, in a particular region, n cells can only hold, between them, n
/// different values. Then those values can be eliminated from elsewhere in the region.
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

/// Find a naked subset of the given degree in the given region.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Vec<Deduction>> {

    // Iterate over all tuples of empty cells from regions of the grid.
    for region in grid.regions() {
        for tuple in region.cells()
            .iter()
            .filter(|x| x.is_empty())
            .combinations(degree) {

            // Take the union of the candidates found in these cells.
            let mut candidates = BitSet::new();
            for cell in tuple.iter() {
                candidates.union_with(cell.candidates());
            }

            // Check if the right number of candidates appear.
            if candidates.len() == degree {
                let deductions = get_deductions(grid, tuple, candidates);
                if !deductions.is_empty() {
                    return Some(deductions);
                }
            }
        }
    }

    None
}

/// Build up the deductions resulting from a naked subset.
fn get_deductions(grid: &Grid, tuple: Vec<&&Cell>, candidates: BitSet) -> Vec<Deduction> {

    let mut deductions = Vec::new();

    // If the cells are all in the same row, then eliminate the candidates from other cells in
    // that row.
    if tuple.iter().all(|x| x.row() == tuple[0].row()) {
        for cell in grid.row_from_cell(tuple[0].idx()).cells().iter() {
            for value in candidates.iter() {
                if cell.has_candidate(value) {
                    if tuple.iter().find(|&&x| x == cell).is_none() {
                        for value in candidates.iter() {
                            deductions.push(Deduction::Elimination(cell.idx(), value));
                        }
                    }
                }
            }
        }
    }

    // If the cells are all in the same columns, then eliminate the candidates from other cells in
    // that columns.
    if tuple.iter().all(|x| x.column() == tuple[0].column()) {
        for cell in grid.column_from_cell(tuple[0].idx()).cells().iter() {
            for value in candidates.iter() {
                if cell.has_candidate(value) {
                    if tuple.iter().find(|&&x| x == cell).is_none() {
                        deductions.push(Deduction::Elimination(cell.idx(), value));
                    }
                }
            }
        }
    }

    // If the cells are all in the same block, then eliminate the candidates from other cells in
    // that block.
    if tuple.iter().all(|x| x.block() == tuple[0].block()) {
        for cell in grid.block_from_cell(tuple[0].idx()).cells().iter() {
            for value in candidates.iter() {
                if cell.has_candidate(value) {
                    if tuple.iter().find(|&&x| x == cell).is_none() {
                        deductions.push(Deduction::Elimination(cell.idx(), value));
                    }
                }
            }
        }
    }

    deductions
}