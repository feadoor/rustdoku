//! A definition of the pointing strategy.

use grid::Grid;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the instances of pointing that appear in the grid.
///
/// Pointing occurs when all occurrences of a given value within a block occur within a single
/// row or column. Then other occurrences of that value can be removed from other cells in the row
/// or column.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {
        // Scan each block, and for each value, check if the positions are limited to a row or column.
        for block in Grid::blocks() {
            for &val in Grid::values() {
                let cells = grid.cells_with_candidate_in_region(val, block);
                if cells.len() < 2 { continue; }

                if let Some(intersection) = Grid::row_containing(&cells).or(Grid::column_containing(&cells)) {
                    if !grid.cells_with_candidate_in_region(val, &(intersection & !block)).is_empty() {
                        yield Step::Pointing { block: *block, region: intersection, value: val };
                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the pointing on the given grid.
pub fn get_deductions(grid: &Grid, pointing: &Step) -> Vec<Deduction> {
    match *pointing {
        Step::Pointing { block, region, value } => grid
            .cells_with_candidate_in_region(value, &(region & !block))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(pointing: &Step) -> String {
    match *pointing {
        Step::Pointing { block, region, value } => format!(
            "Pointing - the {}s in {} eliminate further {}s from {}",
            value, Grid::region_name(&block), value, Grid::region_name(&region),
        ),
        _ => unreachable!(),
    }
}