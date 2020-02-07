//! Processes to generate Sudoku puzzles with a specific pattern of clues.

use itertools::Itertools;
use std::collections::HashSet;
use rand::prelude::*;

use generator::brute_force::BruteForceSolver;
use generator::canonicalization::minlex;
use grid::{Grid, GridSize};

/// A convenience type to represent a pattern of clues within a grid.
type Pattern = Vec<usize>;

/// A convenience type to represent a puzzle.
type Puzzle = Vec<usize>;

/// A structure capable of producing and iterating over puzzles with a particular clue pattern.
pub struct PatternPuzzlesIterator<T: GridSize> {

    /// The starting grid for which puzzles are being generated.
    starting_grid: Grid<T>,

    /// Whether to canonicalize the generated puzzles.
    canonicalize: bool,

    /// A brute-force solver configured to work with this grid
    brute_force_solver: BruteForceSolver,

    /// The stack of seed puzzles still to be examined.
    seed_stack: Vec<Puzzle>,

    /// The queue of puzzles waiting to be returned from the iterator.
    iteration_queue: Vec<Puzzle>,

    /// A set of puzzles already seen.
    seen_puzzles: HashSet<Puzzle>,

    /// The pattern that the clues of generated puzzles will occur in.
    pattern: Pattern,
}

impl <T: GridSize> PatternPuzzlesIterator<T> {

    /// An iterator over puzzles on the given empty grid using a random seed.
    pub fn for_empty_grid_and_pattern(starting_grid: Grid<T>, pattern: Pattern) -> PatternPuzzlesIterator<T> {
        loop {
            if let Some(puzzle) = PatternPuzzlesIterator::random_seed(&starting_grid, &pattern) {
                let brute_force_solver = BruteForceSolver::for_empty_grid(&starting_grid);
                return PatternPuzzlesIterator {
                    starting_grid: starting_grid,
                    canonicalize: true,
                    brute_force_solver: brute_force_solver,
                    seed_stack: vec![puzzle],
                    iteration_queue: vec![],
                    seen_puzzles: HashSet::new(),
                    pattern: pattern,
                }
            }
        }
    }

    /// An iterator over puzzles with the given starting grid using a random seed.
    pub fn for_starting_grid_and_pattern(starting_grid: Grid<T>, pattern: Pattern) -> PatternPuzzlesIterator<T> {
        loop {
            if let Some(puzzle) = PatternPuzzlesIterator::random_seed(&starting_grid, &pattern) {
                let brute_force_solver = BruteForceSolver::for_starting_grid(&starting_grid);
                return PatternPuzzlesIterator {
                    starting_grid: starting_grid,
                    canonicalize: false,
                    brute_force_solver: brute_force_solver,
                    seed_stack: vec![puzzle],
                    iteration_queue: vec![],
                    seen_puzzles: HashSet::new(),
                    pattern: pattern,
                }
            }
        }
    }

    /// Produce a random seed puzzle - possibly without a unique solution - that can be used as the
    /// starting point for a search.
    fn random_seed(starting_grid: &Grid<T>, pattern: &Pattern) -> Option<Puzzle> {
        let mut puzzle = starting_grid.cells().map(|c| starting_grid.value(c).unwrap_or(0));
        for &cell in pattern {
            let valid_clues = PatternPuzzlesIterator::valid_clues(starting_grid, &puzzle, cell);
            if valid_clues.is_empty() { return None; }
            else { puzzle[cell] = *thread_rng().choose(&valid_clues).unwrap(); }
        }
        Some(puzzle)
    }

    /// Find the clues are are valid in the given position, from the current puzzle state.
    fn valid_clues(starting_grid: &Grid<T>, puzzle: &Puzzle, cell: usize) -> Vec<usize> {
        let mut valid = vec![false; 10];
        for candidate in starting_grid.candidates(cell).iter() {
            valid[candidate] = true;
        }
        if let Some(value) = starting_grid.value(cell) {
            valid[value] = true;
        }
        for neighbour in starting_grid.neighbours(cell).iter() {
            valid[puzzle[neighbour]] = false;
        }
        (1..T::size() + 1).filter(|&c| valid[c]).collect()
    }
}

impl <T: GridSize> Iterator for PatternPuzzlesIterator<T> {

    type Item = Puzzle;

    fn next(&mut self) -> Option<Puzzle> {

        // If there are puzzles waiting to be returned, then return the next one.
        if !self.iteration_queue.is_empty() {
            let puzzle = self.iteration_queue.pop().unwrap();
            self.seed_stack.push(puzzle.clone());
            return Some(puzzle);
        }

        loop {

            // If the stack has been exhausted, then we are finished.
            if self.seed_stack.is_empty() {
                loop {
                    if let Some(seed) = Self::random_seed(&self.starting_grid, &self.pattern) {
                        self.seed_stack.push(seed);
                        break;
                    }
                }
            }

            // Find the puzzle at the top of the stack.
            let current_puzzle = self.seed_stack.pop().unwrap();

            // Perform a +2/-2 vicinity search on this puzzle.
            let mut next_puzzles = Vec::new();
            for (&clue1, &clue2) in self.pattern.iter().tuple_combinations() {

                // Set the two clues that will be modified to 0
                let mut puzzle = current_puzzle.clone();
                puzzle[clue1] = 0; puzzle[clue2] = 0;

                // Find the possibilities for each of the 2 modified clues.
                let (poss1, poss2) = (Self::valid_clues(&self.starting_grid, &puzzle, clue1), Self::valid_clues(&self.starting_grid, &puzzle, clue2));

                // Find all of the puzzles that are within the vicinity of the original puzzle.
                for &c1 in &poss1 {
                    puzzle[clue1] = c1;
                    for &c2 in &poss2 {
                        puzzle[clue2] = c2;
                        
                        // Check if the puzzle has a unique solution
                        let canonical_puzzle = if self.canonicalize { minlex::<T>(&puzzle) } else { puzzle.clone() };
                        if !self.seen_puzzles.contains(&canonical_puzzle) && self.brute_force_solver.has_unique_solution(&canonical_puzzle) {
                            self.seen_puzzles.insert(canonical_puzzle.clone());
                            next_puzzles.push(canonical_puzzle);
                        }
                    }
                }
            }

            // Shuffle all of the next puzzles so that the search does not excessively focus on changing
            // the same clues over and over again.
            thread_rng().shuffle(&mut next_puzzles);
            self.iteration_queue.append(&mut next_puzzles);

            // If any puzzles have been produced, then return the next one.
            if let Some(puzzle) = self.iteration_queue.pop() {
                return Some(puzzle);
            }
        }
    }
}