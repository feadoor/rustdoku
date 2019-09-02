//! Generate Sudoku puzzles which have a unique solution, using brute-force.

mod brute_force;
mod canonicalization;
mod patterns;
mod solution_generator;

use rand::prelude::*;

use generator::solution_generator::generate_solved_grid;
use generator::patterns::PatternPuzzlesIterator;
use utils::GeneratorAdapter;

pub fn generate_minimal_puzzles() -> impl Iterator<Item = Vec<usize>> {

    GeneratorAdapter::of(move || { loop {
        let mut solution = generate_solved_grid();

        let mut cell_order: Vec<_> = (0..solution.len()).collect();
        thread_rng().shuffle(&mut cell_order);

        for cell in cell_order {
            let clue = solution[cell];
            solution[cell] = 0;
            if !brute_force::has_unique_solution(&solution) { solution[cell] = clue; }
        }

        yield solution;
    }})
}

pub fn generate_minimal_symmetric_puzzles() -> impl Iterator<Item = Vec<usize>> {

    GeneratorAdapter::of(move || { loop {
        let mut solution = generate_solved_grid();

        let mut cell_order: Vec<_> = (0..(solution.len() + 1) / 2).collect();
        thread_rng().shuffle(&mut cell_order);

        for cell in cell_order {
            let (cell1, cell2) = (cell, solution.len() - cell - 1);
            let (clue1, clue2) = (solution[cell1], solution[cell2]);
            solution[cell1] = 0; solution[cell2] = 0;
            if !brute_force::has_unique_solution(&solution) { solution[cell1] = clue1; solution[cell2] = clue2; }
        }

        yield solution;
    }})
}

pub fn generate_puzzles_with_pattern(pattern: Vec<usize>) -> impl Iterator<Item = Vec<usize>> {
    PatternPuzzlesIterator::for_pattern(pattern)
}
