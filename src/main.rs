//! A pure-logic Sudoku solver.

extern crate ansi_term;
extern crate bit_set;
extern crate itertools;

mod grid;
mod solver;
mod strategies;

use std::io;
use std::io::prelude::*;

use grid::Grid;
use strategies::Deduction::*;

fn main() {
    let stdin = io::stdin();
    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::from_str(&line.unwrap());
        if grid_result.is_ok() {
            let mut grid = grid_result.ok().unwrap();

            while !grid.is_solved() {
                println!("\n{}\n", grid);
                if let Some(mov) = solver::find_move(&grid) {
                    println!("  - {}", mov.description);
                    for deduction in mov.deductions {
                        if let Contradiction = deduction {
                            break;
                        } else {
                            grid.apply_deduction(deduction);
                        }
                    }
                } else {
                    println!("The known strategies are insufficient to make further progress");
                    break;
                }
            }

            println!("\nFinal result:\n\n{}", grid);
        } else {
            println!("{}", grid_result.err().unwrap());
        }
        println!("\nEnter a sudoku:");
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use super::*;

    fn solve(grid: &mut Grid) {
        while !grid.is_solved() {
            if let Some(mov) = solver::find_move(grid) {
                for deduction in mov.deductions {
                    if let Contradiction = deduction {
                        return;
                    } else {
                        grid.apply_deduction(deduction);
                    }
                }
            } else {
                return;
            }
        }
    }

    fn check_grid(grid: &Grid) {
        // Check that each value appears in every region in the grid.
        for region in Grid::regions() {
            for &value in Grid::values() {
                assert!(region.iter().any(|x| grid.value(x) == Some(value)));
            }
        }
    }

    #[test]
    fn test_solves() {
        let file = File::open(&Path::new("grids.txt")).unwrap();
        let reader = BufReader::new(file);
        for line_it in reader.lines() {
            let line = line_it.unwrap();
            if !line.is_empty() && !line.starts_with("//") {
                let mut grid = Grid::from_str(&line).unwrap();
                solve(&mut grid);
                check_grid(&grid);
            }
        }
    }
}