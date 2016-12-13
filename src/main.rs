//! A pure-logic Sudoku solver.

extern crate bit_set;

mod grid;
mod strategies;

use grid::Grid;

fn main() {
    let mut grid = Grid::new([[0, 0, 5, 0, 7, 9, 0, 0, 3],
                              [2, 0, 0, 0, 0, 0, 0, 0, 0],
                              [3, 4, 8, 0, 0, 0, 0, 0, 0],
                              [0, 5, 0, 6, 8, 0, 0, 0, 0],
                              [0, 7, 0, 2, 0, 4, 0, 8, 0],
                              [0, 0, 0, 0, 1, 3, 0, 2, 0],
                              [0, 0, 0, 0, 0, 0, 4, 7, 1],
                              [0, 0, 0, 0, 0, 0, 0, 0, 6],
                              [8, 0, 0, 7, 9, 0, 3, 0, 0]]);

    println!("Before solving:\n\n{}", grid);
    grid.solve();
    println!("\nAfter solving:\n\n{}", grid);
}
