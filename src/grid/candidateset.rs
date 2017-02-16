//! A structure which stores a set of candidates as a bitmask.

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use grid::Candidate;

/// A set of possible candidates for a Sudoku.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CandidateSet {
    mask: usize,
}

/// A structure capable of iterating over the candidates stored in a `CandidateSet`
pub struct CandidateSetIterator {
    mask: usize,
}

impl Iterator for CandidateSetIterator {
    type Item = Candidate;

    fn next(&mut self) -> Option<Candidate> {
        if self.mask != 0 {
            let next = self.mask.trailing_zeros() as Candidate;
            self.mask &= self.mask - 1;
            Some(next)
        } else {
            None
        }
    }
}

impl CandidateSet {
    /// Create a new `CandidateSet` with the given bitmask.
    pub fn new(mask: Candidate) -> CandidateSet {
        CandidateSet { mask: mask }
    }

    /// Create an empty `CandidateSet`.
    pub fn empty() -> CandidateSet {
        CandidateSet::new(0x0)
    }

    /// Create a `CandidateSet` holding all possible candidates.
    pub fn full() -> CandidateSet {
        !CandidateSet::empty()
    }

    /// Create a `CandidateSet` containing the candidates from the given iterator.
    pub fn from_candidates<I>(candidates: I) -> CandidateSet
        where I: IntoIterator<Item = Candidate>
    {
        let mask = candidates.into_iter().fold(0x0, |acc, x| acc | (1 << x));
        CandidateSet::new(mask)
    }

    /// Remove a candidate from this `CandidateSet`.
    pub fn remove_candidate(&mut self, val: Candidate) {
        self.mask &= !(1 << val);
    }

    /// Check if this `CandidateSet` holds a particular value.
    pub fn has_candidate(&self, val: Candidate) -> bool {
        self.mask & (1 << val) != 0
    }

    /// An iterator over the candidates held in this `CandidateSet`
    pub fn iter(&self) -> CandidateSetIterator {
        CandidateSetIterator { mask: self.mask }
    }

    /// The number of cells contained in this `CandidateSet`.
    pub fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }

    /// Check if this `CandidateSet` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first cell from this `CandidateSet`.
    pub fn first(&self) -> Option<Candidate> {
        self.iter().next()
    }

    /// Filter this `CandidateSet` by a predicate.
    pub fn filter<P>(&self, predicate: P) -> CandidateSet
        where P: FnMut(&Candidate) -> bool
    {
        CandidateSet::from_candidates(self.iter().filter(predicate))
    }

    /// Map the indices held in this `CandidateSet`.
    pub fn map<B, F>(&self, f: F) -> Vec<B>
        where F: FnMut(Candidate) -> B
    {
        self.iter().map(f).collect()
    }
}

macro_rules! binop_from_ref_ref {
    ($t: ident, $f: ident) => {
        impl<'a> $t<&'a CandidateSet> for CandidateSet {
            type Output = CandidateSet;

            fn $f(self, rhs: &'a CandidateSet) -> CandidateSet {
                $t::$f(&self, rhs)
            }
        }

        impl<'b> $t<CandidateSet> for &'b CandidateSet {
            type Output = CandidateSet;

            fn $f(self, rhs: CandidateSet) -> CandidateSet {
                $t::$f(self, &rhs)
            }
        }

        impl $t<CandidateSet> for CandidateSet {
            type Output = CandidateSet;

            fn $f(self, rhs: CandidateSet) -> CandidateSet {
                $t::$f(&self, &rhs)
            }
        }
    }
}

// `BitAnd` implementation for `CandidateSet`.
impl<'a, 'b> BitAnd<&'a CandidateSet> for &'b CandidateSet {
    type Output = CandidateSet;

    fn bitand(self, rhs: &'a CandidateSet) -> CandidateSet {
        CandidateSet { mask: self.mask & rhs.mask }
    }
}

impl<'a> BitAndAssign<&'a CandidateSet> for CandidateSet {
    fn bitand_assign(&mut self, other: &'a CandidateSet) {
        self.mask &= other.mask;
    }
}

impl BitAndAssign<CandidateSet> for CandidateSet {
    fn bitand_assign(&mut self, other: CandidateSet) {
        self.mask &= other.mask;
    }
}

binop_from_ref_ref!(BitAnd, bitand);

// `BitOr` implementation for `CandidateSet`.
impl<'a, 'b> BitOr<&'a CandidateSet> for &'b CandidateSet {
    type Output = CandidateSet;

    fn bitor(self, rhs: &'a CandidateSet) -> CandidateSet {
        CandidateSet { mask: self.mask | rhs.mask }
    }
}

impl<'a> BitOrAssign<&'a CandidateSet> for CandidateSet {
    fn bitor_assign(&mut self, other: &'a CandidateSet) {
        self.mask |= other.mask;
    }
}

impl BitOrAssign<CandidateSet> for CandidateSet {
    fn bitor_assign(&mut self, other: CandidateSet) {
        self.mask |= other.mask;
    }
}

binop_from_ref_ref!(BitOr, bitor);

// `BitXor` implementation for `CandidateSet`.
impl<'a, 'b> BitXor<&'a CandidateSet> for &'b CandidateSet {
    type Output = CandidateSet;

    fn bitxor(self, rhs: &'a CandidateSet) -> CandidateSet {
        CandidateSet { mask: self.mask ^ rhs.mask }
    }
}
impl<'a> BitXorAssign<&'a CandidateSet> for CandidateSet {
    fn bitxor_assign(&mut self, other: &'a CandidateSet) {
        self.mask ^= other.mask;
    }
}

impl BitXorAssign<CandidateSet> for CandidateSet {
    fn bitxor_assign(&mut self, other: CandidateSet) {
        self.mask ^= other.mask;
    }
}

binop_from_ref_ref!(BitXor, bitxor);

// `Not` implementation for `CandidateSet`.
impl<'a> Not for &'a CandidateSet {
    type Output = CandidateSet;

    fn not(self) -> CandidateSet {
        CandidateSet { mask: !self.mask & 0x3FE }
    }
}

impl Not for CandidateSet {
    type Output = CandidateSet;

    fn not(self) -> CandidateSet {
        CandidateSet {
            mask: !self.mask & 0x3FE }
    }
}