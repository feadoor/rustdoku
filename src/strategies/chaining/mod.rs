//! Common elements of chaining strategies

pub mod xchain;
pub mod xychain;

use std::fmt;

use grid::{Candidate, CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::Deduction;

#[derive(PartialEq, Eq, Clone)]
pub enum ChainNode {
    Value { cell: CellIdx, value: Candidate },
    Group { line: CellSet, block: CellSet, cells: CellSet, value: Candidate },
}

impl fmt::Display for ChainNode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainNode::Value { cell, value } => write!(f, "{}{}", value, Grid::cell_name(cell)),
            ChainNode::Group { cells, value, .. } => write!(f, "{}{}", value, Grid::region_name(&cells)),
        }
    }
}

pub struct ChainStep {
    node: ChainNode,
    negated: bool,
}

impl fmt::Display for ChainStep {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.negated {
            true => write!(f, "-{}", self.node),
            false => write!(f, "+{}", self.node),
        }
    }
}

pub type Chain = Vec<ChainStep>;

/// Get the deductions arising from the given chain.
pub fn get_deductions(grid: &Grid, chain: &Chain) -> Vec<Deduction> {
    match chain[0] {
        ChainStep { node: ChainNode::Value { cell, value }, negated: false } => vec![Deduction::Elimination(cell, value)],
        ChainStep { node: ChainNode::Value { cell, value }, negated: true } => vec![Deduction::Placement(cell, value)],
        ChainStep { node: ChainNode::Group { cells, value, .. }, negated: false } => {
            cells.map(|cell| Deduction::Elimination(cell, value))
        },
        ChainStep { node: ChainNode::Group { line, block, cells, value}, negated: true } => {
            let elimination_cells = (block | line) & !cells;
            grid.cells_with_candidate_in_region(value, &elimination_cells).map(|cell| Deduction::Elimination(cell, value))
        },
    }
}

/// Get a description of the given chain.
pub fn get_description(chain: &Chain) -> String {
    let mut description = format!("{}", chain[0]);
    for step in chain.iter().skip(1) {
        description.push_str(" --> ");
        description.push_str(&format!("{}", step));
    }
    description
}

/// A trait for strategies which will seek to form chains in the grid.
trait Chaining {

    /// Get the nodes that should be considered by this chaining strategy.
    fn get_nodes(&self, grid: &Grid) -> Vec<ChainNode>;

    /// Given two chain nodes, determine if this chaining strategy can form a link from one to the
    /// negation of the other.
    fn is_linked_on_to_off(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool;

    /// Given two chain nodes, determine if this chaining strategy can form a link from the negation
    /// of one to the other.
    fn is_linked_off_to_on(&self, grid: &Grid, start_node: &ChainNode, end_node: &ChainNode) -> bool;

}

/// Find chains that lead to deductions in the grid using the Floyd-Warshall pathfinding algorithm
fn find_chains<T: Chaining>(grid: &Grid, chainer: T) -> Vec<Chain> {

    // Create the possible chain nodes and treat them as graph vertices. Each node becomes two
    // vertices in the graph - one for itself, and one for its negation.
    let nodes = chainer.get_nodes(grid);
    let mut distances = vec![vec![usize::max_value(); 2 * nodes.len()]; 2 * nodes.len()];
    let mut paths = vec![vec![0; 2 * nodes.len()]; 2 * nodes.len()];

    // Create the adjacency matrix of the graph
    for (start_idx, start_node) in nodes.iter().enumerate() {
        for (end_idx, end_node) in nodes.iter().enumerate() {
            if (start_idx != end_idx) && chainer.is_linked_on_to_off(grid, start_node, end_node) {
                distances[2 * start_idx][2 * end_idx + 1] = 1;
                paths[2 * start_idx][2 * end_idx + 1] = 2 * end_idx + 1;
            }
            if (start_idx != end_idx) && chainer.is_linked_off_to_on(grid, start_node, end_node) {
                distances[2 * start_idx + 1][2 * end_idx] = 1;
                paths[2 * start_idx + 1][2 * end_idx] = 2 * end_idx;
            }
        }
    }

    // Carry out the Floyd-Warshall algorithm
    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                if distances[i][k].saturating_add(distances[k][j]) < distances[i][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                    paths[i][j] = paths[i][k];
                }
            }
        }
    }

    // Return all of the chains that create a deduction - that is, chains from
    // a node to its negation or vice-versa.
    let mut chains = vec![];
    for idx in 0..nodes.len() {

        // From a node to its negation
        if distances[2 * idx][2 * idx + 1] < usize::max_value() {


            let mut chain = Vec::new();
            let (mut negated, mut current_idx) = (false, 2 * idx);
            while current_idx != 2 * idx + 1 {
                chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
                negated = !negated; current_idx = paths[current_idx][2 * idx + 1];
            }
            chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
            chains.push(chain);
        }

        // From the negation of a node to a node
        if distances[2 * idx + 1][2 * idx] < usize::max_value() {

            let mut chain = Vec::new();
            let (mut negated, mut current_idx) = (true, 2 * idx + 1);
            while current_idx != 2 * idx {
                chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
                negated = !negated; current_idx = paths[current_idx][2 * idx];
            }
            chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
            chains.push(chain);
        }
    }

    chains.sort_by_key(|chain| chain.len());
    chains
}