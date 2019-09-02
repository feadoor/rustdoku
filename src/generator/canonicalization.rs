//! Functions relating to canonicalizing generated puzzles.

use generator::brute_force;

/// Find the minlex variant of the given puzzle - that is, the puzzle which is formed from the
/// given puzzle by relabelling digits in such a way that the puzzle's clue are minimal in the
/// lexicographic ordering.
pub fn minlex(puzzle: &[usize]) -> Vec<usize> {

    // Iterate through the puzzle and store the clues in the order they appear.
    let (mut reverse_lookup, mut seen, mut count) = (vec![0; 10], vec![false; 10], 0);
    for &clue in puzzle {
        if clue != 0 && !seen[clue] {
            seen[clue] = true;
            count += 1;
            reverse_lookup[clue] = count;
        }
    }

    // Rewrite the puzzle using the clue ordering that we just worked out.
    let mut new_puzzle = vec![0; puzzle.len()];
    for (idx, &clue) in puzzle.iter().enumerate() {
        new_puzzle[idx] = reverse_lookup[clue];
    }

    new_puzzle
}

/// Check if the given puzzle is minimal - that is, no clues can be removed without creating an
/// invalid puzzle.
pub fn is_minimal(puzzle: &[usize]) -> bool {
    let mut modified_puzzle = puzzle.to_vec();
    for (idx, &clue) in puzzle.iter().enumerate() {
        if clue != 0 {
            modified_puzzle[idx] = 0;
            if brute_force::has_unique_solution(&modified_puzzle) { return false; }
            modified_puzzle[idx] = clue;
        }
    }
    true
}