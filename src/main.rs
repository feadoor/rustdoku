//! A pure-logic Sudoku solver.

extern crate bit_set;
extern crate itertools;

mod grid;
mod strategies;

use grid::Grid;
use strategies::solve;

fn main() {
    let mut grid = Grid::from_str("000005004\
                                   000000910\
                                   000900038\
                                   000304507\
                                   070080060\
                                   803502000\
                                   490003000\
                                   025000000\
                                   600700000")
      .unwrap();

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