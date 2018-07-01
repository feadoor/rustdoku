//! A logical solver for Sudoku

use grid::Grid;

use strategies;
use strategies::Move;

pub fn find_move(grid: &Grid) -> Option<Move> {

    for &strategy in strategies::ALL_STRATEGIES {
        let mov = strategies::find_move(grid, strategy);
        if mov.is_some() {
            return mov;
        }
    }

    None
}
