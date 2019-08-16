//! A pure-logic Sudoku solver.

extern crate rustdoku;

use std::io;
use std::io::BufRead;

use rustdoku::grid::Grid;
use rustdoku::solver;
use rustdoku::solver::SolveConfiguration;

fn main() {

    let stdin = io::stdin();
    let config = SolveConfiguration::with_all_strategies();

    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::from_str(&line.unwrap());
        if grid_result.is_ok() {
            let mut grid = grid_result.unwrap();
            println!("\nInitial grid:\n\n{}", grid);
            let solve_details = solver::solve(&mut grid, &config);
            for step in solve_details.steps {
                println!(" - {}", step);
            }
            println!("\nResult: {:?}", solve_details.result);
            println!("\nFinal grid:\n\n{}", grid);
        } else {
            println!("{}", grid_result.err().unwrap());
        }
        println!("\nEnter a sudoku:");
    }
}
