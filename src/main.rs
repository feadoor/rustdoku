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
use solver::solve;

fn main() {
    let stdin = io::stdin();
    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::from_str(&line.unwrap());
        if grid_result.is_ok() {
            let mut grid = grid_result.ok().unwrap();
            println!("Before solving:\n\n{}", grid);
            println!("\nSolve result: {:?}", solve(&mut grid));
            println!("\nAfter solving:\n\n{}", grid);
        } else {
            println!("{}", grid_result.err().unwrap());
        }
        println!("\nEnter a sudoku:");
    }
}