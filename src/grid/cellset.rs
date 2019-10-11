// ! A structure which stores sets of cells within the grid as bitmasks.

use grid::CellIdx;
use grid::fixed_size::GridSize;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A set of cells from a Sudoku grid, represented internally as a bitmask.
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct CellSet<T: GridSize> {

    /// A bitmask representing the cells contained in this `CellSet`
    bits: Vec<u64>,

    /// Phantom data since the generic type is purely for compile-time safety
    size: PhantomData<T>,
}

/// A structure capable of iterating over the cells held in a `CellSet`.
pub struct CellSetIterator {

    /// A bitmask representing the cells yet to be iterated over
    bits: Vec<u64>,

    /// The index of the currently-active bitmask
    active_idx: usize,
}

impl Iterator for CellSetIterator {

    type Item = CellIdx;

    fn next(&mut self) -> Option<CellIdx> {

        if self.active_idx >= self.bits.len() {
            return None;
        }

        while self.bits[self.active_idx] == 0 && self.active_idx < self.bits.len() - 1 {
            self.active_idx += 1;
        }

        let mask = self.bits[self.active_idx];

        if mask != 0 {
            let next = mask.trailing_zeros() as CellIdx;
            self.bits[self.active_idx] = mask & (mask - 1);
            Some(next + 64 * self.active_idx)
        } else {
            None
        }
    }
}

impl<T: GridSize> CellSet<T> {

    /// Create an empty `CellSet`
    pub fn empty() -> CellSet<T> {
        CellSet {
            bits: vec![0; (T::size() * T::size() + 64 - 1) / 64],
            size: PhantomData,
        }
    }

    /// Create a full `CellSet`
    pub fn full() -> CellSet<T> {
        !CellSet::empty()
    }

    /// Create a new `CellSet` containing only the given single cell
    pub fn from_cell(cell: CellIdx) -> CellSet<T> {
        let mut cell_set = CellSet::empty();
        cell_set.bits[cell / 64] = 1 << (cell % 64);
        cell_set
    }

    /// Create a new `CellSet` containing the given cells
    pub fn from_cells<I: IntoIterator<Item = CellIdx>>(cells: I) -> CellSet<T> {
        let mut cell_set = CellSet::empty();
        for cell in cells {
            cell_set.bits[cell / 64] |= 1 << (cell % 64);
        }
        cell_set
    }

    /// Add the given cell to this `CellSet`
    pub fn add_cell(&mut self, cell: CellIdx) {
        self.bits[cell / 64] |= 1 << (cell % 64);
    }

    /// Remove the given cell from this `CellSet`
    pub fn remove_cell(&mut self, cell: CellIdx) {
        self.bits[cell / 64] &= !(1 << (cell % 64));
    }

    /// An iterator over the cells held in this `CellSet`
    pub fn iter(&self) -> CellSetIterator {
        CellSetIterator {
            bits: self.bits.clone(),
            active_idx: 0,
        }
    }

    /// The number of cells contained in this `CellSet`
    pub fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    /// Check if this `CellSet` is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first cell from this `CellSet`
    pub fn first(&self) -> Option<CellIdx> {
        self.iter().next()
    }

    /// Determine whether this `CellSet` contains a particular cell or not
    pub fn contains(&self, cell: CellIdx) -> bool {
        self.bits[cell / 64] & (1 << (cell % 64)) != 0
    }

    /// Determine whether this `CellSet` contains another `CellSet` as a subset
    pub fn contains_all(&self, other: &CellSet<T>) -> bool {
        self.bits.iter()
            .zip(other.bits.iter())
            .all(|(&own_bits, &other_bits)| other_bits & own_bits == other_bits)
    }

    /// Produce the intersection of the given `CellSet`s
    pub fn intersection(cell_sets: &[&CellSet<T>]) -> CellSet<T> {
        cell_sets.iter().fold(CellSet::full(), |acc, curr| acc & *curr)
    }

    /// Produce the union of the given `CellSet`s
    pub fn union(cell_sets: &[CellSet<T>]) -> CellSet<T> {
        cell_sets.iter().fold(CellSet::empty(), |acc, curr| acc | curr)
    }

    /// Filter this `CellSet` by a predicate
    pub fn filter<P: FnMut(&CellIdx) -> bool>(&self, predicate: P) -> CellSet<T> {
        CellSet::from_cells(self.iter().filter(predicate))
    }

    /// Map the indicates held in this `CellSet`
    pub fn map<B, F: FnMut(CellIdx) -> B>(&self, f: F) -> Vec<B> {
        self.iter().map(f).collect()
    }

}

macro_rules! binop_from_ref_ref {
    ($t: ident, $f: ident) => {

        impl<'a, T: GridSize> $t<&'a CellSet<T>> for CellSet<T> {

            type Output = CellSet<T>;

            fn $f(self, rhs: &'a CellSet<T>) -> CellSet<T> {
                $t::$f(&self, rhs)
            }
        }

        impl<'b, T: GridSize> $t<CellSet<T>> for &'b CellSet<T> {

            type Output = CellSet<T>;

            fn $f(self, rhs: CellSet<T>) -> CellSet<T> {
                $t::$f(self, &rhs)
            }
        }

        impl<T: GridSize> $t<CellSet<T>> for CellSet<T> {

            type Output = CellSet<T>;

            fn $f(self, rhs: CellSet<T>) -> CellSet<T> {
                $t::$f(&self, &rhs)
            }
        }
    }
}

// `BitAnd` implementation for `CellSet`
impl<'a, 'b, T: GridSize> BitAnd<&'a CellSet<T>> for &'b CellSet<T> {

    type Output = CellSet<T>;

    fn bitand(self, rhs: &'a CellSet<T>) -> CellSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits & other_bits)
            .collect();

        CellSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitAndAssign<&'a CellSet<T>> for CellSet<T> {
    fn bitand_assign(&mut self, other: &'a CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] &= mask;
        }
    }
}

impl<T: GridSize> BitAndAssign<CellSet<T>> for CellSet<T> {
    fn bitand_assign(&mut self, other: CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] &= mask;
        }
    }
}

binop_from_ref_ref!(BitAnd, bitand);

// `BitOr` implementation for `CellSet`
impl<'a, 'b, T: GridSize> BitOr<&'a CellSet<T>> for &'b CellSet<T> {

    type Output = CellSet<T>;

    fn bitor(self, rhs: &'a CellSet<T>) -> CellSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits | other_bits)
            .collect();

        CellSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitOrAssign<&'a CellSet<T>> for CellSet<T> {
    fn bitor_assign(&mut self, other: &'a CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] |= mask;
        }
    }
}

impl<T: GridSize> BitOrAssign<CellSet<T>> for CellSet<T> {
    fn bitor_assign(&mut self, other: CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] |= mask;
        }
    }
}

binop_from_ref_ref!(BitOr, bitor);

// `BitXor` implementation for `CellSet`
impl<'a, 'b, T: GridSize> BitXor<&'a CellSet<T>> for &'b CellSet<T> {

    type Output = CellSet<T>;

    fn bitxor(self, rhs: &'a CellSet<T>) -> CellSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits ^ other_bits)
            .collect();

        CellSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitXorAssign<&'a CellSet<T>> for CellSet<T> {
    fn bitxor_assign(&mut self, other: &'a CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] ^= mask;
        }
    }
}

impl<T: GridSize> BitXorAssign<CellSet<T>> for CellSet<T> {
    fn bitxor_assign(&mut self, other: CellSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] ^= mask;
        }
    }
}

binop_from_ref_ref!(BitXor, bitxor);

// `Not` implementation for `CellSet`
impl<'a, T: GridSize> Not for &'a CellSet<T> {

    type Output = CellSet<T>;

    fn not(self) -> CellSet<T> {

        let mut negated_bits: Vec<_> = self.bits.iter().map(|mask| !mask).collect();
        let number_of_cells = T::size() * T::size();
        let high_order_mask = (1 << (number_of_cells % 64)) - 1;
        negated_bits[number_of_cells / 64] &= high_order_mask;

        CellSet {
            bits: negated_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> Not for CellSet<T> {

    type Output = CellSet<T>;

    fn not(self) -> CellSet<T> {

        let mut negated_bits: Vec<_> = self.bits.iter().map(|mask| !mask).collect();
        let number_of_cells = T::size() * T::size();
        let high_order_mask = (1 << (number_of_cells % 64)) - 1;
        negated_bits[number_of_cells / 64] &= high_order_mask;

        CellSet {
            bits: negated_bits,
            size: PhantomData,
        }
    }
}
