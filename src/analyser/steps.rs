//! Analysis that determines how many "steps" are needed to solve a puzzle.

use grid::Grid;
use strategies::{Deduction, Step, Strategy};

/// Determine how many "steps" are needed to solve the given puzzle.
///
/// Allowable techniques are given in ordered groups. A single step involves
/// finding the first group that contains a technique that can be applied to
/// the puzzle, finding all deductions arising from techniques in that group,
/// and applying them all simultaneously.
pub fn steps_to_solve(grid: &Grid, strategies: &[Vec<Strategy>]) -> Option<Vec<usize>> {

    let mut steps_taken = vec![0; strategies.len()]; let mut working_grid = grid.clone();
    'outer: while !working_grid.is_solved() {

        for (idx, strategy_group) in strategies.iter().enumerate() {
            let moves: Vec<Step> = strategy_group.iter().flat_map(|strat| strat.find_steps(&working_grid)).collect();
            let deductions: Vec<Deduction> = moves.iter().flat_map(|mov| mov.get_deductions(&working_grid)).collect();
            if !deductions.is_empty() {
                for deduction in deductions {
                    working_grid.apply_deduction(deduction);
                }
                steps_taken[idx] += 1; continue 'outer;
            }
        }

        break;

    }

    if working_grid.is_solved() { Some(steps_taken) } else { None }

}