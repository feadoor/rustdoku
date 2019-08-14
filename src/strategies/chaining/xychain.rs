//! A definition of the XY-Chain strategy.

use grid::Grid;
use strategies::Step;
use strategies::chaining;
use strategies::chaining::{ChainNode, Chaining};
use utils::GeneratorAdapter;

/// Find XY-Chains that exist in the grid.
///
/// An XY-Chain is an alternating inference chain that only uses strong links within bivalue cells.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {
    GeneratorAdapter::of(move || {
        let xy_chainer = XYChainFinder::new();
        for chain in chaining::find_chains(grid, xy_chainer) {
            yield Step::XYChain { chain }
        }
    })
}

struct XYChainFinder {}

impl XYChainFinder {

    pub fn new() -> XYChainFinder {
        XYChainFinder {}
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
                (*off_cell == *on_cell) && (*on_value != *off_value) && grid.num_candidates(*off_cell) == 2
            },
            _ => unreachable!(),
        }
    }
}

impl Chaining for XYChainFinder {

    fn get_nodes(&self, grid: &Grid) -> Vec<ChainNode> {
        get_value_nodes(grid)
    }

    fn is_linked_on_to_off(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
        match (start_node, end_node) {
            (ChainNode::Value { .. }, ChainNode::Value { .. }) => XYChainFinder::is_linked_value_on_value_off(grid, start_node, end_node),
            _ => unreachable!(),
        }
    }

    fn is_linked_off_to_on(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool {
        match (start_node, end_node) {
            (ChainNode::Value { .. }, ChainNode::Value { .. }) => XYChainFinder::is_linked_value_off_value_on(grid, start_node, end_node),
            _ => unreachable!(),
        }
    }

}

/// Get all `Value` chain nodes for a given candidate from the given grid.
fn get_value_nodes(grid: &Grid) -> Vec<ChainNode> {
    let mut value_nodes = Vec::new();
    for cell in grid.empty_cells().iter() {
        for value in grid.candidates(cell).iter() {
            value_nodes.push(ChainNode::Value { cell, value });
        }
    }
    value_nodes
}
