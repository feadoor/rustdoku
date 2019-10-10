//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod hidden_single;
mod naked_single;
mod box_line;
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

use grid::{CellIdx, Grid, GridSize, RowOrColumn};
use grid::cellset::CellSet;
use grid::candidateset::CandidateSet;
use strategies::chaining::Aic;
use strategies::chaining::ForcingChain;
use strategies::xy_chain::XYChain;

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
pub enum Step<T: GridSize> {
    NoCandidatesForCell { cell: CellIdx },
    NoPlaceForCandidateInRegion { region: CellSet<T>, value: usize},
    FullHouse { region: CellSet<T>, cell: CellIdx, value: usize, },
    HiddenSingle { region: CellSet<T>, cell: CellIdx, value: usize, },
    NakedSingle { cell: CellIdx, value: usize, },
    BoxLine { region: CellSet<T>, neighbours: CellSet<T>, value: usize, },
    HiddenSubset { region: CellSet<T>, cells: CellSet<T>, values: CandidateSet<T>, },
    NakedSubset { region: CellSet<T>, cells: CellSet<T>, values: CandidateSet<T>, },
    Fish { degree: usize, base_type: RowOrColumn, base: CellSet<T>, cover: CellSet<T>, value: usize, },
    FinnedFish { degree: usize, base_type: RowOrColumn, base: CellSet<T>, cover: CellSet<T>, fins: CellSet<T>, value: usize, },
    XYWing { pivot: CellIdx, pincer1: CellIdx, pincer2: CellIdx, value: usize, },
    XYZWing { pivot: CellIdx, pincer1: CellIdx, pincer2: CellIdx, value: usize, },
    WWing { pincer1: CellIdx, pincer2: CellIdx, region: CellSet<T>, covered_value: usize, eliminated_value: usize, },
    WXYZWing { cells: CellSet<T>, value: usize },
    XChain { chain: Aic<T> },
    XYChain { chain: XYChain },
    Aic { chain: Aic<T> },
    AlsAic { chain: Aic<T> },
    ForcingChain { chain: ForcingChain<T> },
    AlsForcingChain { chain: ForcingChain<T> },
}

/// The different strategies available to the solver.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Strategy {
    FullHouse,
    HiddenSingle,
    NakedSingle,
    BoxLine,
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
    AlsAic,
    ForcingChain,
    AlsForcingChain,
}

pub const ALL_STRATEGIES: &'static [Strategy] = &[
    Strategy::FullHouse,
    Strategy::HiddenSingle,
    Strategy::NakedSingle,
    Strategy::BoxLine,
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
    Strategy::AlsAic,
    Strategy::ForcingChain,
    Strategy::AlsForcingChain,
];

impl Strategy {

    /// Find a step arising from the chosen strategy.
    pub fn find_steps<'a, T: GridSize>(&self, grid: &'a Grid<T>) -> Box<dyn Iterator<Item = Step<T>> + 'a> {
        match *self {
            Strategy::FullHouse => Box::new(full_house::find(&grid)),
            Strategy::HiddenSingle => Box::new(hidden_single::find(&grid)),
            Strategy::NakedSingle => Box::new(naked_single::find(&grid)),
            Strategy::BoxLine => Box::new(box_line::find(&grid)),
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
            Strategy::AlsAic => Box::new(chaining::find_als_aics(&grid)),
            Strategy::ForcingChain => Box::new(chaining::find_forcing_chains(&grid)),
            Strategy::AlsForcingChain => Box::new(chaining::find_als_forcing_chains(&grid)),
        }
    }
}

impl <T: GridSize> Step<T> {

    /// Find the deductions given by the step.
    pub fn get_deductions(&self, grid: &Grid<T>) -> Vec<Deduction> {
        match self {
            Step::NoCandidatesForCell { .. } => vec![Deduction::Contradiction],
            Step::NoPlaceForCandidateInRegion { .. } => vec![Deduction::Contradiction],
            ref full_house @ Step::FullHouse { .. } => full_house::get_deductions(grid, full_house),
            ref hidden_single @ Step::HiddenSingle { .. } => hidden_single::get_deductions(grid, hidden_single),
            ref naked_single @ Step::NakedSingle { .. } => naked_single::get_deductions(grid, naked_single),
            ref box_line @ Step::BoxLine { .. } => box_line::get_deductions(grid, box_line),
            ref hidden_subset @ Step::HiddenSubset { .. } => hidden_subset::get_deductions(grid, hidden_subset),
            ref naked_subset @ Step::NakedSubset { .. } => naked_subset::get_deductions(grid, naked_subset),
            ref fish @ Step::Fish { .. } => basic_fish::get_deductions(grid, fish),
            ref finned_fish @ Step::FinnedFish { .. } => finned_fish::get_deductions(grid, finned_fish),
            ref xy_wing @ Step::XYWing { .. } => xy_wing::get_deductions(grid, xy_wing),
            ref xyz_wing @ Step::XYZWing { .. } => xyz_wing::get_deductions(grid, xyz_wing),
            ref w_wing @ Step::WWing { .. } => w_wing::get_deductions(grid, w_wing),
            ref wxyz_wing @ Step::WXYZWing { .. } => wxyz_wing::get_deductions(grid, wxyz_wing),
            Step::XChain { chain } => chaining::get_aic_deductions(grid, chain),
            ref xy_chain @ Step::XYChain { .. } => xy_chain::get_deductions(grid, xy_chain),
            Step::Aic { chain } => chaining::get_aic_deductions(grid, chain),
            Step::AlsAic { chain } => chaining::get_aic_deductions(grid, chain),
            Step::ForcingChain { chain } => chaining::get_forcing_chain_deductions(grid, chain),
            Step::AlsForcingChain { chain } => chaining::get_forcing_chain_deductions(grid, chain),
        }
    }

    /// Get a readable description of the step.
    pub fn get_description(&self, grid: &Grid<T>) -> String {
        match self {
            Step::NoCandidatesForCell { cell } => format!("No candidates remain for cell {}", grid.cell_name(*cell)),
            Step::NoPlaceForCandidateInRegion { region, value } => format!("No place for {} in {}", value, grid.region_name(&region)),
            ref full_house @ Step::FullHouse { .. } => format!("{}", full_house::get_description(grid, full_house)),
            ref hidden_single @ Step::HiddenSingle { .. } => format!("{}", hidden_single::get_description(grid, hidden_single)),
            ref naked_single @ Step::NakedSingle { .. } => format!("{}", naked_single::get_description(grid, naked_single)),
            ref box_line @ Step::BoxLine { .. } => format!("{}", box_line::get_description(grid, box_line)),
            ref hidden_subset @ Step::HiddenSubset { .. } => format!("{}", hidden_subset::get_description(grid, hidden_subset)),
            ref naked_subset @ Step::NakedSubset { .. } => format!("{}", naked_subset::get_description(grid, naked_subset)),
            ref fish @ Step::Fish { .. } => format!("{}", basic_fish::get_description(grid, fish)),
            ref finned_fish @ Step::FinnedFish { .. } => format!("{}", finned_fish::get_description(grid, finned_fish)),
            ref xy_wing @ Step::XYWing { .. } => format!("{}", xy_wing::get_description(grid, xy_wing)),
            ref xyz_wing @ Step::XYZWing { .. } => format!("{}", xyz_wing::get_description(grid, xyz_wing)),
            ref w_wing @ Step::WWing { .. } => format!("{}", w_wing::get_description(grid, w_wing)),
            ref wxyz_wing @ Step::WXYZWing { .. } => format!("{}", wxyz_wing::get_description(grid, wxyz_wing)),
            Step::XChain { chain } => format!("X-Chain - {}", chaining::get_aic_description(grid, chain)),
            ref xy_chain @ Step::XYChain { .. } => format!("{}", xy_chain::get_description(grid, xy_chain)),
            Step::Aic { chain } => format!("AIC - {}", chaining::get_aic_description(grid, chain)),
            Step::AlsAic { chain } => format!("ALS AIC - {}", chaining::get_aic_description(grid, chain)),
            Step::ForcingChain { chain } => format!("Forcing Chain - {}", chaining::get_forcing_chain_description(grid, chain)),
            Step::AlsForcingChain { chain } => format!("ALS Forcing Chain - {}", chaining::get_forcing_chain_description(grid, chain)),
        }
    }
}
