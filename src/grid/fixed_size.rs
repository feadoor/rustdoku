//! A helper struct which allows the emulation of const-generics for grids of a fixed size.

use std::hash::Hash;

/// Until const-generics are available, use types implementing this trait as a stand-in for an
/// associated const on types dependent on the size of the grid.
pub trait GridSize: Clone + Copy + PartialEq + Eq + Hash {
    fn size() -> usize;
}

/// A utility macro used to easily define a struct which implements `GridSize`
#[macro_export]
macro_rules! define_grid_size {
    ($s:ident, $size:expr) => {
        
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        struct $s;

        impl GridSize for $s {
            fn size() -> usize {
                $size
            }
        }
    }
}
