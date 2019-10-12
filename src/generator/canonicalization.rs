//! Functions relating to canonicalizing generated puzzles.

use grid::GridSize;

/// Find the minlex variant of the given puzzle - that is, the puzzle which is formed from the
/// given puzzle by relabelling digits in such a way that the puzzle's clues are minimal in the
/// lexicographic ordering.
pub fn minlex<T: GridSize>(puzzle: &[usize]) -> Vec<usize> {

    // Iterate through the puzzle and store the clues in the order they appear.
    let (mut reverse_lookup, mut seen, mut count) = (vec![0; T::size() + 1], vec![false; T::size() + 1], 0);
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
