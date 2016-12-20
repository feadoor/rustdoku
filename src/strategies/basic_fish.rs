//! A definition of the fish technique.

use itertools::Itertools;

use grid::Grid;
use grid::cellset::CellSet;
use strategies::{Deduction, Move};
use strategies::outputs::BasicFish;

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
            let mov = fish_of_degree($x, $d);
            if mov.is_some() {
                return mov;
            }
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
        let mov = find_standard_fish(grid, degree, value, true);
        if mov.is_some() {
            return mov;
        }
        let mov = find_standard_fish(grid, degree, value, false);
        if mov.is_some() {
            return mov;
        }
    }

    for &value in Grid::values() {
        let mov = find_finned_fish(grid, degree, value, true);
        if mov.is_some() {
            return mov;
        }
        let mov = find_finned_fish(grid, degree, value, false);
        if mov.is_some() {
            return mov;
        }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_standard_fish(grid: &Grid, degree: usize, value: usize, rows: bool) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = if rows {
        Grid::rows().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    } else {
        Grid::columns().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    };

    // Iterate over all possible choices for the base rows / columns, looking for fish without fin
    // cells.
    for bases in base_sets.iter().combinations(degree) {
        let mut cover_sets = Vec::new();
        let mut base_union = CellSet::empty();
        let mut cover_union = CellSet::empty();

        // Build up the cover sets for this set of potential base sets.
        for &base in &bases {
            base_union |= base;

            let new_cells = base & !&cover_union;
            for cell in new_cells.iter() {
                let new_cover = if rows {
                    Grid::column(cell) & &candidate_positions
                } else {
                    Grid::row(cell) & &candidate_positions
                };
                cover_union |= &new_cover;
                cover_sets.push(new_cover);
            }
        }

        // If we have exactly as many cover sets as base sets, then we might have some fishy
        // eliminations on our hands.
        if cover_sets.len() == degree {
            let eliminations = cover_union & !base_union;
            let deductions: Vec<_> = eliminations.iter()
                .map(|ix| Deduction::Elimination(ix, value))
                .collect();
            if !deductions.is_empty() {

                // Get a human-readable description of the deduction and return it.
                let reason = BasicFish {
                    base: bases.iter().map(|&x| x.clone()).collect(),
                    value: value,
                    eliminations: eliminations,
                    finned: false,
                    rows: rows
                };
                return Some(Move {
                    deductions: deductions,
                    reason: Box::new(reason),
                });
            }
        }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_finned_fish(grid: &Grid, degree: usize, value: usize, rows: bool) -> Option<Move> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = if rows {
        Grid::rows().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    } else {
        Grid::columns().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    };

    // Iterate over all potential choices for the base sets, looking for finned fish.
    for bases in base_sets.iter().combinations(degree) {
        let mut cover_sets = Vec::new();
        let mut base_union = CellSet::empty();
        let mut cover_union = CellSet::empty();

        // Build up the cover sets for this set of potential base sets.
        for &base in &bases {
            base_union |= base;

            let new_cells = base & !&cover_union;
            for cell in new_cells.iter() {
                let new_cover = if rows {
                    Grid::column(cell) & &candidate_positions
                } else {
                    Grid::row(cell) & &candidate_positions
                };
                cover_union |= &new_cover;
                cover_sets.push(new_cover);
            }
        }

        // Iterate over all possible choices of covers that leave fins and check for eliminations.
        let num_fins = cover_sets.len() - degree;
        if num_fins == 1 || num_fins == 2 {
            let mut deductions = Vec::new();
            let mut elims = CellSet::empty();
            for ex_covers in cover_sets.iter().combinations(num_fins) {
                let uncovered = ex_covers.iter().fold(CellSet::empty(), |acc, &x| acc | x);
                let fins = &base_union & &uncovered;
                let fin_neighbours = fins.iter()
                    .fold(!CellSet::empty(), |acc, x| acc & Grid::neighbours(x));
                let eliminations = fin_neighbours & &cover_union & !(uncovered | &base_union);
                for cell_idx in eliminations.iter() {
                    deductions.push(Deduction::Elimination(cell_idx, value));
                    elims |= &eliminations;
                }
            }

            if !deductions.is_empty() {

                // Get a human-readable description of the deduction and return it.
                let reason = BasicFish {
                    base: bases.iter().map(|&x| x.clone()).collect(),
                    value: value,
                    eliminations: elims,
                    finned: true,
                    rows: rows
                };
                return Some(Move {
                    deductions: deductions,
                    reason: Box::new(reason),
                });
            }
        }
    }

    None
}
