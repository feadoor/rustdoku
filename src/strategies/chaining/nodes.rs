//! A definition of the nodes that can form part of a chain.

use grid::{Candidate, CellIdx, Grid, GridSize};
use grid::cellset::CellSet;

use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ChainNode<T: GridSize> {
    Value { cell: CellIdx, value: Candidate },
    Group { cells: CellSet<T>, value: Candidate },
    Als { cells: CellSet<T>, cells_with_value: CellSet<T>, value: Candidate },
}

impl <T: GridSize> ChainNode<T> {

    /// Get a readable description of a `ChainNode`
    pub fn get_description(&self, grid: &Grid<T>) -> String {
        match self {
            ChainNode::Value { cell, value } => format!("{}{}", value, grid.cell_name(*cell)),
            ChainNode::Group { cells, value, .. } => format!("{}{}", value, grid.region_name(cells)),
            ChainNode::Als { cells, value, .. } => format!("{}{}", value, grid.region_name(cells)),
        }
    }
}

/// Determine if the two nodes are linked in such a way that the truth
/// of the first implies the falsity of the second
pub fn is_linked_on_to_off<T: GridSize>(grid: &Grid<T>, start_node: &ChainNode<T>, end_node: &ChainNode<T>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { .. }, ChainNode::Value { .. }) => is_linked_value_on_value_off(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Group { .. }) => is_linked_value_on_group_off(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Als { .. }) => is_linked_value_on_als_off(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Value { .. }) => is_linked_group_on_value_off(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Group { .. }) => is_linked_group_on_group_off(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Als { .. }) => is_linked_group_on_als_off(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Value { .. }) => is_linked_als_on_value_off(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Group { .. }) => is_linked_als_on_group_off(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Als { .. }) => is_linked_als_on_als_off(grid, start_node, end_node),
    }
}

/// Determine if the two nodes are linked in such a way that the falsity
/// of the first implies the truth of the second
pub fn is_linked_off_to_on<T: GridSize>(grid: &Grid<T>, start_node: &ChainNode<T>, end_node: &ChainNode<T>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { .. }, ChainNode::Value { .. }) => is_linked_value_off_value_on(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Group { .. }) => is_linked_value_off_group_on(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Als { .. }) => false,
        (ChainNode::Group { .. }, ChainNode::Value { .. }) => is_linked_group_off_value_on(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Group { .. }) => is_linked_group_off_group_on(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Als { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Value { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Group { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Als { .. }) => is_linked_als_off_als_on(grid, start_node, end_node),
    }
}

/// Get all `Value` chain nodes for a given candidate from the given grid.
pub fn get_value_nodes_for_candidate<T: GridSize>(grid: &Grid<T>, candidate: Candidate) -> Vec<ChainNode<T>> {

    let mut value_nodes = Vec::new();

    for cell in grid.cells_with_candidate(candidate).iter() {
        value_nodes.push(ChainNode::Value { cell, value: candidate });
    }

    value_nodes
}

/// Get all `Group` chain nodes for a given candidate from the given grid.
pub fn get_group_nodes_for_candidate<T: GridSize>(grid: &Grid<T>, candidate: Candidate) -> Vec<ChainNode<T>> {

    let mut group_nodes = Vec::new();

    for (region1, region2) in grid.all_regions().iter().tuple_combinations() {
        let intersection = region1 & region2;
        let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
        if cells_with_candidate.len() > 1 {
            group_nodes.push(ChainNode::Group { cells: cells_with_candidate, value: candidate });
        }
    }

    group_nodes
}

/// Get all `Value` chain nodes from the given grid.
pub fn get_value_nodes<T: GridSize>(grid: &Grid<T>) -> Vec<ChainNode<T>> {
    let mut value_nodes = Vec::new();
    for cell in grid.empty_cells().iter() {
        for candidate in grid.candidates(cell).iter() {
            value_nodes.push(ChainNode::Value { cell, value: candidate });
        }
    }
    value_nodes
}

/// Get all `Group` chain nodes from the given grid.
pub fn get_group_nodes<T: GridSize>(grid: &Grid<T>) -> Vec<ChainNode<T>> {

    let mut group_nodes = Vec::new();
    let mut used_nodes = HashSet::new();

    for (region1, region2) in grid.all_regions().iter().tuple_combinations() {
        let intersection = region1 & region2;
        for &candidate in grid.values().iter() {
            let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
            if cells_with_candidate.len() > 1 {
                let group_node = ChainNode::Group { cells: cells_with_candidate, value: candidate };
                if !used_nodes.contains(&group_node) {
                    used_nodes.insert(group_node.clone());
                    group_nodes.push(group_node);
                }
            }
        }
    }

    group_nodes
}

/// Get all `Als` chain nodes from the given grid.
pub fn get_als_nodes<T: GridSize>(grid: &Grid<T>) -> Vec<ChainNode<T>> {

    let mut als_nodes = Vec::new();
    let mut used_nodes = HashSet::new();

    for region in grid.all_regions() {
        let empty_cells = grid.empty_cells_in_region(region);
        for degree in 2..empty_cells.len() {
            for cells in empty_cells.iter().combinations(degree).map(CellSet::from_cells) {
                let candidates = grid.all_candidates_from_region(&cells);
                if candidates.len() == degree + 1 {
                    for value in candidates.iter() {
                        let als_node = ChainNode::Als { cells: cells.clone(), value, cells_with_value: grid.cells_with_candidate_in_region(value, &cells) };
                        if !used_nodes.contains(&als_node) {
                            used_nodes.insert(als_node.clone());
                            als_nodes.push(als_node);
                        }
                    }
                }
            }
        }
    }

    als_nodes
}

fn is_linked_value_on_value_off<T: GridSize>(grid: &Grid<T>, value_on_node: &ChainNode<T>, value_off_node: &ChainNode<T>) -> bool {
    match (value_on_node, value_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            if *on_value == *off_value {
                grid.neighbours(*on_cell).contains(*off_cell)
            } else {
                *on_cell == *off_cell
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_off_value_on<T: GridSize>(grid: &Grid<T>, value_off_node: &ChainNode<T>, value_on_node: &ChainNode<T>) -> bool {
    match (value_off_node, value_on_node) {
        (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Value { cell: on_cell, value: on_value }) => {
            if *on_value == *off_value {
                let regions_to_consider = grid.all_regions_containing(&CellSet::from_cells(vec![*off_cell, *on_cell]));
                regions_to_consider.iter().any(|region| grid.cells_with_candidate_in_region(*off_value, region).len() == 2)
            } else if *on_cell == *off_cell {
                grid.num_candidates(*on_cell) == 2
            } else {
                false
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_on_group_off<T: GridSize>(grid: &Grid<T>, value_on_node: &ChainNode<T>, group_off_node: &ChainNode<T>) -> bool {
    match (value_on_node, group_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Group { cells: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && grid.neighbours(*on_cell).contains_all(off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_value_off<T: GridSize>(grid: &Grid<T>, group_on_node: &ChainNode<T>, value_off_node: &ChainNode<T>) -> bool {
    match (group_on_node, value_off_node) {
        (ChainNode::Group { cells: on_cells, value: on_value, .. }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            *on_value == *off_value && grid.neighbours(*off_cell).contains_all(on_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_off_group_on<T: GridSize>(grid: &Grid<T>, value_off_node: &ChainNode<T>, group_on_node: &ChainNode<T>) -> bool {
    match (value_off_node, group_on_node) {
        (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Group { cells: on_cells, value: on_value }) => {
            if *on_value != *off_value { return false; }
            if on_cells.contains(*off_cell) { return false; }
            let involved_cells = CellSet::from_cell(*off_cell) | on_cells;
            grid.all_regions().iter().any(|region| region.contains(*off_cell) && involved_cells.contains_all(&grid.cells_with_candidate_in_region(*off_value, region)))
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_off_value_on<T: GridSize>(grid: &Grid<T>, group_off_node: &ChainNode<T>, value_on_node: &ChainNode<T>) -> bool {
    match (group_off_node, value_on_node) {
        (ChainNode::Group { cells: off_cells, value: off_value }, ChainNode::Value { cell: on_cell, value: on_value }) => {
            if *on_value != *off_value { return false; }
            if off_cells.contains(*on_cell) { return false; }
            let involved_cells = CellSet::from_cell(*on_cell) | off_cells;
            grid.all_regions().iter().any(|region| region.contains(*on_cell) && involved_cells.contains_all(&grid.cells_with_candidate_in_region(*off_value, region)))
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_group_off<T: GridSize>(grid: &Grid<T>, group_on_node: &ChainNode<T>, group_off_node: &ChainNode<T>) -> bool {
    match (group_on_node, group_off_node) {
        (ChainNode::Group { cells: on_cells, value: on_value }, ChainNode::Group { cells: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && grid.common_neighbours(on_cells).contains_all(off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_off_group_on<T: GridSize>(grid: &Grid<T>, group_on_node: &ChainNode<T>, group_off_node: &ChainNode<T>) -> bool {
    match (group_off_node, group_on_node) {
        (ChainNode::Group { cells: off_cells, value: off_value, .. }, ChainNode::Group { cells: on_cells, value: on_value, .. }) => {
            if *on_value != *off_value { return false; }
            let involved_cells = on_cells | off_cells;
            grid.all_regions().iter().any(|region| !(region & off_cells).is_empty() && involved_cells.contains_all(&grid.cells_with_candidate_in_region(*off_value, region)))
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_on_als_off<T: GridSize>(grid: &Grid<T>, value_on_node: &ChainNode<T>, als_off_node: &ChainNode<T>) -> bool {
    match (value_on_node, als_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            if *on_value == *off_value {
                grid.neighbours(*on_cell).contains_all(off_cells)
            } else {
                CellSet::from_cell(*on_cell) == *off_cells
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_value_off<T: GridSize>(grid: &Grid<T>, als_on_node: &ChainNode<T>, value_off_node: &ChainNode<T>) -> bool {
    match (als_on_node, value_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            if *on_value == *off_value {
                grid.neighbours(*off_cell).contains_all(on_cells)
            } else {
                CellSet::from_cell(*off_cell) == *on_cells
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_als_off<T: GridSize>(grid: &Grid<T>, group_on_node: &ChainNode<T>, als_off_node: &ChainNode<T>) -> bool {
    match (group_on_node, als_off_node) {
        (ChainNode::Group { cells: on_cells, value: on_value }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && grid.common_neighbours(on_cells).contains_all(off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_group_off<T: GridSize>(grid: &Grid<T>, als_on_node: &ChainNode<T>, group_off_node: &ChainNode<T>) -> bool {
    match (als_on_node, group_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Group { value: off_value, .. }) => {
            *on_value == *off_value && grid.common_neighbours(on_cells).contains_all(on_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_als_off<T: GridSize>(grid: &Grid<T>, als_on_node: &ChainNode<T>, als_off_node: &ChainNode<T>) -> bool {
    match (als_on_node, als_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            if *on_value != *off_value {
                on_cells.len() == 1 && *on_cells == *off_cells
            } else {
                grid.common_neighbours(on_cells).contains_all(off_cells)
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_off_als_on<T: GridSize>(_grid: &Grid<T>, als_off_node: &ChainNode<T>, als_on_node: &ChainNode<T>) -> bool {
    match (als_off_node, als_on_node) {
        (ChainNode::Als { cells: off_cells, value: off_value, .. }, ChainNode::Als { cells: on_cells, value: on_value, .. }) => {
            *off_cells == *on_cells && *off_value != *on_value
        },
        _ => unreachable!(),
    }
}
