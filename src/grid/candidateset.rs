//! A structure which stores a set of candidates as a bitmask

use grid::Candidate;
use grid::fixed_size::GridSize;

use std::fmt;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A set of possible candidates for a Sudoku.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CandidateSet<T: GridSize> {
    mask: usize,
    size: PhantomData<T>,
}

/// A structure capable of iterating over the candidates held in a `CandidateSet`
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

impl <T: GridSize> fmt::Display for CandidateSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

impl<T: GridSize> CandidateSet<T> {

    /// Create an empty `CandidateSet`
    pub fn empty() -> CandidateSet<T> {
        CandidateSet { mask: 0, size: PhantomData }
    }

    /// Create a `CandidateSet` holding all possible candidates
    pub fn full() -> CandidateSet<T> {
        !CandidateSet::empty()
    }

    /// Create a `CandidateSet` containing the given candidates
    pub fn from_candidates<I: IntoIterator<Item = Candidate>>(candidates: I) -> CandidateSet<T> {
        let mask = candidates.into_iter().fold(0x0, |acc, x| acc | (1 << x));
        CandidateSet { mask, size: PhantomData }
    }

    /// Add a candidate to this `CandidateSet`
    pub fn add_candidate(&mut self, val: Candidate) {
        self.mask |= 1 << val;
    }

    /// Remove a candidate from this `CandidateSet`
    pub fn remove_candidate(&mut self, val: Candidate) {
        self.mask &= !(1 << val);
    }

    /// Check if this `CandidateSet` holds a particular value
    pub fn has_candidate(&self, val: Candidate) -> bool {
        self.mask & (1 << val) != 0
    }

    /// An iterator over the candidates held in this `CandidateSet`
    pub fn iter(&self) -> CandidateSetIterator {
        CandidateSetIterator { mask: self.mask }
    }

    /// The number of candidates contained in this `CandidateSet`
    pub fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }

    /// Check if this `CandidateSet` is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first candidate from this `CandidateSet`
    pub fn first(&self) -> Option<Candidate> {
        self.iter().next()
    }

    /// Filter this `CandidateSet` by a predicate
    pub fn filter<P: FnMut(&Candidate) -> bool>(&self, predicate: P) -> CandidateSet<T> {
        CandidateSet::from_candidates(self.iter().filter(predicate))
    }

    /// Map the indices held in this `CandidateSet`
    pub fn map<B, F: FnMut(Candidate) -> B>(&self, f: F) -> Vec<B> {
        self.iter().map(f).collect()
    }

}

macro_rules! binop_from_ref_ref {
    ($t: ident, $f: ident) => {

        impl<'a, T: GridSize> $t<&'a CandidateSet<T>> for CandidateSet<T> {

            type Output = CandidateSet<T>;

            fn $f(self, rhs: &'a CandidateSet<T>) -> CandidateSet<T> {
                $t::$f(&self, rhs)
            }
        }

        impl<'b, T: GridSize> $t<CandidateSet<T>> for &'b CandidateSet<T> {

            type Output = CandidateSet<T>;

            fn $f(self, rhs: CandidateSet<T>) -> CandidateSet<T> {
                $t::$f(self, &rhs)
            }
        }

        impl<T: GridSize> $t<CandidateSet<T>> for CandidateSet<T> {

            type Output = CandidateSet<T>;

            fn $f(self, rhs: CandidateSet<T>) -> CandidateSet<T> {
                $t::$f(&self, &rhs)
            }
        }
    }
}

// `BitAnd` implementation for `CandidateSet`
impl<'a, 'b, T: GridSize> BitAnd<&'a CandidateSet<T>> for &'b CandidateSet<T> {

    type Output = CandidateSet<T>;

    fn bitand(self, rhs: &'a CandidateSet<T>) -> CandidateSet<T> {
        CandidateSet { mask: self.mask & rhs.mask, size: PhantomData }
    }
}

impl<'a, T: GridSize> BitAndAssign<&'a CandidateSet<T>> for CandidateSet<T> {
    fn bitand_assign(&mut self, other: &'a CandidateSet<T>) {
        self.mask &= other.mask;
    }
}

impl<T: GridSize> BitAndAssign<CandidateSet<T>> for CandidateSet<T> {
    fn bitand_assign(&mut self, other: CandidateSet<T>) {
        self.mask &= other.mask;
    }
}

binop_from_ref_ref!(BitAnd, bitand);

// `BitOr` implementation for `CandidateSet`
impl<'a, 'b, T: GridSize> BitOr<&'a CandidateSet<T>> for &'b CandidateSet<T> {

    type Output = CandidateSet<T>;

    fn bitor(self, rhs: &'a CandidateSet<T>) -> CandidateSet<T> {
        CandidateSet { mask: self.mask | rhs.mask, size: PhantomData }
    }
}

impl<'a, T: GridSize> BitOrAssign<&'a CandidateSet<T>> for CandidateSet<T> {
    fn bitor_assign(&mut self, other: &'a CandidateSet<T>) {
        self.mask |= other.mask;
    }
}

impl<T: GridSize> BitOrAssign<CandidateSet<T>> for CandidateSet<T> {
    fn bitor_assign(&mut self, other: CandidateSet<T>) {
        self.mask |= other.mask;
    }
}

binop_from_ref_ref!(BitOr, bitor);

// `BitXor` implementation for `CandidateSet`
impl<'a, 'b, T: GridSize> BitXor<&'a CandidateSet<T>> for &'b CandidateSet<T> {

    type Output = CandidateSet<T>;

    fn bitxor(self, rhs: &'a CandidateSet<T>) -> CandidateSet<T> {
        CandidateSet { mask: self.mask ^ rhs.mask, size: PhantomData }
    }
}

impl<'a, T: GridSize> BitXorAssign<&'a CandidateSet<T>> for CandidateSet<T> {
    fn bitxor_assign(&mut self, other: &'a CandidateSet<T>) {
        self.mask ^= other.mask;
    }
}

impl<T: GridSize> BitXorAssign<CandidateSet<T>> for CandidateSet<T> {
    fn bitxor_assign(&mut self, other: CandidateSet<T>) {
        self.mask ^= other.mask;
    }
}

binop_from_ref_ref!(BitXor, bitxor);

// `Not` implementation for `CandidateSet`
impl<'a, T: GridSize> Not for &'a CandidateSet<T> {

    type Output = CandidateSet<T>;

    fn not(self) -> CandidateSet<T> {
        CandidateSet { 
            mask: !self.mask & ((1 << (T::size() + 1)) - 2),
            size: PhantomData,
        }
    }
}

impl<T: GridSize> Not for CandidateSet<T> {

    type Output = CandidateSet<T>;

    fn not(self) -> CandidateSet<T> {
        CandidateSet { 
            mask: !self.mask & ((1 << (T::size() + 1)) - 2),
            size: PhantomData,
        }
    }
}
