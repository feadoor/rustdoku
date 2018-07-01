//! A logical solver for Sudoku

use grid::Grid;

use strategies;
use strategies::Strategy::*;
use strategies::Move;

pub fn find_move(grid: &Grid) -> Option<Move> {

    macro_rules! search {
        ($strat: expr) => {
            let mov = strategies::find_move(grid, $strat);
            if mov.is_some() {
                return mov;
            }
        }
    }

    search!(FullHouse);
    search!(HiddenSingle);
    search!(NakedSingle);
    search!(Pointing);
    search!(Claiming);
    search!(NakedSubset(2));
    search!(HiddenSubset(2));
    search!(NakedSubset(3));
    search!(HiddenSubset(3));
    search!(NakedSubset(4));
    search!(HiddenSubset(4));
    search!(Fish(2));
    search!(Fish(3));
    search!(FinnedFish(2));
    search!(FinnedFish(3));
    search!(Fish(4));
    search!(FinnedFish(4));
    search!(XYWing);
    search!(XYZWing);
    search!(WWing);

    None
}
