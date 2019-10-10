//! A structure representing a single cell within a Sudoku grid

use grid::Candidate;
use grid::candidateset::CandidateSet;
use grid::fixed_size::GridSize;

/// A single cell within a Sudoku grid
#[derive(Copy, Clone)]
pub struct Cell<T: GridSize> {

    /// The value, if any, held by this `Cell`
    value: Option<Candidate>,

    /// The potential values that this `Cell` can hold
    candidates: CandidateSet<T>,
}

impl<T: GridSize> Cell<T> {

    /// Create a new `Cell` with no value, and with all candidates possible
    pub fn empty() -> Cell<T> {
        Cell { value: None, candidates: CandidateSet::full() }
    }

    // Get the value currently held in this `Cell`
    pub fn value(&self) -> Option<Candidate> {
        self.value
    }

    /// Set the value currently held in this `Cell`
    pub fn set_value(&mut self, val: Candidate) {
        self.value = Some(val);
        self.candidates = CandidateSet::empty();
    }

    /// Determine whether this `Cell` is empty or not.
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Determine the candidates which are allowed in this `Cell`
    pub fn candidates(&self) -> CandidateSet<T> {
        self.candidates
    }

    /// Remove a potential candidate from this `Cell`
    pub fn remove_candidate(&mut self, val: Candidate) {
        self.candidates.remove_candidate(val);
    }

    /// Check if a given candidate is allowed in this `Cell`
    pub fn has_candidate(&self, val: Candidate) -> bool {
        self.candidates.has_candidate(val)
    }

    /// Get the first candidate that can go in this `Cell`
    pub fn first_candidate(&self) -> Option<Candidate> {
        self.candidates.first()
    }

    /// Get the number of candidates that can go in this `Cell`
    pub fn num_candidates(&self) -> usize {
        self.candidates.len()
    }
}
