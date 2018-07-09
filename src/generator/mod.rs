//! Generate Sudoku puzzles which have a unique solution, using brute-force.

use rand::prelude::*;

mod brute_force;

pub fn generate_minimal_puzzle() -> Vec<usize> {
    let mut solution = brute_force::get_random_solution();

    let mut cell_order: Vec<_> = (0..solution.len()).collect();
    thread_rng().shuffle(&mut cell_order);

    for cell in cell_order {
        let clue = solution[cell];
        solution[cell] = 0;
        if !brute_force::has_unique_solution(&solution) { solution[cell] = clue; }
    }

    solution
}

pub fn generate_minimal_symmetric_puzzle() -> Vec<usize> {
    let mut solution = brute_force::get_random_solution();

    let mut cell_order: Vec<_> = (0..(solution.len() + 1) / 2).collect();
    thread_rng().shuffle(&mut cell_order);

    for cell in cell_order {
        let (cell1, cell2) = (cell, solution.len() - cell - 1);
        let (clue1, clue2) = (solution[cell1], solution[cell2]);
        solution[cell1] = 0; solution[cell2] = 0;
        if !brute_force::has_unique_solution(&solution) { solution[cell1] = clue1; solution[cell2] = clue2; }
    }

    solution
}

pub fn generate_puzzle_with_pattern(pattern: &[bool]) -> Option<Vec<usize>> {
    let max_retries = 1_000;
    for _ in 0..max_retries {
        let mut solution = brute_force::get_random_solution();
        for (cell, val) in pattern.iter().enumerate() {
            if !val { solution[cell] = 0; }
        }
        if brute_force::has_unique_solution(&solution) {
            return Some(solution);
        }
    }
    None
}