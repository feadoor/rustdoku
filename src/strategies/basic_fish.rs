//! A definition of the fish strategy.

use itertools::chain;
use itertools::Itertools;

use grid::{Grid, GridSize};
use grid::RowOrColumn;
use grid::RowOrColumn::*;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the fish of the given degree that appear in the grid.
///
/// A fish is when, within n rows (columns), all occurrences of a particular digit can be covered
/// by n columns (rows). Then all other occurrences of that digit within the cover columns (rows)
/// can be eliminated.
pub fn find_with_degree<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize) -> impl Iterator<Item = Step<T>> + 'a {

    grid.values().into_iter().flat_map(move |value| {
        let row_fish = find_fish(grid, degree, value, Row);
        let col_fish = find_fish(grid, degree, value, Column);
        chain(row_fish, col_fish)
    })
}

/// Find, if it exists, a fish of the given degree with the given value in the grid.
fn find_fish<'a, T: GridSize>(grid: &'a Grid<T>, degree: usize, value: usize, base_type: RowOrColumn) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(#[coroutine] move || {

        // Generate all possible base sets for this fish.
        let candidate_positions = grid.cells_with_candidate(value);
        let base_sets: Vec<CellSet<T>> = grid.group_cells_by(&candidate_positions, base_type);

        // Iterate over all possible choices for the base rows / columns, looking for fish without fin
        // cells.
        for bases in base_sets.into_iter().combinations(degree) {

            // Build up the cover sets for this set of potential base sets.
            let base_union = CellSet::union(&bases);
            let cover_sets = match base_type {
                Row => grid.intersecting_columns(&base_union),
                Column => grid.intersecting_rows(&base_union),
            };

            // If we have exactly as many cover sets as base sets, then we might have some fishy
            // eliminations on our hands.
            if cover_sets.len() == degree {
                let cover_union = CellSet::union(&cover_sets) & &candidate_positions;
                if !(&cover_union & !&base_union).is_empty() {
                    yield Step::Fish { degree, base_type, base: base_union.clone(), cover: cover_union.clone(), value };
                }
            }
        }
    })
}

/// Get the deductions arising from the fish on the given grid.
pub fn get_deductions<T: GridSize>(_grid: &Grid<T>, fish: &Step<T>) -> Vec<Deduction> {
    match fish {
        Step::Fish { base, cover, value, .. } => (cover & !base)
            .map(|cell| Deduction::Elimination(cell, *value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, fish: &Step<T>) -> String {
    match fish {
        Step::Fish { base_type, base, cover, value, .. } => {
            let base_regions = get_base_regions(grid, *base_type, base);
            let cover_regions = get_cover_regions(grid, *base_type, cover);
            format!(
                "{} - on value {} with base ({}) and cover ({})",
                get_fish_name(base_regions.len()),
                value,
                base_regions.iter().map(|x| grid.region_name(x)).collect::<Vec<_>>().join(", "),
                cover_regions.iter().map(|x| grid.region_name(x)).collect::<Vec<_>>().join(", "),
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
        _ => "Fish",
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
