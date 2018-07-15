//! A definition of the fish technique.

use itertools::Itertools;

use grid::Grid;
use grid::Region;
use grid::Region::*;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};

/// Return, if one exists, a finned fish of the given degree.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
///
/// This pattern is now extended to allow for 'fin cells' - cells in the original rows (columns)
/// which are not covered by the columns (rows). Then only the eliminated digits which can also
/// see all fin cells are valid.
pub fn find_with_degree(grid: &Grid, degree: usize) -> Option<Step> {

    for &value in Grid::values() {
        if let Some(mov) = find_finned_fish(grid, degree, value, Row) { return Some(mov); }
        if let Some(mov) = find_finned_fish(grid, degree, value, Column) { return Some(mov); }
    }

    None
}

/// Find, if it exists, a finned fish of the given degree with the given value in the grid.
fn find_finned_fish(grid: &Grid, degree: usize, value: usize, base_type: Region) -> Option<Step> {

    // Generate all possible base sets for this fish.
    let candidate_positions = grid.cells_with_candidate(value);
    let base_sets: Vec<CellSet> = candidate_positions.group_by(base_type);

    // Iterate over all potential choices for the base sets, looking for finned fish.
    for bases in base_sets.iter().combinations(degree) {

        // Build up the cover sets for this set of potential base sets.
        let base_union = CellSet::union(&bases);
        let cover_sets = match base_type {
            Row => Grid::intersecting_columns(&base_union),
            Column => Grid::intersecting_rows(&base_union),
            Block => unreachable!(),
        };

        // Iterate over all possible choices of covers that leave fins and check for eliminations.
        let num_fins = cover_sets.len() - degree;
        if num_fins == 1 || num_fins == 2 {
            let full_cover = CellSet::union(&cover_sets) & &candidate_positions;
            for ex_covers in cover_sets.iter().map(|c| *c).combinations(num_fins) {
                let uncovered = CellSet::union(&ex_covers);
                let cover_union = full_cover & !uncovered;
                let fins = &base_union & &uncovered;
                if !(fins.common_neighbours() & &cover_union & !base_union).is_empty() {
                    return Some(Step::FinnedFish { degree, base_type, base: base_union, cover: cover_union, fins, value });
                }
            }
        }
    }

    None
}

/// Get the deductions arising from the finned fish on the given grid.
pub fn get_deductions(_grid: &Grid, finned_fish: &Step) -> Vec<Deduction> {
    match *finned_fish {
        Step::FinnedFish { base, cover, fins, value, .. } => (fins.common_neighbours() & &cover & !base)
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(finned_fish: &Step) -> String {
    match *finned_fish {
        Step::FinnedFish { base_type, base, cover, fins, value, .. } => {
            let base_regions = get_base_regions(base_type, &base);
            let cover_regions = get_cover_regions(base_type, &cover);
            format!(
                "Finned {} - on value {} with base ({}), cover ({}) and fins {}",
                get_fish_name(base_regions.len()),
                value,
                base_regions.iter().map(|x| Grid::region_name(x)).collect::<Vec<_>>().join(", "),
                cover_regions.iter().map(|x| Grid::region_name(x)).collect::<Vec<_>>().join(", "),
                fins,
            )
        },
        _ => unreachable!(),
    }
}

fn get_fish_name<'a>(size: usize) -> &'a str {
    match size {
        2 => "X-Wing",
        3 => "Swordfish",
        4 => "Jellyfish",
        _ => unreachable!(),
    }
}

fn get_base_regions(base_type: Region, base_union: &CellSet) -> Vec<&CellSet> {
    match base_type {
        Row => Grid::intersecting_rows(base_union),
        Column => Grid::intersecting_columns(base_union),
        _ => unreachable!(),
    }
}

fn get_cover_regions(base_type: Region, cover_union: &CellSet) -> Vec<&CellSet> {
    match base_type {
        Row => Grid::intersecting_columns(cover_union),
        Column => Grid::intersecting_rows(cover_union),
        _ => unreachable!(),
    }
}