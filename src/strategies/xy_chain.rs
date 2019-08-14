//! A definition of the XY-Chain strategy.

use grid::{Candidate, CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::{Deduction, Step};
use utils::GeneratorAdapter;

#[derive(Clone)]
pub struct XYChainNode {
    cell: CellIdx,
    on_value: Candidate,
    off_value: Candidate,
}

pub type XYChain = Vec<XYChainNode>;

/// Find the XY-Chains that exist in the grid.
///
/// An XY-Chain is a chain of bivalue cells, connected by weak links on common candidates. One of
/// the two candidates at the ends of the chain must be true, so any cells which can see both ends
/// of the chain can have that candidate eliminated.
pub fn find<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        // Find all nodes that need to be considered for this chain - that is, candidates in bivalue
        // cells of the grid.
        let nodes = find_xy_nodes(grid);

        // Create an adjacency matrix representing all the bivalue cells in the grid
        let mut distances = vec![vec![usize::max_value(); nodes.len()]; nodes.len()];
        let mut paths = vec![vec![0; nodes.len()]; nodes.len()];

        for (start_idx, start_node) in nodes.iter().enumerate() {
            for (end_idx, end_node) in nodes.iter().enumerate() {
                if start_idx != end_idx && is_connected(start_node, end_node) {
                    distances[start_idx][end_idx] = 1;
                    paths[start_idx][end_idx] = end_idx;
                }
            }
        }

        // Run a pathfinding algorithm to hunt for XY-Chains
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

        // Check for XY-Chains with eliminations
        let mut xy_chains = Vec::new();

        for start_idx in 0..nodes.len() {
            for end_idx in start_idx + 1..nodes.len() {
                if distances[start_idx][end_idx] < usize::max_value() {
                    if !get_elimination_cells(grid, &nodes[start_idx], &nodes[end_idx]).is_empty() {
                    
                        // Reconstruct the path that makes this XY-Chain
                        let (mut chain, mut current_idx) = (vec![], start_idx);
                        while current_idx != end_idx {
                            chain.push(nodes[current_idx].clone());
                            current_idx = paths[current_idx][end_idx];
                        }
                        chain.push(nodes[end_idx].clone());
                        xy_chains.push(chain);
                    }
                }
            }
        }

        xy_chains.sort_by_key(|chain| chain.len());
        for chain in xy_chains {
            yield Step::XYChain { chain };
        }
    })
}

/// Get the deductions arising from the XY-Chain on the given grid.
pub fn get_deductions(grid: &Grid, xy_chain: &Step) -> Vec<Deduction> {
    match xy_chain {
        Step::XYChain { chain } => {
            let (start_node, end_node) = (&chain[0], &chain[chain.len() - 1]);
            let value = end_node.on_value;
            get_elimination_cells(grid, start_node, end_node).map(|cell| Deduction::Elimination(cell, value))
        },
        _ => unreachable!(),
    }
}

/// Get a concise description of this step, to be used in a description of a solution path.
pub fn get_description(xy_chain: &Step) -> String {
    match xy_chain {
        Step::XYChain { chain } => {
            let mut description = format!("XY-Chain - ={}= ", chain[0].off_value);
            for node in chain.iter() {
                description.push_str(&format!("{} ={}= ", Grid::cell_name(node.cell), node.on_value));
            }
            description
        },
        _ => unreachable!(),
    }
}

/// Check if there is a connection from one node to another in an XY-Chain
fn is_connected(from_node: &XYChainNode, to_node: &XYChainNode) -> bool {
    Grid::neighbours(from_node.cell).contains(to_node.cell) && from_node.on_value == to_node.off_value
}

/// Get the elimination cells for the XY-Chain with the given endpoints.
fn get_elimination_cells(grid: &Grid, start_node: &XYChainNode, end_node: &XYChainNode) -> CellSet {
    if start_node.off_value != end_node.on_value { 
        CellSet::empty() 
    } else {
        let cells_with_candidate = grid.cells_with_candidate(end_node.on_value);
        let possible_eliminations = Grid::neighbours(start_node.cell) & Grid::neighbours(end_node.cell);
        cells_with_candidate & possible_eliminations
    }
}

/// Find all candidates in bivalue cells in the grid.
fn find_xy_nodes(grid: &Grid) -> Vec<XYChainNode> {
    let mut nodes = Vec::new();
    for cell in grid.cells_with_n_candidates(2).iter() {
        let candidates = grid.candidates(cell).map(|x| x);
        nodes.push(XYChainNode { cell, on_value: candidates[0], off_value: candidates[1] });
        nodes.push(XYChainNode { cell, on_value: candidates[1], off_value: candidates[0] });
    }
    nodes
}