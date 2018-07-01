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

use grid::{CellIdx, Grid};

/// The different types of deduction that can be made on a grid.
#[derive(Copy, Clone)]
pub enum Deduction {
    /// Indicates that the given value can be placed in the cell at the given index.
    Placement(CellIdx, usize),
    /// Indicates that the given value can be eliminated from the cell at the given index.
    Elimination(CellIdx, usize),
    /// Indicates that the grid was found to be in contradiction.
    Contradiction,
}

/// A move that can be made on the grid.
#[derive(Clone)]
pub struct Move {
    /// The placements or eliminations resulting from this move.
    pub deductions: Vec<Deduction>,
    /// A short description of the move that identifies it to the savvy solver.
    pub description: String,
}

impl Move {
    pub fn with_strategy(self, strategy: Strategy) -> AnnotatedMove {
        AnnotatedMove { mov: self, strategy }
    }
}

/// A move, along with the associated strategy that discovered it.
#[derive(Clone)]
pub struct AnnotatedMove {
    /// The move itself.
    pub mov: Move,
    /// The strategy that was used to find the move.
    pub strategy: Strategy,
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
    Strategy::FinnedFish(2),
    Strategy::FinnedFish(3),
    Strategy::Fish(4),
    Strategy::FinnedFish(4),
    Strategy::XYWing,
    Strategy::XYZWing,
    Strategy::WWing,
];

/// Find a deduction arising from the chosen strategy.
pub fn find_move(grid: &Grid, strategy: Strategy) -> Option<AnnotatedMove> {
    (match strategy {
        Strategy::FullHouse => full_house::find(&grid),
        Strategy::HiddenSingle => hidden_single::find(&grid),
        Strategy::NakedSingle => naked_single::find(&grid),
        Strategy::Pointing => pointing::find(&grid),
        Strategy::Claiming => claiming::find(&grid),
        Strategy::HiddenSubset(sz) => hidden_subset::find_with_degree(&grid, sz),
        Strategy::NakedSubset(sz) => naked_subset::find_with_degree(&grid, sz),
        Strategy::Fish(sz) => basic_fish::find_with_degree(&grid, sz),
        Strategy::FinnedFish(sz) => finned_fish::find_with_degree(&grid, sz),
        Strategy::XYWing => xy_wing::find(&grid),
        Strategy::XYZWing => xyz_wing::find(&grid),
        Strategy::WWing => w_wing::find(&grid),
    }).map(|x| x.with_strategy(strategy))
}