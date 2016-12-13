//! Some stuctures which represent regions (rows, columns, blocks) within a Sudoku grid.

use grid::{CellIdx, Grid, LARGE_SIZE, SMALL_SIZE};
use grid::cell::Cell;

/// A row of the grid.
pub struct Row<'a> {
    /// The grid to which this row belongs.
    grid: &'a Grid,
    /// The index of this row.
    row_idx: usize,
    /// The index of cells within the row, used for iterating.
    col_idx: usize,
}

impl<'a> Row<'a> {
    /// The row with the given index from the supplied grid.
    pub fn new(grid: &'a Grid, idx: usize) -> Row<'a> {
        Row {
            grid: grid,
            row_idx: idx,
            col_idx: 0,
        }
    }
}

impl<'a> Iterator for Row<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        let cell_idx = CellIdx::new(self.row_idx, self.col_idx);

        self.col_idx += 1;

        match self.col_idx {
            0...LARGE_SIZE => Some(self.grid.get_cell(cell_idx)),
            _ => None,
        }
    }
}

/// A row of the grid.
pub struct Column<'a> {
    /// The grid to which this row belongs.
    grid: &'a Grid,
    /// The index of this column.
    col_idx: usize,
    /// The index of cells within the column, used for iterating.
    row_idx: usize,
}

impl<'a> Column<'a> {
    /// The column with the given index from the supplied grid.
    pub fn new(grid: &'a Grid, idx: usize) -> Column<'a> {
        Column {
            grid: grid,
            col_idx: idx,
            row_idx: 0,
        }
    }
}

impl<'a> Iterator for Column<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        let cell_idx = CellIdx::new(self.row_idx, self.col_idx);

        self.row_idx += 1;

        match self.row_idx {
            0...LARGE_SIZE => Some(self.grid.get_cell(cell_idx)),
            _ => None,
        }
    }
}

/// A block within the grid.
pub struct Block<'a> {
    /// The grid to which this block belongs.
    grid: &'a Grid,
    /// The index of rows within the block, used for iterating.
    row_idx: usize,
    /// The index of columns within the block, used for iterating.
    col_idx: usize,
    /// The number of cells we have iterated over so far.
    count: usize,
}

impl<'a> Block<'a> {
    /// The block with the given indices from the supplied grid.
    pub fn new(grid: &'a Grid, row_idx: usize, col_idx: usize) -> Block<'a> {
        Block {
            grid: grid,
            row_idx: 3 * row_idx,
            col_idx: 3 * col_idx,
            count: 0,
        }
    }
}

impl<'a> Iterator for Block<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        let cell_idx = CellIdx::new(self.row_idx, self.col_idx);

        self.col_idx += 1;
        if self.col_idx % SMALL_SIZE == 0 {
            self.col_idx -= 3;
            self.row_idx += 1;
        }
        self.count += 1;

        match self.count {
            0...LARGE_SIZE => Some(self.grid.get_cell(cell_idx)),
            _ => None,
        }
    }
}

/// All cells within the grid.
pub struct Cells<'a> {
    /// The grid to which the cells belong.
    grid: &'a Grid,
    /// The index of rows within the grid, used for iterating.
    row_idx: usize,
    /// The index of columns within the grid, used for iterating.
    col_idx: usize,
}

impl<'a> Cells<'a> {
    /// The cells from the supplied grid.
    pub fn new(grid: &'a Grid) -> Cells<'a> {
        Cells {
            grid: grid,
            row_idx: 0,
            col_idx: 0,
        }
    }
}

impl<'a> Iterator for Cells<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        let cell_idx = CellIdx::new(self.row_idx, self.col_idx);

        self.col_idx += 1;
        if self.col_idx == LARGE_SIZE {
            self.col_idx = 0;
            self.row_idx += 1;
        }

        if cell_idx.row < LARGE_SIZE {
            Some(self.grid.get_cell(cell_idx))
        } else {
            None
        }
    }
}