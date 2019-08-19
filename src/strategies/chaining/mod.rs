//! Common elements of chaining strategies

mod aic;
mod forcing;
mod nodes;

use grid::Grid;
use grid::candidateset::CandidateSet;
use strategies::Step;
use utils::GeneratorAdapter;

pub use self::aic::{Aic, get_aic_deductions, get_aic_description};
pub use self::forcing::{ForcingChain, get_forcing_chain_deductions, get_forcing_chain_description};

pub fn find_xchains<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        for candidate in CandidateSet::full().iter() {

            // Create the possible chain nodes for this candidate
            let mut nodes = nodes::get_value_nodes_for_candidate(grid, candidate);
            nodes.append(&mut nodes::get_group_nodes_for_candidate(grid, candidate));

            // Find the X-Chains
            for chain in aic::find_aics(grid, nodes) {
                yield Step::XChain { chain };
            }
        }
    })
}

pub fn find_aics<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        // Create the possible chain nodes
        let mut nodes = nodes::get_value_nodes(grid);
        nodes.append(&mut nodes::get_group_nodes(grid));

        // Find the AICs
        for chain in aic::find_aics(grid, nodes) {
            yield Step::Aic { chain };
        }
    })
}

pub fn find_als_aics<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        // Create the possible chain nodes
        let mut nodes = nodes::get_value_nodes(grid);
        nodes.append(&mut nodes::get_group_nodes(grid));
        nodes.append(&mut nodes::get_als_nodes(grid));

        // Find the AICs
        for chain in aic::find_aics(grid, nodes) {
            yield Step::AlsAic { chain };
        }
    })
}

pub fn find_forcing_chains<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        // Create the possible chain nodes
        let mut nodes = nodes::get_value_nodes(grid);
        nodes.append(&mut nodes::get_group_nodes(grid));

        // Find the forcing chains
        for chain in forcing::find_forcing_chains(grid, nodes) {
            yield Step::ForcingChain { chain };
        }
    })
}

pub fn find_als_forcing_chains<'a>(grid: &'a Grid) -> impl Iterator<Item = Step> + 'a {

    GeneratorAdapter::of(move || {

        // Create the possible chain nodes
        let mut nodes = nodes::get_value_nodes(grid);
        nodes.append(&mut nodes::get_group_nodes(grid));
        nodes.append(&mut nodes::get_als_nodes(grid));

        // Find the forcing chains
        for chain in forcing::find_forcing_chains(grid, nodes) {
            yield Step::AlsForcingChain { chain };
        }
    })
}