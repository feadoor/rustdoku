//! A structure which stores sets of cells within the grid as bitmasks.

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use grid::CellIdx;

/// A set of cells from a Sudoku grid, represented internally as a bitmask.
#[derive(Eq, PartialEq, Debug)]
pub struct CellSet {
    /// The high order bits of the bitmask.
    pub hi: u64,
    /// The low order bits of the bitmask.
    pub lo: u64
}

/// A structure capable of iterating over the cells held in a `CellSet`.
pub struct CellSetIterator {
    /// The remaining cells, high order bits.
    hi: u64,
    /// The remaining cells, low order bits.
    lo: u64,
}

impl Iterator for CellSetIterator {
    type Item = CellIdx;

    fn next(&mut self) -> Option<CellIdx> {
        if self.lo != 0 {
            let next = self.lo.trailing_zeros() as usize;
            self.lo &= self.lo - 1;
            Some(next)
        } else if self.hi != 0 {
            let next = self.hi.trailing_zeros() as usize + 64;
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

    /// Create a `CellSet` containing the cells contained in the given iterator.
    pub fn from_cells<I>(cells: I) -> CellSet
        where I: IntoIterator<Item = CellIdx>
    {
        let mut lo = 0x0;
        let mut hi = 0x0;
        for cell in cells {
            match cell {
                0...63 => lo |= 1 << cell,
                64...81 => hi |= 1 << (cell - 64),
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