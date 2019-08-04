//! A structure which stores sets of cells within the grid as bitmasks.

use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use grid::CellIdx;
use grid::Grid;
use grid::Region;
use grid::Region::*;

/// A set of cells from a Sudoku grid, represented internally as a bitmask.
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct CellSet {
    /// The high order bits of the bitmask.
    pub hi: u64,
    /// The low order bits of the bitmask.
    pub lo: u64,
}

/// A structure capable of iterating over the cells held in a `CellSet`.
pub struct CellSetIterator {
    /// The remaining cells, high order bits.
    hi: u64,
    /// The remaining cells, low order bits.
    lo: u64,
}

impl fmt::Display for CellSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.iter().map(|x| Grid::cell_name(x)).collect::<Vec<_>>().join(", "))
    }
}

impl fmt::Debug for CellSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.iter().map(|x| Grid::cell_name(x)).collect::<Vec<_>>().join(", "))
    }
}

impl Iterator for CellSetIterator {
    type Item = CellIdx;

    fn next(&mut self) -> Option<CellIdx> {
        if self.lo != 0 {
            let next = self.lo.trailing_zeros() as CellIdx;
            self.lo &= self.lo - 1;
            Some(next)
        } else if self.hi != 0 {
            let next = self.hi.trailing_zeros() as CellIdx + 64;
            self.hi &= self.hi - 1;
            Some(next)
        } else {
            None
        }
    }
}

impl CellSet {
    /// Create a new `CellSet` with the high and low order bitmasks set to the given values.
    pub fn new(hi: u64, lo: u64) -> CellSet {
        CellSet {
            hi: hi,
            lo: lo,
        }
    }

    /// Create an empty `CellSet`.
    pub fn empty() -> CellSet {
        CellSet::new(0x0, 0x0)
    }

    /// Create a `CellSet` holding all cells.
    pub fn full() -> CellSet {
        !CellSet::empty()
    }

    /// Create a `CellSet` containing the cells contained in the given iterator.
    pub fn from_cells<I: IntoIterator<Item = CellIdx>>(cells: I) -> CellSet {
        let mut lo = 0x0;
        let mut hi = 0x0;
        for cell in cells {
            match cell {
                0..=63 => lo |= 1 << cell,
                64..=81 => hi |= 1 << (cell - 64),
                _ => unreachable!(),
            }
        }

        CellSet {
            hi: hi,
            lo: lo,
        }
    }

    /// An iterator over the cells held in this `CellSet`.
    pub fn iter(&self) -> CellSetIterator {
        CellSetIterator {
            hi: self.hi,
            lo: self.lo,
        }
    }

    /// The number of cells contained in this `CellSet`.
    pub fn len(&self) -> usize {
        self.hi.count_ones() as usize + self.lo.count_ones() as usize
    }

    /// Check if this `CellSet` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first cell from this `CellSet`.
    pub fn first(&self) -> Option<CellIdx> {
        self.iter().next()
    }

    /// Determine whether this `CellSet` contains a particular cell or not.
    pub fn contains(&self, cell: CellIdx) -> bool {
        match cell {
            0..=63 => self.lo & (1 << cell) != 0,
            64..=81 => self.hi & (1 << (cell - 64)) != 0,
            _ => unreachable!(),
        }
    }

    /// Determine whether this `CellSet` contains another `CellSet` as a subset.
    pub fn contains_all(&self, other: CellSet) -> bool {
        ((self.lo & other.lo) == other.lo) && ((self.hi & other.hi) == other.hi)
    }

    /// Produce the intersection of the given `CellSet`s
    pub fn intersection(cell_sets: &[CellSet]) -> CellSet {
        cell_sets.iter().fold(CellSet::full(), |acc, curr| acc & curr)
    }

    /// Produce the union of the given `CellSet`s
    pub fn union(cell_sets: &[CellSet]) -> CellSet {
        cell_sets.iter().fold(CellSet::empty(), |acc, curr| acc | curr)
    }

    /// Get a `CellSet` representing all common neighbours of the given cells.
    pub fn common_neighbours(&self) -> CellSet {
        self.iter().fold(CellSet::full(), |acc, cell| acc & Grid::neighbours(cell))
    }

    /// Group the cells in this `CellSet` by rows / columns / blocks.
    pub fn group_by(&self, variety: Region) -> Vec<CellSet> {
        let regions = match variety {
            Row => Grid::rows(),
            Column => Grid::columns(),
            Block => Grid::blocks(),
        };

        regions.iter()
            .map(|cells| cells & self)
            .filter(|cells| !cells.is_empty())
            .collect()
    }

    /// Filter this `CellSet` by a predicate.
    pub fn filter<P>(&self, predicate: P) -> CellSet
        where P: FnMut(&CellIdx) -> bool
    {
        CellSet::from_cells(self.iter().filter(predicate))
    }

    /// Map the indices held in this `CellSet`.
    pub fn map<B, F>(&self, f: F) -> Vec<B>
        where F: FnMut(CellIdx) -> B
    {
        self.iter().map(f).collect()
    }
}

macro_rules! binop_from_ref_ref {
    ($t: ident, $f: ident) => {
        impl<'a> $t<&'a CellSet> for CellSet {
            type Output = CellSet;

            fn $f(self, rhs: &'a CellSet) -> CellSet {
                $t::$f(&self, rhs)
            }
        }

        impl<'b> $t<CellSet> for &'b CellSet {
            type Output = CellSet;

            fn $f(self, rhs: CellSet) -> CellSet {
                $t::$f(self, &rhs)
            }
        }

        impl $t<CellSet> for CellSet {
            type Output = CellSet;

            fn $f(self, rhs: CellSet) -> CellSet {
                $t::$f(&self, &rhs)
            }
        }
    }
}

// `BitAnd` implementation for `CellSet`.
impl<'a, 'b> BitAnd<&'a CellSet> for &'b CellSet {
    type Output = CellSet;

    fn bitand(self, rhs: &'a CellSet) -> CellSet {
        CellSet {
            hi: self.hi & rhs.hi,
            lo: self.lo & rhs.lo,
        }
    }
}

impl<'a> BitAndAssign<&'a CellSet> for CellSet {
    fn bitand_assign(&mut self, other: &'a CellSet) {
        self.hi &= other.hi;
        self.lo &= other.lo;
    }
}

impl BitAndAssign<CellSet> for CellSet {
    fn bitand_assign(&mut self, other: CellSet) {
        self.hi &= other.hi;
        self.lo &= other.lo;
    }
}

binop_from_ref_ref!(BitAnd, bitand);

// `BitOr` implementation for `CellSet`.
impl<'a, 'b> BitOr<&'a CellSet> for &'b CellSet {
    type Output = CellSet;

    fn bitor(self, rhs: &'a CellSet) -> CellSet {
        CellSet {
            hi: self.hi | rhs.hi,
            lo: self.lo | rhs.lo,
        }
    }
}

impl<'a> BitOrAssign<&'a CellSet> for CellSet {
    fn bitor_assign(&mut self, other: &'a CellSet) {
        self.hi |= other.hi;
        self.lo |= other.lo;
    }
}

impl BitOrAssign<CellSet> for CellSet {
    fn bitor_assign(&mut self, other: CellSet) {
        self.hi |= other.hi;
        self.lo |= other.lo;
    }
}

binop_from_ref_ref!(BitOr, bitor);

// `BitXor` implementation for `CellSet`.
impl<'a, 'b> BitXor<&'a CellSet> for &'b CellSet {
    type Output = CellSet;

    fn bitxor(self, rhs: &'a CellSet) -> CellSet {
        CellSet {
            hi: self.hi ^ rhs.hi,
            lo: self.lo ^ rhs.lo,
        }
    }
}
impl<'a> BitXorAssign<&'a CellSet> for CellSet {
    fn bitxor_assign(&mut self, other: &'a CellSet) {
        self.hi ^= other.hi;
        self.lo ^= other.lo;
    }
}

impl BitXorAssign<CellSet> for CellSet {
    fn bitxor_assign(&mut self, other: CellSet) {
        self.hi ^= other.hi;
        self.lo ^= other.lo;
    }
}

binop_from_ref_ref!(BitXor, bitxor);

// `Not` implementation for `CellSet`.
impl<'a> Not for &'a CellSet {
    type Output = CellSet;

    fn not(self) -> CellSet {
        CellSet {
            hi: !self.hi & 0x1FFFF,
            lo: !self.lo,
        }
    }
}

impl Not for CellSet {
    type Output = CellSet;

    fn not(self) -> CellSet {
        CellSet {
            hi: !self.hi & 0x1FFFF,
            lo: !self.lo,
        }
    }
}