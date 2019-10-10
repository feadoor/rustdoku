//! A tool to search for single-step solutions to Sudoku puzzles.

extern crate rustdoku;

use std::io;
use std::io::BufRead;

use rustdoku::define_grid_size;
use rustdoku::grid::{Grid, GridSize};
use rustdoku::grid::cellset::CellSet;
use rustdoku::solver;
use rustdoku::solver::{SolveConfiguration, SolveResult};
use rustdoku::strategies::Strategy::*;

define_grid_size!(Grid9, 9);

fn main() {

    let stdin = io::stdin();

    let singles_only = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle,
    ]);

    let singles_and_line_box_only = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle, BoxLine,
    ]);


    let basic_config = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle, BoxLine, NakedSubset(2), HiddenSubset(2),
        NakedSubset(3), HiddenSubset(3), NakedSubset(4), HiddenSubset(4),
    ]);

    let full_config = SolveConfiguration::with_all_strategies();

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
        
        println!("\n\nInitial grid:\n\n{}\n\n", grid);

        let basics = solver::solve(&mut grid, &basic_config);
        for step in basics.steps {
            println!(" - {}", step.get_description(&grid));
        }

        println!("\n\nAfter solving basics:\n\n{}\n\n", grid);

        if let SolveResult::Solved = basics.result {
            println!("All done!");
            continue;
        }

        for &strategy in full_config.strategies() {
            for step in strategy.find_steps(&grid).filter(|step| step.get_deductions(&grid).len() > 0) {

                let mut secondary_grid = grid.clone();
                for deduction in step.get_deductions(&secondary_grid) {
                    secondary_grid.apply_deduction(deduction);
                }

                let after_singles = solver::solve(&mut secondary_grid, &singles_only);
                if let SolveResult::Solved = after_singles.result {
                    println!("{} - singles to the end", step.get_description(&grid));
                    continue;
                }

                let after_singles_and_line_box = solver::solve(&mut secondary_grid, &singles_and_line_box_only);
                if let SolveResult::Solved = after_singles_and_line_box.result {
                    println!("{} - singles and line-box to the end", step.get_description(&grid));
                    continue;
                }

                let after_basics = solver::solve(&mut secondary_grid, &basic_config);
                if let SolveResult::Solved = after_basics.result {
                    println!("{} - basics to the end", step.get_description(&grid));
                    continue;
                }
            }
        }

        println!("\nEnter a sudoku:");
    }
}
