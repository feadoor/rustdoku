//! A tool to search for single-step solutions to Sudoku puzzles.

extern crate rustdoku;

use std::io;
use std::io::BufRead;

use rustdoku::grid::Grid;
use rustdoku::solver;
use rustdoku::solver::{SolveConfiguration, SolveResult};
use rustdoku::strategies::Strategy::*;

fn main() {
    
    let stdin = io::stdin();

    let singles_only = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle,
    ]);

    let singles_and_line_box_only = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle, Pointing, Claiming,
    ]);


    let basic_config = SolveConfiguration::with_strategies(vec![
        FullHouse, HiddenSingle, NakedSingle, Pointing, Claiming, NakedSubset(2), NakedSubset(3), 
        NakedSubset(4), HiddenSubset(2), HiddenSubset(3), HiddenSubset(4),
    ]);

    let full_config = SolveConfiguration::with_all_strategies();

    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::from_str(&line.unwrap());
        if grid_result.is_ok() {

            let mut grid = grid_result.unwrap();
            println!("\n\nInitial grid:\n\n{}\n\n", grid);

            let basics = solver::solve(&mut grid, &basic_config);
            for step in basics.steps {
                println!(" - {}", step);
            }

            println!("\n\nAfter solving basics:\n\n{}\n\n", grid);

            if let SolveResult::Solved = basics.result {
                println!("All done!");
                continue;
            }

            let mut next_steps = Vec::new();
            for &strategy in full_config.strategies() {
                next_steps.extend(strategy.find_steps(&grid).filter(|step| step.get_deductions(&grid).len() > 0));
            }

            for step in next_steps {
                let mut secondary_grid = grid.clone();
                for deduction in step.get_deductions(&secondary_grid) {
                    secondary_grid.apply_deduction(deduction);
                }
                
                let after_singles = solver::solve(&mut secondary_grid, &singles_only);
                if let SolveResult::Solved = after_singles.result {
                    println!("{} - singles to the end", step);
                    continue;
                }

                let after_singles_and_line_box = solver::solve(&mut secondary_grid, &singles_and_line_box_only);
                if let SolveResult::Solved = after_singles_and_line_box.result {
                    println!("{} - singles and line-box to the end", step);
                    continue;
                }

                let after_basics = solver::solve(&mut secondary_grid, &singles_and_line_box_only);
                if let SolveResult::Solved = after_basics.result {
                    println!("{} - basics to the end", step);
                    continue;
                }
            }

        } else {
            println!("{}", grid_result.err().unwrap());
        }

        println!("\nEnter a sudoku:");
    }
}
