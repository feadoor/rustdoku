//! A pure-logic Sudoku solver.

extern crate ansi_term;
extern crate bit_set;
extern crate itertools;

use std::io;
use std::io::BufRead;

mod grid;
mod solver;
mod strategies;

use grid::Grid;
use solver::SolveConfiguration;

fn main() {
    let stdin = io::stdin();
    let config = SolveConfiguration::with_strategies(strategies::ALL_STRATEGIES.to_vec());

    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::from_str(&line.unwrap());
        if grid_result.is_ok() {
            let mut grid = grid_result.unwrap();
            println!("\nInitial grid:\n\n{}", grid);
            let solve_details = solver::solve(&mut grid, &config);
            for mov in solve_details.moves {
                println!("  - {}", mov.mov.description);
            }
            println!("\nResult: {:?}", solve_details.result);
            println!("\nFinal grid:\n\n{}", grid);
        } else {
            println!("{}", grid_result.err().unwrap());
        }
        println!("\nEnter a sudoku:");
    }
}
