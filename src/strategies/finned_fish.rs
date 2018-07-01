//! A definition of the fish technique.

use itertools::Itertools;

use grid::Grid;
use grid::Region;
use grid::Region::*;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a finned fish of the given degree.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
///
/// This pattern is now extended to allow for 'fin cells' - cells in the original rows (columns)
/// which are not covered by the columns (rows). Then only the eliminated digits which can also
/// see all fin cells are valid.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Move> {

    for &value in Grid::values() {
        if let Some(mov) = find_finned_fish(grid, degree, value, Row) { return Some(mov); }
        if let Some(mov) = find_finned_fish(grid, degree, value, Column) { return Some(mov); }
    }

    None
}

/// Find, if it exists, a finned fish of the given degree with the given value in the grid.
fn find_finned_fish(grid: &Grid, degree: usize, value: usize, base_type: Region) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = candidate_positions.group_by(base_type);

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

            let cover_union = cover_sets.iter().fold(CellSet::empty(), |acc, &cover| acc | cover) & &candidate_positions;
            for ex_covers in cover_sets.iter().combinations(num_fins) {
                let uncovered = ex_covers.iter().fold(CellSet::empty(), |acc, &&x| acc | x);
                let fins = &base_union & &uncovered;
                let fin_neighbours = fins.common_neighbours();
                for cell_idx in (fin_neighbours & &cover_union & !(uncovered | &base_union)).iter() {
                    deductions.push(Deduction::Elimination(cell_idx, value));
                }
            }

            if !deductions.is_empty() {
                return Some(Move {
                    deductions: deductions,
                    description: format!(
                        "Finned fish on {}s in ({})", value,
                        (match base_type {
                            Row => Grid::intersecting_rows(&base_union),
                            Column => Grid::intersecting_columns(&base_union),
                            Block => unreachable!(),
                        }).iter().map(|&x| Grid::region_name(x)).collect::<Vec<String>>().join(", ")
                    ),
                });
            }
        }
    }

    None
}
