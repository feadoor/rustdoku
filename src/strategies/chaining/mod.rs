//! Common elements of chaining strategies

pub mod aic;
pub mod xchain;

use std::collections::VecDeque;
use std::fmt;

use grid::{Candidate, CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::Deduction;

#[derive(PartialEq, Eq, Clone)]
pub enum ChainNode {
    Value { cell: CellIdx, value: Candidate },
    Group { line: CellSet, block: CellSet, cells: CellSet, value: Candidate },
    Als { cells: CellSet, cells_with_value: CellSet, value: Candidate },
}

#[derive(PartialEq, Eq)]
pub struct ChainStep {
    node: ChainNode,
    negated: bool,
}

pub type Chain = Vec<ChainStep>;

/// Get the deductions arising from the given chain.
pub fn get_deductions(grid: &Grid, chain: &Chain) -> Vec<Deduction> {

    // From a node to its negation
    if chain[0].negated == false {
        let (on_node, off_node) = (&chain[1].node, &chain[chain.len() - 2].node);
        get_strong_link_deductions(grid, on_node, off_node)
    }

    // From the negation of a node to the original node
    else {
        match chain[0].node {
            ChainNode::Value { cell, value } => vec![Deduction::Placement(cell, value)],
            ChainNode::Group { line, block, cells, value } => {
                let elimination_cells = (block | line) & !cells;
                grid.cells_with_candidate_in_region(value, &elimination_cells).map(|cell| Deduction::Elimination(cell, value))
            },
            ChainNode::Als { cells_with_value, value, .. } => {
                let elimination_cells = CellSet::intersection(&cells_with_value.map(|ix| *Grid::neighbours(ix)));
                grid.cells_with_candidate_in_region(value, &elimination_cells).map(|cell| Deduction::Elimination(cell, value))
            },
        }
    }
}

/// Get a description of the given chain.
pub fn get_description(chain: &Chain) -> String {

    // A discontinuous loop from a node to its negation
    if chain[0].negated == false {
        let mut description = format!("{}", chain[1]);
        for step in chain.iter().skip(2).take(chain.len() - 3) {
            description.push_str(" --> ");
            description.push_str(&format!("{}", step));
        }
        description
    }

    // A discontinuous loop from the negation of a node to the original node
    else {
        let mut description = format!("{}", chain[0]);
        for step in chain.iter().skip(1) {
            description.push_str(" --> ");
            description.push_str(&format!("{}", step));
        }
        description
    }
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

/// Get the deductions that result from the given two nodes being strongly linked
fn get_strong_link_deductions(grid: &Grid, on_node: &ChainNode, off_node: &ChainNode) -> Vec<Deduction> {
    let (value, on_cells, off_cells) = (get_value(on_node), get_value_cells(on_node), get_value_cells(off_node));
    let elimination_cells = CellSet::intersection(&(on_cells | off_cells).map(|ix| * Grid::neighbours(ix)));
    grid.cells_with_candidate_in_region(value, &elimination_cells).map(|cell| Deduction::Elimination(cell, value))
}

/// Extract the value from a `ChainNode`
fn get_value(node: &ChainNode) -> Candidate {
    match node {
        ChainNode::Value { value, .. } => *value,
        ChainNode::Group { value, .. } => *value,
        ChainNode::Als { value, .. } => *value,
    }
}

/// Extract the cells with the right value from a `ChainNode`
fn get_value_cells(node: &ChainNode) -> CellSet {
    match node {
        ChainNode::Value { cell, .. } => CellSet::from_cell(*cell),
        ChainNode::Group { cells, .. } => *cells,
        ChainNode::Als { cells_with_value, .. } => *cells_with_value,
    }
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

/// Implement a handy display trait for `ChainStep`
impl fmt::Display for ChainStep {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.negated {
            true => write!(f, "-{}", self.node),
            false => write!(f, "+{}", self.node),
        }
    }
}

/// Find chains that lead to deductions in the grid using Dijkstra's algorithm
fn find_chains<T: Chaining>(grid: &Grid, chainer: T) -> Vec<Chain> {

    // Create the possible chain nodes and treat them as graph vertices. Each node becomes two
    // vertices in the graph - one for itself, and one for its negation.
    let nodes = chainer.get_nodes(grid);


    // Create the adjacency lists for the graph
    let mut adjacencies = vec![vec![]; 2 * nodes.len()];
    for (start_idx, start_node) in nodes.iter().enumerate() {
        for (end_idx, end_node) in nodes.iter().enumerate() {
            if (start_idx != end_idx) && chainer.is_linked_on_to_off(grid, start_node, end_node) {
                adjacencies[2 * start_idx].push(2 * end_idx + 1);
            }
            if (start_idx != end_idx) && chainer.is_linked_off_to_on(grid, start_node, end_node) {
                adjacencies[2 * start_idx + 1].push(2 * end_idx);
            }
        }
    }

    let mut chains = Vec::new();

    // Search for chains from a node to its negation
    for start_idx in 0..nodes.len() {
        if let Some(chain) = breadth_first_search(&nodes, &adjacencies, 2 * start_idx, 2 * start_idx + 1) {
            chains.push(chain);
        }
    }

    // Search for chains from the negation of a node to the original node
    for start_idx in 0..nodes.len() {
        if let Some(chain) = breadth_first_search(&nodes, &adjacencies, 2 * start_idx + 1, 2 * start_idx) {
            chains.push(chain);
        }
    }

    chains.sort_by_key(|chain| if chain[0].negated { chain.len() } else { chain.len() - 2 });
    chains
}

/// Perform a breadth-first search looking for a chain between the two given nodes
fn breadth_first_search(nodes: &[ChainNode], adjacencies: &[Vec<usize>], start_idx: usize, end_idx: usize) -> Option<Chain> {
    let (mut queue, mut visited, mut parents) = (VecDeque::new(), vec![false; adjacencies.len()], vec![0; adjacencies.len()]);
    queue.push_back(start_idx); visited[start_idx] = true;
    while !queue.is_empty() {

        let current_idx = queue.pop_front().unwrap();
        if current_idx == end_idx {
            return Some(create_chain(nodes, &parents, start_idx, end_idx));
        }

        for &next_idx in &adjacencies[current_idx] {
            if !visited[next_idx] {
                queue.push_back(next_idx); visited[next_idx] = true;
                parents[next_idx] = current_idx;
            }
        }
    }

    None
}

/// Translate a path found via breadth-first search into an actual chain
fn create_chain(nodes: &[ChainNode], parents: &[usize], start_idx: usize, end_idx: usize) -> Chain {
    let mut chain = Vec::new();
    let (mut negated, mut current_idx) = (start_idx % 2 == 1, end_idx);
    while current_idx != start_idx || chain.len() == 0 {
        chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
        negated = !negated; current_idx = parents[current_idx];
    }
    chain.push(ChainStep { node: nodes[current_idx / 2].clone(), negated });
    chain
}