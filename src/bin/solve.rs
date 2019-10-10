//! A pure-logic Sudoku solver.

extern crate rustdoku;

use std::io;
use std::io::BufRead;

use rustdoku::define_grid_size;
use rustdoku::grid::{Grid, GridSize};
use rustdoku::grid::cellset::CellSet;
use rustdoku::solver;
use rustdoku::solver::SolveConfiguration;

define_grid_size!(Grid9, 9);

fn main() {

    let stdin = io::stdin();
    let config = SolveConfiguration::with_all_strategies();

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {

        let entered_string = line.unwrap();
        if entered_string.len() != 81 {
            println!("The grid must consist of 81 characters");
            continue;
        }

        let clues: Vec<usize> = entered_string.bytes().map(|byte| match byte {
            b'1'..=b'9' => (byte - b'0') as usize,
            _ => 0,
        }).collect();

        let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 81]);
        for (idx, clue) in clues.iter().enumerate() {
            if *clue > 0 { grid.place_value(idx, *clue); }
        }

        println!("\nInitial grid:\n\n{}", grid);
        let solve_details = solver::solve(&mut grid, &config);
        for step in solve_details.steps {
            println!(" - {}", step.get_description(&grid));
        }
        println!("\nResult: {:?}", solve_details.result);
        println!("\nFinal grid:\n\n{}", grid);
        

        println!("\nEnter a sudoku:");
    }
}
