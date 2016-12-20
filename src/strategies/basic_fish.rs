//! A definition of the fish technique.

use grid::Grid;
use grid::cellset::CellSet;
use strategies::Deduction;

use itertools::Itertools;

/// Return, if one exists, a deduction based on a fish.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
///
/// This pattern can be extended to allow for 'fin cells' - cells in the original rows (columns)
/// which are not covered by the columns (rows). Then only the eliminated digits which can also
/// see all fin cells are valid.
pub fn find(grid: &Grid) -> Option<Vec<Deduction>> {
    macro_rules! find_fish {
        ($d: expr, $x :ident) => {
            let deductions = fish_of_degree($x, $d);
            if deductions.is_some() {
                return deductions;
            }
        }
    }

    find_fish!(2, grid);
    find_fish!(3, grid);
    find_fish!(4, grid);

    None
}

/// Find a fish elimination of the given degree.
fn fish_of_degree(grid: &Grid, degree: usize) -> Option<Vec<Deduction>> {
    for &value in Grid::values() {
        let deductions = find_fish(grid, degree, value, true);
        if deductions.is_some() {
            return deductions;
        }
        let deductions = find_fish(grid, degree, value, false);
        if deductions.is_some() {
            return deductions;
        }
    }

    None
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_fish(grid: &Grid, degree: usize, value: usize, rows: bool) -> Option<Vec<Deduction>> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = if rows {
        Grid::rowsets().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    } else {
        Grid::colsets().iter()
            .map(|set| set & &candidate_positions)
            .filter(|x| !x.is_empty())
            .collect()
    };

    // Iterate over all possible choices for the base rows / columns. First, look for fish without
    // fins.
    for bases in base_sets.iter().combinations(degree) {
        let mut cover_sets = Vec::new();
        let mut base_union = CellSet::empty();
        let mut cover_union = CellSet::empty();

        // Build up the cover sets for this set of potential base sets.
        for base in bases {
            base_union |= base;

            let new_cells = base & !&cover_union;
            for cell in new_cells.iter() {
                let new_cover = if rows {
                    Grid::colset(cell) & &candidate_positions
                } else {
                    Grid::rowset(cell) & &candidate_positions
                };
                cover_union |= &new_cover;
                cover_sets.push(new_cover);
            }
        }

        // If we have exactly as many cover sets as base sets, then we might have some fishy
        // eliminations on our hands.
        if cover_sets.len() == degree {
            let deductions: Vec<_> = (cover_union & !base_union).iter()
                .map(|ix| Deduction::Elimination(ix, value))
                .collect();
            if !deductions.is_empty() {
                return Some(deductions);
            }
        }
    }

    // Now, look for finned fish.
    for bases in base_sets.iter().combinations(degree) {
        let mut cover_sets = Vec::new();
        let mut base_union = CellSet::empty();
        let mut cover_union = CellSet::empty();

        // Build up the cover sets for this set of potential base sets.
        for base in bases {
            base_union |= base;

            let new_cells = base & !&cover_union;
            for cell in new_cells.iter() {
                let new_cover = if rows {
                    Grid::colset(cell) & &candidate_positions
                } else {
                    Grid::rowset(cell) & &candidate_positions
                };
                cover_union |= &new_cover;
                cover_sets.push(new_cover);
            }
        }

        // The number of 'extra' columns that we have is equal to the number of fins. Iterate over
        // all possible choices of columns that leave fins and check for eliminations.
        let num_fins = cover_sets.len() - degree;
        if num_fins == 1 || num_fins == 2 {
            let mut deductions = Vec::new();
            for ex_covers in cover_sets.iter().combinations(num_fins) {
                let uncovered = ex_covers.iter().fold(CellSet::empty(), |acc, &x| acc | x);
                let fins = &base_union & &uncovered;
                let fin_neighbours = fins.iter()
                    .fold(!CellSet::empty(), |acc, x| acc & Grid::neighbours_set(x));
                let eliminations = fin_neighbours & &cover_union & !(uncovered | &base_union);
                for cell_idx in eliminations.iter() {
                    deductions.push(Deduction::Elimination(cell_idx, value));
                }
            }

            if !deductions.is_empty() {
                return Some(deductions);
            }
        }
    }

    None
}
