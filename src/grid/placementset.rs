//! A structure which stores sets of cell/value pairs as bitmasks.

use grid::{CellIdx, Candidate};
use grid::fixed_size::GridSize;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A simple structure representing a placement
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Placement {
    pub cell: CellIdx,
    pub candidate: Candidate,
}

/// A set of cell/value pairs from a Sudoku grid, represented internally as a bitmask.
#[derive(Eq, PartialEq, Clone)]
pub struct PlacementSet<T: GridSize> {

    /// A bitmask representing the placements contained in this `PlacementSet`
    bits: Vec<u64>,

    /// Phantom data since the generic type is purely for compile-time safety
    size: PhantomData<T>,
}

/// A structure capable of iterating over the placements held in a `PlacementSet`.
pub struct PlacementSetIterator {

    /// A bitmask representing the placements yet to be iterated over
    bits: Vec<u64>,

    /// The index of the currently-active bitmask
    active_idx: usize,
}

impl Iterator for PlacementSetIterator {

    type Item = Placement;

    fn next(&mut self) -> Option<Placement> {

        if self.active_idx >= self.bits.len() {
            return None;
        }

        while self.bits[self.active_idx] == 0 && self.active_idx < self.bits.len() - 1 {
            self.active_idx += 1;
        }

        let mask = self.bits[self.active_idx];
        if mask != 0 {
            let next = mask.trailing_zeros() as usize + 64 * self.active_idx as CellIdx;
            self.bits[self.active_idx] = mask & (mask - 1);
            Some(Placement { cell: next / 9, candidate: next % 9 + 1 })
        } else {
            None
        }
    }
}

impl <T: GridSize> PlacementSet<T> {

    /// Create an empty `PlacementSet`
    pub fn empty() -> PlacementSet<T> {
        PlacementSet {
            bits: vec![0; (T::size() * T::size() * T::size() + 64 - 1) / 64],
            size: PhantomData,
        }
    }

    /// Create a full `PlacementSet`
    pub fn full() -> PlacementSet<T> {
        !PlacementSet::empty()
    }

    /// Create a new `PlacementSet` containing only the given single placement
    pub fn from_placement(placement: Placement) -> PlacementSet<T> {
        let mut placement_set = PlacementSet::empty();
        let index = 9 * placement.cell + placement.candidate - 1;
        placement_set.bits[index / 64] = 1 << (index % 64);
        placement_set
    }

    /// Create a new `PlacementSet` containing the given placements
    pub fn from_placements<I: IntoIterator<Item = Placement>>(placements: I) -> PlacementSet<T> {
        let mut placement_set = PlacementSet::empty();
        for placement in placements {
            let index = 9 * placement.cell + placement.candidate - 1;
            placement_set.bits[index / 64] |= 1 << (index % 64);
        }
        placement_set
    }

    /// Add the given placement to this `PlacementSet`
    pub fn add_placement(&mut self, placement: Placement) {
        let index = 9 * placement.cell + placement.candidate - 1;
        self.bits[index / 64] |= 1 << (index % 64);
    }

    /// Remove the given placement from this `PlacementSet`
    pub fn remove_placement(&mut self, placement: Placement) {
        let index = 9 * placement.cell + placement.candidate - 1;
        self.bits[index / 64] &= !(1 << (index % 64));
    }

    /// An iterator over the placements held in this `PlacementSet`
    pub fn iter(&self) -> PlacementSetIterator {
        PlacementSetIterator {
            bits: self.bits.clone(),
            active_idx: 0,
        }
    }

    /// The number of placements contained in this `PlacementSet`
    pub fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    /// Check if this `PlacementSet` is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first placement from this `PlacementSet`
    pub fn first(&self) -> Option<Placement> {
        self.iter().next()
    }

    /// Determine whether this `PlacementSet` contains a particular placement or not
    pub fn contains(&self, placement: Placement) -> bool {
        let index = 9 * placement.cell + placement.candidate - 1;
        self.bits[index / 64] & (1 << (index % 64)) != 0
    }

    /// Determine whether this `PlacementSet` contains another `PlacementSet` as a subset
    pub fn contains_all(&self, other: &PlacementSet<T>) -> bool {
        self.bits.iter()
            .zip(other.bits.iter())
            .all(|(&own_bits, &other_bits)| other_bits & own_bits == other_bits)
    }

    /// Produce the intersection of the given `PlacementSet`s
    pub fn intersection(placement_sets: &[&PlacementSet<T>]) -> PlacementSet<T> {
        placement_sets.iter().fold(PlacementSet::full(), |acc, curr| acc & *curr)
    }

    /// Produce the union of the given `PlacementSet`s
    pub fn union(placement_sets: &[PlacementSet<T>]) -> PlacementSet<T> {
        placement_sets.iter().fold(PlacementSet::empty(), |acc, curr| acc | curr)
    }

    /// Filter this `PlacementSet` by a predicate
    pub fn filter<P: FnMut(&Placement) -> bool>(&self, predicate: P) -> PlacementSet<T> {
        PlacementSet::from_placements(self.iter().filter(predicate))
    }

    /// Map the indices held in this `PlacementSet`
    pub fn map<B, F: FnMut(Placement) -> B>(&self, f: F) -> Vec<B> {
        self.iter().map(f).collect()
    }
}

macro_rules! binop_from_ref_ref {
    ($t: ident, $f: ident) => {

        impl<'a, T: GridSize> $t<&'a PlacementSet<T>> for PlacementSet<T> {

            type Output = PlacementSet<T>;

            fn $f(self, rhs: &'a PlacementSet<T>) -> PlacementSet<T> {
                $t::$f(&self, rhs)
            }
        }

        impl<'b, T: GridSize> $t<PlacementSet<T>> for &'b PlacementSet<T> {

            type Output = PlacementSet<T>;

            fn $f(self, rhs: PlacementSet<T>) -> PlacementSet<T> {
                $t::$f(self, &rhs)
            }
        }

        impl<T: GridSize> $t<PlacementSet<T>> for PlacementSet<T> {

            type Output = PlacementSet<T>;

            fn $f(self, rhs: PlacementSet<T>) -> PlacementSet<T> {
                $t::$f(&self, &rhs)
            }
        }
    }
}

// `BitAnd` implementation for `PlacementSet`
impl<'a, 'b, T: GridSize> BitAnd<&'a PlacementSet<T>> for &'b PlacementSet<T> {

    type Output = PlacementSet<T>;

    fn bitand(self, rhs: &'a PlacementSet<T>) -> PlacementSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits & other_bits)
            .collect();

        PlacementSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitAndAssign<&'a PlacementSet<T>> for PlacementSet<T> {
    fn bitand_assign(&mut self, other: &'a PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] &= mask;
        }
    }
}

impl<T: GridSize> BitAndAssign<PlacementSet<T>> for PlacementSet<T> {
    fn bitand_assign(&mut self, other: PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] &= mask;
        }
    }
}

binop_from_ref_ref!(BitAnd, bitand);

// `BitOr` implementation for `PlacementSet`
impl<'a, 'b, T: GridSize> BitOr<&'a PlacementSet<T>> for &'b PlacementSet<T> {

    type Output = PlacementSet<T>;

    fn bitor(self, rhs: &'a PlacementSet<T>) -> PlacementSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits | other_bits)
            .collect();

        PlacementSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitOrAssign<&'a PlacementSet<T>> for PlacementSet<T> {
    fn bitor_assign(&mut self, other: &'a PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] |= mask;
        }
    }
}

impl<T: GridSize> BitOrAssign<PlacementSet<T>> for PlacementSet<T> {
    fn bitor_assign(&mut self, other: PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] |= mask;
        }
    }
}

binop_from_ref_ref!(BitOr, bitor);

// `BitXor` implementation for `PlacementSet`
impl<'a, 'b, T: GridSize> BitXor<&'a PlacementSet<T>> for &'b PlacementSet<T> {

    type Output = PlacementSet<T>;

    fn bitxor(self, rhs: &'a PlacementSet<T>) -> PlacementSet<T> {

        let new_bits = self.bits.iter()
            .zip(rhs.bits.iter())
            .map(|(own_bits, other_bits)| own_bits ^ other_bits)
            .collect();

        PlacementSet {
            bits: new_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> BitXorAssign<&'a PlacementSet<T>> for PlacementSet<T> {
    fn bitxor_assign(&mut self, other: &'a PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] ^= mask;
        }
    }
}

impl<T: GridSize> BitXorAssign<PlacementSet<T>> for PlacementSet<T> {
    fn bitxor_assign(&mut self, other: PlacementSet<T>) {
        for (idx, mask) in other.bits.iter().enumerate() {
            self.bits[idx] ^= mask;
        }
    }
}

binop_from_ref_ref!(BitXor, bitxor);

// `Not` implementation for `PlacementSet`
impl<'a, T: GridSize> Not for &'a PlacementSet<T> {

    type Output = PlacementSet<T>;

    fn not(self) -> PlacementSet<T> {

        let mut negated_bits: Vec<_> = self.bits.iter().map(|mask| !mask).collect();
        let number_of_cells = T::size() * T::size();
        let high_order_mask = (1 << (number_of_cells % 64)) - 1;
        negated_bits[number_of_cells / 64] &= high_order_mask;

        PlacementSet {
            bits: negated_bits,
            size: PhantomData,
        }
    }
}

impl<'a, T: GridSize> Not for PlacementSet<T> {

    type Output = PlacementSet<T>;

    fn not(self) -> PlacementSet<T> {

        let mut negated_bits: Vec<_> = self.bits.iter().map(|mask| !mask).collect();
        let number_of_cells = T::size() * T::size() * T::size();
        let high_order_mask = (1 << (number_of_cells % 64)) - 1;
        negated_bits[number_of_cells / 64] &= high_order_mask;

        PlacementSet {
            bits: negated_bits,
            size: PhantomData,
        }
    }
}
