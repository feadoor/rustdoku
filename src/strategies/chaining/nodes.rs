//! A definition of the nodes that can form part of a chain.

use grid::{Candidate, CellIdx, Grid};
use grid::candidateset::CandidateSet;
use grid::cellset::CellSet;

use itertools::Itertools;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub enum ChainNode {
    Value { cell: CellIdx, value: Candidate },
    Group { line: CellSet, block: CellSet, cells: CellSet, value: Candidate },
    Als { cells: CellSet, cells_with_value: CellSet, value: Candidate },
}

/// Implement a handy display trait for `ChainNode`
impl fmt::Display for ChainNode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainNode::Value { cell, value } => write!(f, "{}{}", value, Grid::cell_name(cell)),
            ChainNode::Group { cells, value, .. } => write!(f, "{}{}", value, Grid::region_name(&cells)),
            ChainNode::Als { cells, value, .. } => write!(f, "{}{}", value, Grid::region_name(&cells)),
        }
    }
}

/// Implement a handy display trait for `ChainNode`
impl fmt::Debug for ChainNode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainNode::Value { cell, value } => write!(f, "{}{}", value, Grid::cell_name(cell)),
            ChainNode::Group { cells, value, .. } => write!(f, "{}{}", value, Grid::region_name(&cells)),
            ChainNode::Als { cells, value, .. } => write!(f, "{}{}", value, Grid::region_name(&cells)),
        }
    }
}

/// Determine if the two nodes are linked in such a way that the truth
/// of the first implies the falsity of the second
pub fn is_linked_on_to_off(grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
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
pub fn is_linked_off_to_on(grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
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
pub fn get_value_nodes_for_candidate(grid: &Grid, candidate: Candidate) -> Vec<ChainNode> {
    let mut value_nodes = Vec::new();
    for cell in grid.cells_with_candidate(candidate).iter() {
        value_nodes.push(ChainNode::Value { cell, value: candidate });
    }
    value_nodes
}

/// Get all `Group` chain nodes for a given candidate from the given grid.
pub fn get_group_nodes_for_candidate(grid: &Grid, candidate: Candidate) -> Vec<ChainNode> {
    let mut group_nodes = Vec::new();

    // Intersections of rows and boxes
    for row in Grid::rows() {
        for block in Grid::blocks() {
            let intersection = row & block;
            let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
            if cells_with_candidate.len() > 1 {
                group_nodes.push(ChainNode::Group { line: *row, block: *block, cells: cells_with_candidate, value: candidate });
            }
        }
    }

    // Intersections of columns and boxes
    for column in Grid::columns() {
        for block in Grid::blocks() {
            let intersection = column & block;
            let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
            if cells_with_candidate.len() > 1 {
                group_nodes.push(ChainNode::Group { line: *column, block: *block, cells: cells_with_candidate, value: candidate });
            }
        }
    }

    group_nodes
}

/// Get all `Value` chain nodes from the given grid.
pub fn get_value_nodes(grid: &Grid) -> Vec<ChainNode> {
    let mut value_nodes = Vec::new();
    for cell in grid.empty_cells().iter() {
        for candidate in grid.candidates(cell).iter() {
            value_nodes.push(ChainNode::Value { cell, value: candidate });
        }
    }
    value_nodes
}

/// Get all `Group` chain nodes from the given grid.
pub fn get_group_nodes(grid: &Grid) -> Vec<ChainNode> {
    let mut group_nodes = Vec::new();

    // Intersections of rows and boxes
    for row in Grid::rows() {
        for block in Grid::blocks() {
            let intersection = row & block;
            for candidate in CandidateSet::full().iter() {
                let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
                if cells_with_candidate.len() > 1 {
                    group_nodes.push(ChainNode::Group { line: *row, block: *block, cells: cells_with_candidate, value: candidate });
                }
            }
        }
    }

    // Intersections of columns and boxes
    for column in Grid::columns() {
        for block in Grid::blocks() {
            let intersection = column & block;
            for candidate in CandidateSet::full().iter() {
                let cells_with_candidate = grid.cells_with_candidate_in_region(candidate, &intersection);
                if cells_with_candidate.len() > 1 {
                    group_nodes.push(ChainNode::Group { line: *column, block: *block, cells: cells_with_candidate, value: candidate });
                }
            }
        }
    }

    group_nodes
}

/// Get all `Als` chain nodes from the given grid.
pub fn get_als_nodes(grid: &Grid) -> Vec<ChainNode> {

    let mut als_nodes = Vec::new();

    // ALSs within rows
    for row in Grid::rows() {
        let empty_cells = grid.empty_cells_in_region(row);
        for degree in 2..empty_cells.len() {
            for cells in empty_cells.iter().combinations(degree).map(CellSet::from_cells) {
                let candidates = grid.all_candidates_from_region(&cells);
                if candidates.len() == degree + 1 {
                    for value in candidates.iter() {
                        als_nodes.push(ChainNode::Als { cells, value, cells_with_value: grid.cells_with_candidate_in_region(value, &cells) });
                    }
                }
            }
        }
    }

    // ALSs within columns
    for column in Grid::columns() {
        let empty_cells = grid.empty_cells_in_region(column);
        for degree in 2..empty_cells.len() {
            for cells in empty_cells.iter().combinations(degree).map(CellSet::from_cells) {
                let candidates = grid.all_candidates_from_region(&cells);
                if candidates.len() == degree + 1 {
                    for value in candidates.iter() {
                        als_nodes.push(ChainNode::Als { cells, value, cells_with_value: grid.cells_with_candidate_in_region(value, &cells) });
                    }
                }
            }
        }
    }

    // ALSs within blocks, but not within a single row or column
    for block in Grid::blocks() {
        let empty_cells = grid.empty_cells_in_region(block);
        for degree in 2..empty_cells.len() {
            for cells in empty_cells.iter().combinations(degree).map(CellSet::from_cells) {
                let candidates = grid.all_candidates_from_region(&cells);
                if candidates.len() == degree + 1 && Grid::row_containing(&cells).is_none() && Grid::column_containing(&cells).is_none() {
                    for value in candidates.iter() {
                        als_nodes.push(ChainNode::Als { cells, value, cells_with_value: grid.cells_with_candidate_in_region(value, &cells) });
                    }
                }
            }
        }
    }

    als_nodes
}

fn is_linked_value_on_value_off(_grid: &Grid, value_on_node: &ChainNode, value_off_node: &ChainNode) -> bool {
    match (value_on_node, value_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            *on_value == *off_value && Grid::neighbours(*on_cell).contains(*off_cell)
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_off_value_on(grid: &Grid, value_off_node: &ChainNode, value_on_node: &ChainNode) -> bool {
    match (value_off_node, value_on_node) {
        (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Value { cell: on_cell, value: on_value }) => {
            if *on_value == *off_value {
                Grid::neighbours(*off_cell).contains(*on_cell) && !grid.candidate_in_region(*off_value, &(Grid::neighbours(*on_cell) & Grid::neighbours(*off_cell)))
            } else if *on_cell == *off_cell {
                grid.num_candidates(*on_cell) == 2
            } else {
                false
            }
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_on_group_off(_grid: &Grid, value_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
    match (value_on_node, group_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Group { cells: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && Grid::neighbours(*on_cell).contains_all(*off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_value_off(_grid: &Grid, group_on_node: &ChainNode, value_off_node: &ChainNode) -> bool {
    match (group_on_node, value_off_node) {
        (ChainNode::Group { cells: on_cells, value: on_value, .. }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            *on_value == *off_value && Grid::neighbours(*off_cell).contains_all(*on_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_off_group_on(grid: &Grid, value_off_node: &ChainNode, group_on_node: &ChainNode) -> bool {
    match (value_off_node, group_on_node) {
        (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Group { line: on_line, block: on_block, cells: on_cells, value: on_value }) => {
            if *on_value != *off_value { return false; }
            if on_cells.contains(*off_cell) { return false; }
            let peers = grid.cells_with_candidate_in_region(*off_value, Grid::neighbours(*off_cell));
            (on_line.contains(*off_cell) && on_cells.contains_all(peers & on_line)) ||
            (on_block.contains(*off_cell) && on_cells.contains_all(peers & on_block))
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_off_value_on(grid: &Grid, group_off_node: &ChainNode, value_on_node: &ChainNode) -> bool {
    match (group_off_node, value_on_node) {
        (ChainNode::Group { line: off_line, block: off_block, cells: off_cells, value: off_value }, ChainNode::Value { cell: on_cell, value: on_value }) => {
            if *on_value != *off_value { return false; }
            if off_cells.contains(*on_cell) { return false; }
            let peers = grid.cells_with_candidate_in_region(*on_value, Grid::neighbours(*on_cell));
            (off_line.contains(*on_cell) && off_cells.contains_all(peers & off_line)) ||
            (off_block.contains(*on_cell) && off_cells.contains_all(peers & off_block))
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_group_off(_grid: &Grid, group_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
    match (group_on_node, group_off_node) {
        (ChainNode::Group { line: on_line, block: on_block, cells: on_cells, value: on_value }, ChainNode::Group { cells: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && (on_cells & off_cells).is_empty() && (on_line | on_block).contains_all(*off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_off_group_on(grid: &Grid, group_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
    match (group_off_node, group_on_node) {
        (ChainNode::Group { line: off_line, block: off_block, cells: off_cells, value: off_value, .. }, ChainNode::Group { cells: on_cells, value: on_value, .. }) => {
            if *on_value != *off_value { return false; }
            (off_line.contains_all(*on_cells) && grid.cells_with_candidate_in_region(*off_value, off_line) == (off_cells | on_cells)) ||
            (off_block.contains_all(*on_cells) && grid.cells_with_candidate_in_region(*off_value, off_block) == (off_cells | on_cells))
        },
        _ => unreachable!(),
    }
}

fn is_linked_value_on_als_off(_grid: &Grid, value_on_node: &ChainNode, als_off_node: &ChainNode) -> bool {
    match (value_on_node, als_off_node) {
        (ChainNode::Value { cell: on_cell, value: on_value }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && Grid::neighbours(*on_cell).contains_all(*off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_value_off(_grid: &Grid, als_on_node: &ChainNode, value_off_node: &ChainNode) -> bool {
    match (als_on_node, value_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Value { cell: off_cell, value: off_value }) => {
            *on_value == *off_value && Grid::neighbours(*off_cell).contains_all(*on_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_group_on_als_off(_grid: &Grid, group_on_node: &ChainNode, als_off_node: &ChainNode) -> bool {
    match (group_on_node, als_off_node) {
        (ChainNode::Group { cells: on_cells, value: on_value, line: on_line, block: on_block }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            *on_value == *off_value && (on_cells & off_cells).is_empty() && (on_line | on_block).contains_all(*off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_group_off(_grid: &Grid, als_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
    match (als_on_node, group_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Group { cells: off_cells, value: off_value, line: off_line, block: off_block }) => {
            *on_value == *off_value && (on_cells & off_cells).is_empty() && (off_line | off_block).contains_all(*on_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_on_als_off(_grid: &Grid, als_on_node: &ChainNode, als_off_node: &ChainNode) -> bool {
    match (als_on_node, als_off_node) {
        (ChainNode::Als { cells_with_value: on_cells, value: on_value, .. }, ChainNode::Als { cells_with_value: off_cells, value: off_value, .. }) => {
            if *on_value != *off_value { return false; }
            let common_neighbours = CellSet::intersection(&on_cells.map(|ix| *Grid::neighbours(ix)));
            common_neighbours.contains_all(*off_cells)
        },
        _ => unreachable!(),
    }
}

fn is_linked_als_off_als_on(_grid: &Grid, als_off_node: &ChainNode, als_on_node: &ChainNode) -> bool {
    match (als_off_node, als_on_node) {
        (ChainNode::Als { cells: off_cells, value: off_value, .. }, ChainNode::Als { cells: on_cells, value: on_value, .. }) => {
            *off_cells == *on_cells && *off_value != *on_value
        },
        _ => unreachable!(),
    }
}