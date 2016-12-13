//! A structure representing a single cell within a Sudoku grid.

use bit_set::BitSet;

use grid::CellIdx;

/// A single cell within a Sudoku grid.
#[derive(Default)]
pub struct Cell {
    /// The value, if any, held by this `Cell`.
    value: Option<usize>,
    /// The potential values that this `Cell` can hold.
    candidates: BitSet,
    /// The coordinates of this cell within its grid.
    idx: CellIdx,
}

impl Cell {

    /// Create a new `Cell` at the given index with no value, and with all candidates possible.
    pub fn new(idx: CellIdx) -> Cell {
        let candidates = BitSet::from_bytes(&[0b01111111, 0b11000000]);
        Cell {
            value : None,
            candidates: candidates,
            idx: CellIdx::new(idx.row, idx.col),
        }
    }

    /// Get the value currently held in this `Cell`.
    pub fn value(&self) -> Option<usize> {
        self.value
    }

    /// Set the value currently held in this `Cell`.
    pub fn set_value(&mut self, val: usize) {
        self.value = Some(val);
        self.candidates.clear();
    }

    /// Determine whether this `Cell` is empty or not.
    pub fn is_empty(&self) -> bool {
        self.value == None
    }

    /// Get the candidates which are allowed in this `Cell`.
    pub fn candidates(&self) -> &BitSet {
        &self.candidates
    }

    /// Remove a potential candidate from this `Cell`.
    pub fn remove_candidate(&mut self, val: usize) {
        self.candidates.remove(val);
    }

    /// Check if a given candidate is allowed in this `Cell`.
    pub fn has_candidate(&self, val: usize) -> bool {
        self.candidates.contains(val)
    }

    /// Get the index of this cell within its grid.
    pub fn idx(&self) -> CellIdx {
        self.idx
    }
}
