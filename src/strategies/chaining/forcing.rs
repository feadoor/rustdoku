//! Strategies that look for forcing chains in a grid.
//!
//! A forcing chain is a misnomer - it actually consists of multiple chains, starting from a set
//! of premises of which at least one must be true. The most common such sets of premises are
//! of the the candidates in a single cell, or all of the occurrences of a particular candidate
//! in a single region of the grid.
//!
//! If there exists a common consequence of a chain from each of the starting premises, then that
//! consequence must indeed be true, and can be added to the grid.

use grid::{Candidate, CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::Deduction;
use strategies::chaining::nodes;
use strategies::chaining::nodes::ChainNode;

use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(PartialEq, Eq)]
/// A struct representing a single inference which is part of a forcing chain
pub struct ForcingChainInference {
    node: ChainNode,
    negated: bool,
}

/// A convenience type to represent an entire forcing chain
pub type ForcingChain = Vec<Vec<ForcingChainInference>>;

/// Implement a handy display trait for `ForcingChainInference`
impl fmt::Display for ForcingChainInference {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.negated {
            true => write!(f, "-{}", self.node),
            false => write!(f, "+{}", self.node),
        }
    }
}

/// Search for forcing chains in the given grid, using the given nodes as possible links in a chain.
pub fn find_forcing_chains(grid: &Grid, nodes: Vec<ChainNode>) -> Vec<ForcingChain> {

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

    // Determine which nodes represent single candidates, and group them by cell and by candidate.
    let mut nodes_by_cell_and_candidate = vec![vec![Vec::new(); 9]; 81];
    for (idx, node) in nodes.iter().enumerate() {
        if let ChainNode::Value { cell, value } = node {
            nodes_by_cell_and_candidate[*cell][*value].push(idx);
        }
    }

    // For each unsolved cell, determine the common consequences of chains from each candidate in
    // that cell.
    for cell in grid.empty_cells().iter() {

    }
}

/// Perform a breadth-first search looking for chains starting from the ON version of the given node.
/// Return enough information to know what all of the consequences of such chains are, and to be
/// able to reconstruct the chains themselves.
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