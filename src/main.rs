//! A pure-logic Sudoku solver.

extern crate bit_set;

mod grid;
mod strategies;

use grid::Grid;
use strategies::solve;

fn main() {
    let mut grid = Grid::from_string("082050000\
                                      700009060\
                                      000040000\
                                      009306007\
                                      200000003\
                                      600205800\
                                      000070000\
                                      010400008\
                                      000030410");

    println!("Before solving:\n\n{}", grid);
    solve(&mut grid);
    println!("\nAfter solving:\n\n{}", grid);
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use super::*;

    fn check_grid(grid: &Grid) {
        // Check that each value appears in every region in the grid.
        for region in grid.regions() {
            for value in 1..region.cells().len() + 1 {
                assert!(region.cells().iter().any(|x| x.value() == Some(value)));
            }
        }
    }

    #[test]
    fn test_solves() {
        let file = File::open(&Path::new("grids.txt")).unwrap();
        let reader = BufReader::new(file);
        for line_it in reader.lines() {
            let mut grid = Grid::from_string(&line_it.unwrap());
            println!("Before solving:\n\n{}", grid);
            solve(&mut grid);
            println!("\nAfter solving:\n\n{}", grid);
            check_grid(&grid);
        }
    }
}