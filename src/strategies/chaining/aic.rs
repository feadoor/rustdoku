//! Strategies that look for AICs in a grid.
//!
//! An AIC (Alternating Inference Chain) is a pattern that consists of a chain of logical
//! inferences, each following form the previous, and alternating between positive and negative
//! statements about the grid.
//!
//! In such a chain that begins with a negative inference and ends with a positive one, one of the
//! two endpoints must be true. If those two endpoints have common peers, those peers can be
//! eliminated from the grid.

use grid::{Candidate, CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::Deduction;
use strategies::chaining::nodes;
use strategies::chaining::nodes::ChainNode;

use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(PartialEq, Eq)]
/// A struct representing a single inference which is part of an AIC
pub struct AicInference {
    node: ChainNode,
    negated: bool,
}

/// A convenience type for a candidate that would be eliminated by an AIC inference
pub type AffectedCandidate = (CellIdx, Candidate);

/// A convenience type to represent an entire AIC
pub type Aic = Vec<AicInference>;

/// Implement a handy display trait for `AicInference`
impl fmt::Display for AicInference {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.negated {
            true => write!(f, "-{}", self.node),
            false => write!(f, "+{}", self.node),
        }
    }
}

/// Search for AICs in the given grid, constructed from the given possible nodes.
pub fn find_aics(grid: &Grid, nodes: Vec<ChainNode>) -> Vec<Aic> {

    // Create the adjacency lists for the given nodes. Each node is treated as two different
    // vertices in the graph of linked inferences - one for the inference where the node is ON,
    // and one for the inference where the node is OFF.
    let mut adjacencies = vec![vec![]; 2 * nodes.len()];
    for (start_idx, start_node) in nodes.iter().enumerate() {
        for (end_idx, end_node) in nodes.iter().enumerate() {
            if (start_idx != end_idx) && nodes::is_linked_on_to_off(grid, start_node, end_node) {
                adjacencies[2 * start_idx].push(2 * end_idx + 1);
            }
            if (start_idx != end_idx) && nodes::is_linked_off_to_on(grid, start_node, end_node) {
                adjacencies[2 * start_idx + 1].push(2 * end_idx);
            }
        }
    }

    // For each node, decide which candidates would be eliminated if it were ON
    let affected_candidates: Vec<_> = nodes.iter().map(|node| find_affected_candidates(grid, node)).collect();

    // For each OFF version of a node, perform a breadth-first search and look for ON nodes that
    // are a result, via an AIC, of the original OFF inference. If the OFF inference and the linked
    // ON inference have any affected candidates in common, then they can be eliminated as a result
    // of the chain.
    let mut chains = Vec::new();
    for start_idx in 0..nodes.len() {
        for chain in breadth_first_search(&nodes, &adjacencies, &affected_candidates, start_idx) {
            chains.push(chain);
        }
    }

    chains.sort_by_key(|chain| chain.len());
    chains
}

/// Get the deductions arising from the given AIC.
pub fn get_aic_deductions(grid: &Grid, aic: &Aic) -> Vec<Deduction> {
    let (on_node, off_node) = (&aic[0].node, &aic[aic.len() - 1].node);
    get_strong_link_deductions(grid, on_node, off_node)
}

/// Get a description of the given chain.
pub fn get_aic_description(aic: &Aic) -> String {
    let mut description = format!("{}", aic[0]);
    for inference in aic.iter().skip(1) {
        description.push_str(" --> ");
        description.push_str(&format!("{}", inference));
    }
    description
}

/// Find candidates which would be eliminated as a result of the given node being ON
fn find_affected_candidates(grid: &Grid, node: &ChainNode) -> HashSet<AffectedCandidate> {

    let mut affected_candidates = HashSet::new();

    // All cells which are in sight of every candidate that might be switched ON in this node is dead
    let (value, value_cells) = (get_value(node), get_value_cells(node));
    let common_neighbours = CellSet::intersection(&value_cells.map(|ix| *Grid::neighbours(ix)));
    for cell in grid.cells_with_candidate_in_region(value, &common_neighbours).iter() {
        affected_candidates.insert((cell, value));
    }

    // If this node is for a single cell, then every other candidate in that cell is dead
    if value_cells.len() == 1 {
        let cell = value_cells.first().unwrap();
        for other_value in grid.candidates(cell).iter() {
            if other_value != value {
                affected_candidates.insert((cell, other_value));
            }
        }
    }

    affected_candidates
}

/// Get the deductions that arise from a strong link between the two given nodes.
fn get_strong_link_deductions(grid: &Grid, node1: &ChainNode, node2: &ChainNode) -> Vec<Deduction> {
    let first_affected_candidates = find_affected_candidates(grid, node1);
    let second_affected_candidates = find_affected_candidates(grid, node2);
    let common_affected_candidates = first_affected_candidates.intersection(&second_affected_candidates);
    common_affected_candidates.map(|&(cell, value)| Deduction::Elimination(cell, value)).collect()
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

/// Perform a breadth-first search looking for chains starting from the OFF version of the given node
/// and ending in the ON version of another node, in such a way that eliminations result.
fn breadth_first_search(nodes: &[ChainNode], adjacencies: &[Vec<usize>], affected_candidates: &[HashSet<AffectedCandidate>], start_idx: usize) -> Vec<Aic> {

    // The current state of the search, and a record of how each node was reached
    let (mut queue, mut visited, mut parents) = (VecDeque::new(), vec![false; adjacencies.len()], vec![0; adjacencies.len()]);
    queue.push_back(2 * start_idx + 1); visited[2 * start_idx + 1] = true;

    // Continue until we have exhausted all reachable endpoints
    let mut chains = Vec::new();
    while !queue.is_empty() {
        let current_idx = queue.pop_front().unwrap();

        // If we have a usable chain, then store it
        if current_idx % 2 == 0 && !affected_candidates[current_idx / 2].is_disjoint(&affected_candidates[start_idx]) {
            chains.push(create_chain(nodes, &parents, 2 * start_idx + 1, current_idx));
        }

        // Add all possible next nodes to the queue
        for &next_idx in &adjacencies[current_idx] {
            if !visited[next_idx] {
                queue.push_back(next_idx); visited[next_idx] = true;
                parents[next_idx] = current_idx;
            }
        }
    }

    chains
}

/// Translate a path found via breadth-first search into an actual chain
fn create_chain(nodes: &[ChainNode], parents: &[usize], start_idx: usize, end_idx: usize) -> Aic {
    let mut chain = Vec::new();
    let (mut negated, mut current_idx) = (end_idx % 2 == 1, end_idx);
    while current_idx != start_idx {
        chain.push(AicInference { node: nodes[current_idx / 2].clone(), negated });
        negated = !negated; current_idx = parents[current_idx];
    }
    chain.push(AicInference { node: nodes[current_idx / 2].clone(), negated });
    chain.reverse();
    chain
}
