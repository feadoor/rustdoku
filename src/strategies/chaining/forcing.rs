//! Strategies that look for forcing chains in a grid.
//!
//! A forcing chain is a misnomer - it actually consists of multiple chains, starting from a set
//! of premises of which at least one must be true. The most common such sets of premises are
//! of the the candidates in a single cell, or all of the occurrences of a particular candidate
//! in a single region of the grid.
//!
//! If there exists a common consequence of a chain from each of the starting premises, then that
//! consequence must indeed be true, and can be added to the grid.

use grid::{Grid, GridSize};
use strategies::Deduction;
use strategies::chaining::nodes;
use strategies::chaining::nodes::ChainNode;

use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
/// A struct representing a single inference which is part of a forcing chain
pub struct ForcingChainInference<T: GridSize> {
    node: ChainNode<T>,
    negated: bool,
}

/// A convenience type to represent an entire forcing chain
pub type ForcingChain<T> = Vec<Vec<ForcingChainInference<T>>>;

impl <T: GridSize> ForcingChainInference<T> {

    /// Get a readable description of a `ForcingChainInference`
    fn get_description(&self, grid: &Grid<T>) -> String {
        match self.negated {
            true => format!("-{}", self.node.get_description(grid)),
            false => format!("+{}", self.node.get_description(grid)),
        }
    }
}

/// Search for forcing chains in the given grid, using the given nodes as possible links in a chain.
pub fn find_forcing_chains<T: GridSize>(grid: &Grid<T>, nodes: Vec<ChainNode<T>>) -> Vec<ForcingChain<T>> {

    let mut chains = Vec::new();

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

    // Determine which nodes represent single candidates, and store the indices of these nodes.
    let mut nodes_by_cell_and_candidate = vec![vec![0; T::size() + 1]; T::size() * T::size()];
    for (idx, node) in nodes.iter().enumerate() {
        if let ChainNode::Value { cell, value } = node {
            nodes_by_cell_and_candidate[*cell][*value] = idx;
        }
    }

    // Store the results of a breadth-first search from each single-value node.
    let mut search_results = vec![SearchResults { visited: Vec::new(), parents: Vec::new() }; nodes.len()];
    for (idx, node) in nodes.iter().enumerate() {
        if let ChainNode::Value { .. } = node {
            search_results[idx] = breadth_first_search(&adjacencies, idx);
        }
    }

    // For each unsolved cell, determine the common consequences of chains from each candidate in
    // that cell.
    for cell in grid.empty_cells().iter().filter(|&c| grid.num_candidates(c) >= 3) {
        let premises = grid.candidates(cell).map(|value| nodes_by_cell_and_candidate[cell][value]);
        chains.append(&mut find_chains(&nodes, &adjacencies, &search_results, &premises));
    }

    // For each region, and each candidate missing from the region, determine the common consequences
    // of chains from each placement of that candidate in the region.
    for region in grid.all_regions() {
        for candidate in grid.values_missing_from_region(region).iter() {
            let cells = grid.cells_with_candidate_in_region(candidate, region);
            if cells.len() >= 3 {
                let premises = cells.map(|cell| nodes_by_cell_and_candidate[cell][candidate]);
                chains.append(&mut find_chains(&nodes, &adjacencies, &search_results, &premises));
            }
        }
    }

    chains.sort_by_key(|chain| (chain.len(), chain.iter().map(|c| c.len()).sum::<usize>()));
    chains
}

/// Get the deductions arising from the given Forcing chain.
pub fn get_forcing_chain_deductions<T: GridSize>(grid: &Grid<T>, forcing_chain: &ForcingChain<T>) -> Vec<Deduction> {
    let consequence = forcing_chain[0].last().unwrap();
    match consequence {
        ForcingChainInference { node: ChainNode::Value { cell, value }, negated: true } => {
            vec![Deduction::Elimination(*cell, *value)]
        },
        ForcingChainInference { node: ChainNode::Value { cell, value }, negated: false } => {
            vec![Deduction::Placement(*cell, *value)]
        },
        ForcingChainInference { node: ChainNode::Group { cells, value, .. }, negated: true } => {
            cells.map(|cell| Deduction::Elimination(cell, *value))
        },
        ForcingChainInference { node: ChainNode::Group { cells, value }, negated: false } => {
            let elimination_region = grid.common_neighbours(cells);
            grid.cells_with_candidate_in_region(*value, &elimination_region).map(|cell| Deduction::Elimination(cell, *value))
        },
        ForcingChainInference { node: ChainNode::Als { cells_with_value, value, .. }, negated: true } => {
            cells_with_value.map(|cell| Deduction::Elimination(cell, *value))
        },
        ForcingChainInference { node: ChainNode::Als { cells_with_value, value, .. }, negated: false } => {
            let elimination_region = grid.common_neighbours(cells_with_value);
            grid.cells_with_candidate_in_region(*value, &elimination_region).map(|cell| Deduction::Elimination(cell, *value))
        },
    }
}

/// Get a description of the given chain.
pub fn get_forcing_chain_description<T: GridSize>(grid: &Grid<T>, forcing_chain: &ForcingChain<T>) -> String {
    let mut description = String::new();
    for chain in forcing_chain {
        description.push_str(&format!("\n        {}", chain[0].get_description(grid)));
        for inference in chain.iter().skip(1) {
            description.push_str(" --> ");
            description.push_str(&format!("{}", inference.get_description(grid)));
        }
    }
    description
}

/// Determine the common consequences, and chains proving them, for the given set of starting premises.
fn find_chains<T: GridSize>(nodes: &[ChainNode<T>], adjacencies: &[Vec<usize>], search_information: &[SearchResults], premises: &[usize]) -> Vec<ForcingChain<T>> {

    // Find the common consequences of the starting premises
    let common_consequences: Vec<_> = (0..adjacencies.len()).filter(|&idx|
        premises.iter().all(|&premise| search_information[premise].visited[idx])
    ).collect();

    // For each common consequence, produce a forcing chain that proves it.
    let mut chains = Vec::new();
    for consequence in common_consequences {
        let chains_for_this_consequence = premises.iter().map(|&premise|
            create_chain(nodes, &search_information[premise].parents, 2 * premise, consequence)
        ).collect();
        chains.push(chains_for_this_consequence);
    }

    chains
}

/// A struct representing the output of a breadth-first search from a forcing chain node.
#[derive(Clone)]
struct SearchResults {
    visited: Vec<bool>,
    parents: Vec<usize>,
}

/// Perform a breadth-first search looking for chains starting from the ON version of the given node.
/// Return enough information to know what all of the consequences of such chains are, and to be
/// able to reconstruct the chains themselves.
fn breadth_first_search(adjacencies: &[Vec<usize>], start_idx: usize) -> SearchResults {

    // The current state of the search, and a record of how each node was reached
    let (mut queue, mut visited, mut parents) = (VecDeque::new(), vec![false; adjacencies.len()], vec![0; adjacencies.len()]);
    queue.push_back(2 * start_idx); visited[2 * start_idx] = true;

    // Continue until we have exhausted all reachable endpoints
    while !queue.is_empty() {
        let current_idx = queue.pop_front().unwrap();

        // Add all possible next nodes to the queue
        for &next_idx in &adjacencies[current_idx] {
            if !visited[next_idx] {
                queue.push_back(next_idx); visited[next_idx] = true;
                parents[next_idx] = current_idx;
            }
        }
    }

    SearchResults  { visited, parents }
}

/// Translate a path found via breadth-first search into an actual chain
fn create_chain<T: GridSize>(nodes: &[ChainNode<T>], parents: &[usize], start_idx: usize, end_idx: usize) -> Vec<ForcingChainInference<T>> {
    let mut chain = Vec::new();
    let (mut negated, mut current_idx) = (end_idx % 2 == 1, end_idx);
    while current_idx != start_idx {
        chain.push(ForcingChainInference { node: nodes[current_idx / 2].clone(), negated });
        negated = !negated; current_idx = parents[current_idx];
    }
    chain.push(ForcingChainInference { node: nodes[current_idx / 2].clone(), negated });
    chain.reverse();
    chain
}
