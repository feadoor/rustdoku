//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod hidden_single;
mod naked_single;
mod pointing;
mod claiming;
mod naked_subset;
mod hidden_subset;
mod basic_fish;
mod finned_fish;
mod xy_wing;
mod xyz_wing;
mod w_wing;
mod wxyz_wing;
mod chaining;
mod xy_chain;

use grid::{CellIdx, Grid, Region};
use grid::cellset::CellSet;
use grid::candidateset::CandidateSet;
use strategies::chaining::Aic;
use strategies::xy_chain::XYChain;

use std::fmt;

/// The different types of deduction that can be made on a grid.
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
    /// Indicates that the grid was found to be in contradiction.
    Contradiction,
}

/// A step to be taken in the process of solving a given grid.
pub enum Step {
    NoCandidatesForCell { cell: CellIdx },
    NoPlaceForCandidateInRegion { region: CellSet, value: usize},
    FullHouse { region: CellSet, cell: CellIdx, value: usize, },
    HiddenSingle { region: CellSet, cell: CellIdx, value: usize, },
    NakedSingle { cell: CellIdx, value: usize, },
    Pointing { block: CellSet, region: CellSet, value: usize, },
    Claiming { region: CellSet, block: CellSet, value: usize, },
    HiddenSubset { region: CellSet, cells: CellSet, values: CandidateSet, },
    NakedSubset { region: CellSet, cells: CellSet, values: CandidateSet, },
    Fish { degree: usize, base_type: Region, base: CellSet, cover: CellSet, value: usize, },
    FinnedFish { degree: usize, base_type: Region, base: CellSet, cover: CellSet, fins: CellSet, value: usize, },
    XYWing { pivot: CellIdx, pincer1: CellIdx, pincer2: CellIdx, value: usize, },
    XYZWing { pivot: CellIdx, pincer1: CellIdx, pincer2: CellIdx, value: usize, },
    WWing { pincer1: CellIdx, pincer2: CellIdx, region: CellSet, covered_value: usize, eliminated_value: usize, },
    WXYZWing { cells: CellSet, value: usize },
    XChain { chain: Aic },
    XYChain { chain: XYChain },
    Aic { chain: Aic },
}

/// The different strategies available to the solver.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Strategy {
    FullHouse,
    HiddenSingle,
    NakedSingle,
    Pointing,
    Claiming,
    HiddenSubset(usize),
    NakedSubset(usize),
    Fish(usize),
    FinnedFish(usize),
    XYWing,
    XYZWing,
    WWing,
    WXYZWing,
    XChain,
    XYChain,
    Aic,
}

pub const ALL_STRATEGIES: &'static [Strategy] = &[
    Strategy::FullHouse,
    Strategy::HiddenSingle,
    Strategy::NakedSingle,
    Strategy::Pointing,
    Strategy::Claiming,
    Strategy::NakedSubset(2),
    Strategy::HiddenSubset(2),
    Strategy::NakedSubset(3),
    Strategy::HiddenSubset(3),
    Strategy::NakedSubset(4),
    Strategy::HiddenSubset(4),
    Strategy::Fish(2),
    Strategy::Fish(3),
    Strategy::Fish(4),
    Strategy::XYWing,
    Strategy::XYZWing,
    Strategy::FinnedFish(2),
    Strategy::FinnedFish(3),
    Strategy::FinnedFish(4),
    Strategy::XChain,
    Strategy::XYChain,
    Strategy::WWing,
    Strategy::WXYZWing,
    Strategy::Aic,
];

impl Strategy {

    /// Find a step arising from the chosen strategy.
    pub fn find_steps<'a>(&self, grid: &'a Grid) -> Box<dyn Iterator<Item = Step> + 'a> {
        match *self {
            Strategy::FullHouse => Box::new(full_house::find(&grid)),
            Strategy::HiddenSingle => Box::new(hidden_single::find(&grid)),
            Strategy::NakedSingle => Box::new(naked_single::find(&grid)),
            Strategy::Pointing => Box::new(pointing::find(&grid)),
            Strategy::Claiming => Box::new(claiming::find(&grid)),
            Strategy::HiddenSubset(sz) => Box::new(hidden_subset::find_with_degree(&grid, sz)),
            Strategy::NakedSubset(sz) => Box::new(naked_subset::find_with_degree(&grid, sz)),
            Strategy::Fish(sz) => Box::new(basic_fish::find_with_degree(&grid, sz)),
            Strategy::FinnedFish(sz) => Box::new(finned_fish::find_with_degree(&grid, sz)),
            Strategy::XYWing => Box::new(xy_wing::find(&grid)),
            Strategy::XYZWing => Box::new(xyz_wing::find(&grid)),
            Strategy::WWing => Box::new(w_wing::find(&grid)),
            Strategy::WXYZWing => Box::new(wxyz_wing::find(&grid)),
            Strategy::XChain => Box::new(chaining::find_xchains(&grid)),
            Strategy::XYChain => Box::new(xy_chain::find(&grid)),
            Strategy::Aic => Box::new(chaining::find_aics(&grid)),
        }
    }
}

impl Step {

    /// Find the deductions given by the step.
    pub fn get_deductions(&self, grid: &Grid) -> Vec<Deduction> {
        match self {
            Step::NoCandidatesForCell { .. } => vec![Deduction::Contradiction],
            Step::NoPlaceForCandidateInRegion { .. } => vec![Deduction::Contradiction],
            ref full_house @ Step::FullHouse { .. } => full_house::get_deductions(&grid, full_house),
            ref hidden_single @ Step::HiddenSingle { .. } => hidden_single::get_deductions(&grid, hidden_single),
            ref naked_single @ Step::NakedSingle { .. } => naked_single::get_deductions(&grid, naked_single),
            ref pointing @ Step::Pointing { .. } => pointing::get_deductions(&grid, pointing),
            ref claiming @ Step::Claiming { .. } => claiming::get_deductions(&grid, claiming),
            ref hidden_subset @ Step::HiddenSubset { .. } => hidden_subset::get_deductions(&grid, hidden_subset),
            ref naked_subset @ Step::NakedSubset { .. } => naked_subset::get_deductions(&grid, naked_subset),
            ref fish @ Step::Fish { .. } => basic_fish::get_deductions(&grid, fish),
            ref finned_fish @ Step::FinnedFish { .. } => finned_fish::get_deductions(&grid, finned_fish),
            ref xy_wing @ Step::XYWing { .. } => xy_wing::get_deductions(&grid, xy_wing),
            ref xyz_wing @ Step::XYZWing { .. } => xyz_wing::get_deductions(&grid, xyz_wing),
            ref w_wing @ Step::WWing { .. } => w_wing::get_deductions(&grid, w_wing),
            ref wxyz_wing @ Step::WXYZWing { .. } => wxyz_wing::get_deductions(&grid, &wxyz_wing),
            Step::XChain { chain } => chaining::get_aic_deductions(&grid, &chain),
            ref xy_chain @ Step::XYChain { .. } => xy_chain::get_deductions(&grid, &xy_chain),
            Step::Aic { chain } => chaining::get_aic_deductions(&grid, &chain),
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::NoCandidatesForCell { cell } => write!(f, "No candidates remain for cell {}", Grid::cell_name(*cell)),
            Step::NoPlaceForCandidateInRegion { region, value } => write!(f, "No place for {} in {}", value, Grid::region_name(&region)),
            ref full_house @ Step::FullHouse { .. } => write!(f, "{}", full_house::get_description(&full_house)),
            ref hidden_single @ Step::HiddenSingle { .. } => write!(f, "{}", hidden_single::get_description(&hidden_single)),
            ref naked_single @ Step::NakedSingle { .. } => write!(f, "{}", naked_single::get_description(&naked_single)),
            ref pointing @ Step::Pointing { .. } => write!(f, "{}", pointing::get_description(&pointing)),
            ref claiming @ Step::Claiming { .. } => write!(f, "{}", claiming::get_description(&claiming)),
            ref hidden_subset @ Step::HiddenSubset { .. } => write!(f, "{}", hidden_subset::get_description(&hidden_subset)),
            ref naked_subset @ Step::NakedSubset { .. } => write!(f, "{}", naked_subset::get_description(&naked_subset)),
            ref fish @ Step::Fish { .. } => write!(f, "{}", basic_fish::get_description(&fish)),
            ref finned_fish @ Step::FinnedFish { .. } => write!(f, "{}", finned_fish::get_description(&finned_fish)),
            ref xy_wing @ Step::XYWing { .. } => write!(f, "{}", xy_wing::get_description(&xy_wing)),
            ref xyz_wing @ Step::XYZWing { .. } => write!(f, "{}", xyz_wing::get_description(&xyz_wing)),
            ref w_wing @ Step::WWing { .. } => write!(f, "{}", w_wing::get_description(&w_wing)),
            ref wxyz_wing @ Step::WXYZWing { .. } => write!(f, "{}", wxyz_wing::get_description(&wxyz_wing)),
            Step::XChain { chain } => write!(f, "X-Chain - {}", chaining::get_aic_description(chain)),
            ref xy_chain @ Step::XYChain { .. } => write!(f, "{}", xy_chain::get_description(&xy_chain)),
            Step::Aic { chain } => write!(f, "AIC - {}", chaining::get_aic_description(chain)),
        }
    }
}
