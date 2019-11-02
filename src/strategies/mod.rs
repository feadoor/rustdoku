//! Definitions of solving strategies for Sudoku puzzles.

mod full_house;
mod hidden_single;
mod naked_single;
mod box_line;
mod cell_interaction;
mod naked_subset;
mod hidden_subset;
mod basic_fish;

use grid::{CellIdx, Grid, GridSize, RowOrColumn};
use grid::cellset::CellSet;
use grid::candidateset::CandidateSet;
use grid::placementset::PlacementSet;

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
    BoxLine { region: CellSet<T>, eliminations: PlacementSet<T>, value: usize, },
    CellInteraction { cell: CellIdx, eliminations: PlacementSet<T>, },
    HiddenSubset { region: CellSet<T>, cells: CellSet<T>, values: CandidateSet<T>, },
    NakedSubset { region: CellSet<T>, cells: CellSet<T>, values: CandidateSet<T>, },
    Fish { degree: usize, base_type: RowOrColumn, base: CellSet<T>, cover: CellSet<T>, value: usize, },
}

/// The different strategies available to the solver.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Strategy {
    FullHouse,
    HiddenSingle,
    NakedSingle,
    BoxLine,
    CellInteraction,
    HiddenSubset(usize),
    NakedSubset(usize),
    Fish(usize),
}

pub const ALL_STRATEGIES: &'static [Strategy] = &[
    Strategy::FullHouse,
    Strategy::HiddenSingle,
    Strategy::NakedSingle,
    Strategy::BoxLine,
    Strategy::CellInteraction,
    Strategy::NakedSubset(2),
    Strategy::HiddenSubset(2),
    Strategy::NakedSubset(3),
    Strategy::HiddenSubset(3),
    Strategy::NakedSubset(4),
    Strategy::HiddenSubset(4),
    Strategy::Fish(2),
    Strategy::Fish(3),
    Strategy::Fish(4),
];

impl Strategy {

    /// Find a step arising from the chosen strategy.
    pub fn find_steps<'a, T: GridSize>(&self, grid: &'a Grid<T>) -> Box<dyn Iterator<Item = Step<T>> + 'a> {
        match *self {
            Strategy::FullHouse => Box::new(full_house::find(&grid)),
            Strategy::HiddenSingle => Box::new(hidden_single::find(&grid)),
            Strategy::NakedSingle => Box::new(naked_single::find(&grid)),
            Strategy::BoxLine => Box::new(box_line::find(&grid)),
            Strategy::CellInteraction => Box::new(cell_interaction::find(&grid)),
            Strategy::HiddenSubset(sz) => Box::new(hidden_subset::find_with_degree(&grid, sz)),
            Strategy::NakedSubset(sz) => Box::new(naked_subset::find_with_degree(&grid, sz)),
            Strategy::Fish(sz) => Box::new(basic_fish::find_with_degree(&grid, sz)),
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
            ref cell_interaction @ Step::CellInteraction { .. } => cell_interaction::get_deductions(grid, cell_interaction),
            ref hidden_subset @ Step::HiddenSubset { .. } => hidden_subset::get_deductions(grid, hidden_subset),
            ref naked_subset @ Step::NakedSubset { .. } => naked_subset::get_deductions(grid, naked_subset),
            ref fish @ Step::Fish { .. } => basic_fish::get_deductions(grid, fish),
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
            ref cell_interaction @ Step::CellInteraction { .. } => cell_interaction::get_description(grid, cell_interaction),
            ref hidden_subset @ Step::HiddenSubset { .. } => format!("{}", hidden_subset::get_description(grid, hidden_subset)),
            ref naked_subset @ Step::NakedSubset { .. } => format!("{}", naked_subset::get_description(grid, naked_subset)),
            ref fish @ Step::Fish { .. } => format!("{}", basic_fish::get_description(grid, fish)),
        }
    }
}
