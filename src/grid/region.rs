//! A type representing a region (row, column, block) of a Sudoku grid.

use grid::cell::Cell;

/// A region of a Sudoku grid - row, column or block.
pub struct Region<'a> {
    cells: Vec<&'a Cell>,
}

impl<'a> Region<'a> {
    /// Create a new region from the given cells.
    pub fn new(cells: Vec<&Cell>) -> Region {
        Region { cells: cells }
    }

    /// A slice view of the cells contained in this region.
    pub fn cells(&self) -> &[&Cell] {
        &self.cells
    }

    /// The empty cells from this region.
    pub fn empty_cells(&self) -> Vec<&Cell> {
        self.cells.iter().filter(|x| x.is_empty()).map(|x| *x).collect()
    }

    /// The cells from this region which can hold a particular value.
    pub fn potential_cells(&self, val: usize) -> Vec<&Cell> {
        self.cells.iter().filter(|x| x.has_candidate(val)).map(|x| *x).collect()
    }
}