//! Generate Sudoku puzzles which have a unique solution, using brute-force.

mod brute_force;
mod canonicalization;
mod patterns;

use generator::patterns::PatternPuzzlesIterator;
use grid::{Grid, GridSize};

pub fn generate_puzzles_for_grid_with_pattern<T: GridSize>(grid: Grid<T>, pattern: Vec<usize>) -> impl Iterator<Item = Vec<usize>> {
    PatternPuzzlesIterator::for_empty_grid_and_pattern(grid, pattern)
}
