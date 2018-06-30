//! A definition of the fish technique.

use itertools::Itertools;

use grid::Grid;
use grid::Region;
use grid::Region::*;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};

/// Return, if one exists, a deduction based on a fish of the given degree.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Move> {

    for &value in Grid::values() {
        if let Some(mov) = find_fish(grid, degree, value, Row) { return Some(mov); }
        if let Some(mov) = find_fish(grid, degree, value, Column) { return Some(mov); }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_fish(grid: &Grid, degree: usize, value: usize, base_type: Region) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = candidate_positions.group_by(base_type);

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
            let cover_union = cover_sets.iter().fold(CellSet::empty(), |acc, &cover| acc | cover) & &candidate_positions;
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
