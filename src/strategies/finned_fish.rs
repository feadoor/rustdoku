//! A definition of the finned fish strategy.

use itertools::chain;
use itertools::Itertools;

use grid::{Grid, GridSize};
use grid::RowOrColumn;
use grid::RowOrColumn::*;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the finned fish of the given degree that appear in the grid.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
///
/// This pattern is now extended to allow for 'fin cells' - cells in the original rows (columns)
/// which are not covered by the columns (rows). Then only the eliminated digits which can also
/// see all fin cells are valid.
pub fn find_with_degree<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize) -> impl Iterator<Item = Step<T>> + 'a {

    grid.values().into_iter().flat_map(move |value| {
        let row_fish = find_finned_fish(grid, degree, value, Row);
        let col_fish = find_finned_fish(grid, degree, value, Column);
        chain(row_fish, col_fish)
    })
}

/// Find, if it exists, a finned fish of the given degree with the given value in the grid.
fn find_finned_fish<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize, value: usize, base_type: RowOrColumn) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Generate all possible base sets for this fish.
        let candidate_positions = grid.cells_with_candidate(value);
        let base_sets: Vec<CellSet<T>> = grid.group_cells_by(&candidate_positions, base_type);

        // Iterate over all potential choices for the base sets, looking for finned fish.
        for bases in base_sets.into_iter().combinations(degree) {

            // Build up the cover sets for this set of potential base sets.
            let base_union = CellSet::union(&bases);
            let cover_sets = match base_type {
                Row => grid.intersecting_columns(&base_union),
                Column =>grid.intersecting_rows(&base_union),
            };

            // Iterate over all possible choices of covers that leave fins and check for eliminations.
            let num_fins = cover_sets.len() - degree;
            let full_cover = CellSet::union(&cover_sets) & &candidate_positions;
            if num_fins > 0 {
                for ex_covers in cover_sets.into_iter().combinations(num_fins) {
                    let uncovered = CellSet::union(&ex_covers);
                    let cover_union = &full_cover & !(&uncovered);
                    let fins = &base_union & &uncovered;
                    if !(grid.common_neighbours(&fins) & &cover_union & !(&base_union)).is_empty() {
                        yield Step::FinnedFish { degree, base_type, base: base_union.clone(), cover: cover_union, fins, value };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the finned fish on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, finned_fish: &Step<T>) -> Vec<Deduction> {
    match finned_fish {
        Step::FinnedFish { base, cover, fins, value, .. } => (grid.common_neighbours(fins) & cover & !base)
            .map(|cell| Deduction::Elimination(cell, *value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, finned_fish: &Step<T>) -> String {
    match finned_fish {
        Step::FinnedFish { base_type, base, cover, fins, value, .. } => {
            let base_regions = get_base_regions(grid, *base_type, base);
            let cover_regions = get_cover_regions(grid, *base_type, cover);
            format!(
                "Finned {} - on value {} with base ({}), cover ({}) and fins {}",
                get_fish_name(base_regions.len()),
                value,
                base_regions.iter().map(|x| grid.region_name(x)).collect::<Vec<_>>().join(", "),
                cover_regions.iter().map(|x| grid.region_name(x)).collect::<Vec<_>>().join(", "),
                grid.region_name(&fins),
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

fn get_base_regions<T: GridSize>(grid: &Grid<T>, base_type: RowOrColumn, base_union: &CellSet<T>) -> Vec<CellSet<T>> {
    match base_type {
        Row => grid.intersecting_rows(base_union),
        Column => grid.intersecting_columns(base_union),
    }
}

fn get_cover_regions<T: GridSize>(grid: &Grid<T>, base_type: RowOrColumn, cover_union: &CellSet<T>) -> Vec<CellSet<T>> {
    match base_type {
        Row => grid.intersecting_columns(cover_union),
        Column => grid.intersecting_rows(cover_union),
    }
}
