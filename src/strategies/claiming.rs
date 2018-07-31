//! A definition of the claiming strategy.

use grid::Grid;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

/// Find the instances of claiming that appear in the grid.
///
/// Claiming occurs when all occurrences of a given value within a row or column occur within a
/// single block. Then that value can be eliminated from other positions within the block.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {
        // Scan each row / column, and for each value, check if the positions are limited to a
        // particular block.
        for region in Grid::rows().iter().chain(Grid::columns().iter()) {
            for &val in Grid::values() {
                let cells = grid.cells_with_candidate_in_region(val, region);
                if cells.len() < 2 { continue; }

                if let Some(block) = Grid::block_containing(&cells) {
                    if !grid.cells_with_candidate_in_region(val, &(block & !region)).is_empty() {
                        yield Step::Claiming { region: *region, block: block, value: val };

                    }
                }
            }
        }
    })
}

/// Get the deductions arising from the claiming on the given grid.
pub fn get_deductions(grid: &Grid, claiming: &Step) -> Vec<Deduction> {
    match *claiming {
        Step::Claiming { region, block, value } => grid
            .cells_with_candidate_in_region(value, &(block & !region))
            .map(|cell| Deduction::Elimination(cell, value)),
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(claiming: &Step) -> String {
    match *claiming {
        Step::Claiming { region, block, value } => format!(
            "Claiming - the {}s in {} eliminate further {}s from {}",
            value, Grid::region_name(&region), value, Grid::region_name(&block),
        ),
        _ => unreachable!(),
    }
}