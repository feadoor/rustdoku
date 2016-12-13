//! A pure-logic Sudoku solver.

extern crate bit_set;

mod grid;
mod strategies;

use grid::Grid;
use strategies::solve;

fn main() {
    let mut grid = Grid::from_string("080020006\
                                      000806000\
                                      300000901\
                                      409000000\
                                      050307060\
                                      000000805\
                                      205000009\
                                      000403000\
                                      100070030");

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
            solve(&mut grid);
            check_grid(&grid);
        }
    }
}