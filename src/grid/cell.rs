//! A structure representing a single cell within a Sudoku grid.

// Bitmasks representing the possible candidates 1-9 that can be in a cell.
static MASKS: [usize; 10] = [
    0x000, 0x001, 0x002, 0x004, 0x008,
    0x010, 0x020, 0x040, 0x080, 0x100,
];

// A bitmask representing a cell with all candidates filled.
static MASK_ALL: usize = 0x1ff;

/// A single cell within a Sudoku grid.
#[derive(Copy, Clone)]
pub struct Cell {
    /// The value, if any, held by this `Cell`.
    value: Option<usize>,
    /// The potential values that this `Cell` can hold.
    candidates: usize,
}

impl Cell {
    /// Create a new `Cell` with no value, and with all candidates possible.
    pub fn new() -> Cell {
        Cell { value: None, candidates: MASK_ALL }
    }

    /// Get the value currently held in this `Cell`.
    pub fn value(&self) -> Option<usize> {
        self.value
    }

    /// Set the value currently held in this `Cell`.
    pub fn set_value(&mut self, val: usize) {
        self.value = Some(val);
        self.candidates = MASKS[val];
    }

    /// Determine whether this `Cell` is empty or not.
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Get the candidates which are allowed in this `Cell`.
    pub fn candidates(&self) -> usize {
        self.candidates
    }

    /// Remove a potential candidate from this `Cell`.
    pub fn remove_candidate(&mut self, val: usize) {
        self.candidates &= !MASKS[val];
    }

    /// Check if a given candidate is allowed in this `Cell`.
    pub fn has_candidate(&self, val: usize) -> bool {
        self.candidates & MASKS[val] != 0
    }

    /// Get the first candidate that can go in this `Cell`.
    pub fn first_candidate(&self) -> usize {
        self.candidates.trailing_zeros() as usize + 1
    }

    /// Get the number of candidates that can go in this `Cell`.
    pub fn num_candidates(&self) -> usize {
        self.candidates.count_ones() as usize
    }
}
