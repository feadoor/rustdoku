//! A definition of the fish technique.

use itertools::Itertools;

use grid::Grid;
use grid::cellset::CellSet;
use grid::regions::Region;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a fish.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
///
/// This pattern can be extended to allow for 'fin cells' - cells in the original rows (columns)
/// which are not covered by the columns (rows). Then only the eliminated digits which can also
/// see all fin cells are valid.
pub fn find(grid: &Grid) -> Option<Move> {
    macro_rules! find_fish {
        ($d: expr, $x :ident) => {
            if let Some(mov) = fish_of_degree($x, $d) { return Some(mov); }
        }
    }

    find_fish!(2, grid);
    find_fish!(3, grid);
    find_fish!(4, grid);

    None
}

/// Find a fish elimination of the given degree.
fn fish_of_degree(grid: &Grid, degree: usize) -> Option<Move> {
    for &value in Grid::values() {
        if let Some(mov) = find_standard_fish(grid, degree, value, true) { return Some(mov); }
        if let Some(mov) = find_standard_fish(grid, degree, value, false) { return Some(mov); }
    }

    for &value in Grid::values() {
        if let Some(mov) = find_finned_fish(grid, degree, value, true) { return Some(mov); }
        if let Some(mov) = find_finned_fish(grid, degree, value, false) { return Some(mov); }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_standard_fish(grid: &Grid, degree: usize, value: usize, base_type: Region) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets = candidate_positions.group_by(base_type);

    // Iterate over all possible choices for the base rows / columns, looking for fish without fin
    // cells.
    for bases in base_sets.iter().combinations(degree) {

        // Build up the cover sets for this set of potential base sets.
        let base_union = bases.iter().fold(CellSet::empty(), |acc, &base| acc | base);
        let cover_sets = match base_type {
            Row => Grid::intersecting_columns(&base_union),
            Column => Grid::intersecting_rows(&base_union),
            Block => unreachable!(),
        };

        // If we have exactly as many cover sets as base sets, then we might have some fishy
        // eliminations on our hands.
        if cover_sets.len() == degree {
            let cover_union = cover_sets.iter().fold(CellSet::empty(), |acc, &cover| acc | cover);
            let eliminations = cover_union & !base_union;
            let deductions: Vec<_> = eliminations.iter()
                .map(|ix| Deduction::Elimination(ix, value))
                .collect();
            if !deductions.is_empty() {
                return Some(Move { deductions: deductions });
            }
        }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_finned_fish(grid: &Grid, degree: usize, value: usize, base_type: Region) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets = candidate_positions.group_by(base_type);

    // Iterate over all potential choices for the base sets, looking for finned fish.
    for bases in base_sets.iter().combinations(degree) {

        // Build up the cover sets for this set of potential base sets.
        let base_union = bases.iter().fold(CellSet::empty(), |acc, &base| acc | base);
        let cover_sets = match base_type {
            Row => Grid::intersecting_columns(&base_union),
            Column => Grid::intersecting_rows(&base_union),
            Block => unreachable!(),
        };

        // Iterate over all possible choices of covers that leave fins and check for eliminations.
        let num_fins = cover_sets.len() - degree;
        if num_fins == 1 || num_fins == 2 {
            let mut deductions = Vec::new();
            let mut elims = CellSet::empty();

            let cover_union = cover_sets.iter().fold(CellSet::empty(), |acc, &cover| acc | cover);
            for ex_covers in cover_sets.iter().combinations(num_fins) {
                let uncovered = ex_covers.iter().fold(CellSet::empty(), |acc, &&x| acc | x);
                let fins = &base_union & &uncovered;
                let fin_neighbours = fins.all_neighbours();
                let eliminations = fin_neighbours & &cover_union & !(uncovered | &base_union);
                for cell_idx in eliminations.iter() {
                    deductions.push(Deduction::Elimination(cell_idx, value));
                    elims |= &eliminations;
                }
            }

            if !deductions.is_empty() {
                return Some(Move { deductions: deductions });
            }
        }
    }

    None
}
