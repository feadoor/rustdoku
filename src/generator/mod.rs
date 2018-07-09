//! Generate Sudoku puzzles which have a unique solution, using brute-force.

use rand::prelude::*;

mod brute_force;

pub fn generate_puzzle() -> Vec<usize> {
    let mut solution = brute_force::get_random_solution(&[0; 81]).unwrap();
    reduce(&mut solution);
    solution
}

fn reduce(solved_cells: &mut [usize]) {
    let mut cell_order: Vec<_> = (0..solved_cells.len()).collect();
    thread_rng().shuffle(&mut cell_order);
    for cell in cell_order {
        let clue = solved_cells[cell];
        solved_cells[cell] = 0;
        if !brute_force::has_unique_solution(&solved_cells) { solved_cells[cell] = clue; }
    }
}