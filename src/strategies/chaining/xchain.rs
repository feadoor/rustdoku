//! A definition of the X-Chain strategy.

use grid::{Candidate, Grid};
use grid::candidateset::CandidateSet;
use strategies::Step;
use strategies::chaining;
use strategies::chaining::{ChainNode, Chaining};
use utils::GeneratorAdapter;

/// Find X-Chains that exist in the grid.
///
/// An X-Chain is an alternating inference chain that only uses links between the same candidate
/// value throughout the length of the chain.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {
    GeneratorAdapter::of(move || {
        for candidate in CandidateSet::full().iter() {
            let x_chainer = XChainFinder::with_candidate(candidate);
            for chain in chaining::find_chains(grid, x_chainer) {
                yield Step::XChain { chain }
            }
        }
    })
}

struct XChainFinder {
    value: Candidate,
}

impl XChainFinder {

    pub fn with_candidate(value: Candidate) -> XChainFinder {
        XChainFinder { value }
    }

    fn is_linked_value_on_value_off(_grid: &Grid, value_on_node: &ChainNode, value_off_node: &ChainNode) -> bool {
        match (value_on_node, value_off_node) {
            (ChainNode::Value { cell: on_cell, .. }, ChainNode::Value { cell: off_cell, .. }) => {
                Grid::neighbours(*on_cell).contains(*off_cell)
            },
            _ => unreachable!(),
        }
    }

    fn is_linked_value_off_value_on(grid: &Grid, value_off_node: &ChainNode, value_on_node: &ChainNode) -> bool {
        match (value_off_node, value_on_node) {
            (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Value { cell: on_cell, .. }) => {
                Grid::neighbours(*off_cell).contains(*on_cell) && !grid.candidate_in_region(*off_value, &(Grid::neighbours(*on_cell) & Grid::neighbours(*off_cell)))
            },
            _ => unreachable!(),
        }
    }

    fn is_linked_value_on_group_off(_grid: &Grid, value_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
        match (value_on_node, group_off_node) {
            (ChainNode::Value { cell: on_cell, .. }, ChainNode::Group { cells: off_cells, .. }) => {
                Grid::neighbours(*on_cell).contains_all(*off_cells)
            },
            _ => unreachable!(),
        }
    }

    fn is_linked_group_on_value_off(_grid: &Grid, group_on_node: &ChainNode, value_off_node: &ChainNode) -> bool {
        match (group_on_node, value_off_node) {
            (ChainNode::Group { cells: on_cells, .. }, ChainNode::Value { cell: off_cell, .. }) => {
                Grid::neighbours(*off_cell).contains_all(*on_cells)
            },
            _ => unreachable!(),
        }
    }

    fn is_linked_value_off_group_on(grid: &Grid, value_off_node: &ChainNode, group_on_node: &ChainNode) -> bool {
        match (value_off_node, group_on_node) {
            (ChainNode::Value { cell: off_cell, value: off_value }, ChainNode::Group { line: on_line, block: on_block, cells: on_cells, .. }) => {
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
            (ChainNode::Group { line: off_line, block: off_block, cells: off_cells, .. }, ChainNode::Value { cell: on_cell, value: on_value }) => {
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
            (ChainNode::Group { line: on_line, block: on_block, cells: on_cells, .. }, ChainNode::Group { cells: off_cells, .. }) => {
                (on_cells & off_cells).is_empty() && (on_line | on_block).contains_all(*off_cells)
            },
            _ => unreachable!(),
        }
    }

    fn is_linked_group_off_group_on(grid: &Grid, group_on_node: &ChainNode, group_off_node: &ChainNode) -> bool {
        match (group_off_node, group_on_node) {
            (ChainNode::Group { line: off_line, block: off_block, cells: off_cells, value: off_value, .. }, ChainNode::Group { cells: on_cells, .. }) => {
                (off_line.contains_all(*on_cells) && grid.cells_with_candidate_in_region(*off_value, off_line) == (off_cells | on_cells)) ||
                (off_block.contains_all(*on_cells) && grid.cells_with_candidate_in_region(*off_value, off_block) == (off_cells | on_cells))
            },
            _ => unreachable!(),
        }
    }

}

impl Chaining for XChainFinder {

    fn get_nodes(&self, grid: &Grid) -> Vec<ChainNode> {
        let mut value_nodes = get_value_nodes_for_candidate(grid, self.value);
        let mut group_nodes = get_group_nodes_for_candidate(grid, self.value);
        value_nodes.append(&mut group_nodes);
        value_nodes
    }

    fn is_linked_on_to_off(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
        match (start_node, end_node) {
            (ChainNode::Value { .. }, ChainNode::Value { .. }) => XChainFinder::is_linked_value_on_value_off(grid, start_node, end_node),
            (ChainNode::Value { .. }, ChainNode::Group { .. }) => XChainFinder::is_linked_value_on_group_off(grid, start_node, end_node),
            (ChainNode::Group { .. }, ChainNode::Value { .. }) => XChainFinder::is_linked_group_on_value_off(grid, start_node, end_node),
            (ChainNode::Group { .. }, ChainNode::Group { .. }) => XChainFinder::is_linked_group_on_group_off(grid, start_node, end_node),
        }
    }

    fn is_linked_off_to_on(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
        match (start_node, end_node) {
            (ChainNode::Value { .. }, ChainNode::Value { .. }) => XChainFinder::is_linked_value_off_value_on(grid, start_node, end_node),
            (ChainNode::Value { .. }, ChainNode::Group { .. }) => XChainFinder::is_linked_value_off_group_on(grid, start_node, end_node),
            (ChainNode::Group { .. }, ChainNode::Value { .. }) => XChainFinder::is_linked_group_off_value_on(grid, start_node, end_node),
            (ChainNode::Group { .. }, ChainNode::Group { .. }) => XChainFinder::is_linked_group_off_group_on(grid, start_node, end_node),
        }
    }

}

/// Get all `Value` chain nodes for a given candidate from the given grid.
fn get_value_nodes_for_candidate(grid: &Grid, candidate: Candidate) -> Vec<ChainNode> {
    let mut value_nodes = Vec::new();
    for cell in grid.cells_with_candidate(candidate).iter() {
        value_nodes.push(ChainNode::Value { cell, value: candidate });
    }
    value_nodes
}

/// Get all `Group` chain nodes for a given candidate from the given grid.
fn get_group_nodes_for_candidate(grid: &Grid, candidate: Candidate) -> Vec<ChainNode> {
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