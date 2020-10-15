//! A definition of the MSLS strategy

use itertools::chain;
use itertools::Itertools;

use grid::{Grid, GridSize};
use grid::RowOrColumn;
use grid::RowOrColumn::*;
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find MSLSs that appear in the grid.
///
/// An MSLS is when, considering the positions that a set of numbers can be
/// placed in a certain set of rows (or columns) of the grid, we can make
/// deductions about which cells they must be placed in.
pub fn find<'a, T: GridSize>(grid: &'a Grid<T>) -> impl Iterator<Item = Step<T>> + 'a {
    (2..T::size() + 1).cartesian_product(2..T::size()).flat_map(move |(base_deg, digit_deg)| {
        let row_msls = find_msls(grid, base_deg, digit_deg, Row);
        let col_msls = find_msls(grid, base_deg, digit_deg, Column);
        chain(row_msls, col_msls)
    })
}

/// Find, if it exists, an MSLS of the given degree
fn find_msls<'a, T: GridSize>(grid: &'a Grid<T>, base_degree: usize, digit_degree: usize, base_type: RowOrColumn) -> impl Iterator<Item = Step<T>> + 'a {

    GeneratorAdapter::of(move || {

        // Iterate over all possible choices for the base rows / columns, looking for an MSLS
        let all_bases = if base_type == Row { grid.rows() } else { grid.columns() };
        for base_sets in  all_bases.iter().combinations(base_degree) {
            let base_union = CellSet::union(&base_sets.iter().map(|&x| x.clone()).collect::<Vec<_>>());

            // Iterate over all subsets of possible digits
            for base_digits in (1..T::size() + 1).combinations(digit_degree) {

                // Count the number of times these digits need to be placed in the base rows or columns
                let missing_count: usize = base_sets.iter().map(|rc| base_digits.iter().map(|&d| if grid.value_placed_in_region(d, rc) { 0 } else { 1 }).sum::<usize>()).sum();

                // Count, in the crudest possible way, the maximum number of times that the digits can actually appear
                let mut single_cells = CellSet::empty();
                let mut cover_sets = Vec::new();
                let mut placement_count = 0;

                let all_covers = if base_type == Row { grid.columns() } else { grid.rows() };
                for cover in all_covers {
                    let base_intersection = cover & &base_union;
                    let digits_to_place: Vec<usize> = base_digits.iter().filter(|&&d| grid.candidate_in_region(d, &base_intersection)).map(|d| *d).collect();
                    if digits_to_place.len() > base_degree {
                        single_cells |= &base_intersection;
                        placement_count += base_degree;
                    } else if digits_to_place.len() == base_degree {
                        single_cells |= &base_intersection;
                        for &digit in &digits_to_place {
                            cover_sets.push((cover.clone(), digit));
                        }
                        placement_count += base_degree;
                    } else {
                        for &digit in &digits_to_place {
                            cover_sets.push((cover.clone(), digit));
                        }
                        placement_count += digits_to_place.len();
                    }
                }

                // If the two counts are equal, it's MSLS town!
                if missing_count == placement_count {
                    yield Step::Msls { base: base_sets.iter().map(|&x| x.clone()).collect(), digits: base_digits.clone(), single_cells, cover: cover_sets };
                }   
            }
        }        

    })
}

/// Get the deductions arising from the MSLS on the given grid.
pub fn get_deductions<T: GridSize>(grid: &Grid<T>, msls: &Step<T>) -> Vec<Deduction> {
    match msls {
        Step::Msls { base, digits, single_cells, cover } => {
            let base_union = CellSet::union(&base);
            let digit_set = CandidateSet::from_candidates(digits.iter().map(|x| *x));
            
            let mut deductions = Vec::new();

            for cell in single_cells.iter() {
                for eliminated_digit in (grid.candidates(cell) & (!digit_set)).iter() {
                    deductions.push(Deduction::Elimination(cell, eliminated_digit));
                }
            }

            for (cover_set, digit) in cover {
                for cell in (cover_set & !&base_union).iter() {
                    if grid.has_candidate(cell, *digit) {
                        deductions.push(Deduction::Elimination(cell, *digit));
                    }
                }
            }

            deductions
        },
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description<T: GridSize>(grid: &Grid<T>, msls: &Step<T>) -> String {
    match msls {
        Step::Msls { base, digits, .. } => {
            format!(
                "MSLS - on values ({}) with base ({})",
                digits.iter().join(", "),
                base.iter().map(|x| grid.region_name(x)).collect::<Vec<_>>().join(", "),
            )
        },
        _ => unreachable!(),
    }
}
