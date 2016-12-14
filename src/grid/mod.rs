//! A structure representing the state of a Sudoku grid.

pub mod cell;
pub mod region;

use std::fmt;

use self::cell::Cell;
use self::region::Region;

/// The size of the internal regions in a Sudoku grid.
pub const SMALL_SIZE: usize = 3;
/// The overall size of a Sudoku grid.
pub const LARGE_SIZE: usize = SMALL_SIZE * SMALL_SIZE;

/// A type that lets us refer to coordinates in the grid.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct CellIdx {
    row: usize,
    col: usize,
}

impl CellIdx {
    /// Create a `CellIdx` from the given row and column indices.
    pub fn new(row: usize, col: usize) -> CellIdx {
        CellIdx {
            row: row,
            col: col,
        }
    }
}

/// A Sudoku grid.
pub struct Grid {
    cells: [[Cell; LARGE_SIZE]; LARGE_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Create a String which will separate groups of rows in the grid.
        let dashes = String::from_utf8(vec![b'-'; 2 * SMALL_SIZE - 1]).unwrap();
        let row_sep = (0..SMALL_SIZE).map(|_| "+".to_string() + &dashes).collect::<String>() + "+";

        try!(write!(f, "{}", row_sep));

        // Iterate over every block in the grid.
        for block_idx_r in 0..SMALL_SIZE {
            for row_idx in block_idx_r * SMALL_SIZE..(block_idx_r + 1) * SMALL_SIZE {

                // Start the line off with some border.
                try!(write!(f, "\n|"));
                for block_idx_c in 0..SMALL_SIZE {
                    for col_idx in block_idx_c * SMALL_SIZE..(block_idx_c + 1) * SMALL_SIZE {

                        // Write either the number in the cell, or a dot if there isn't one.
                        let val = self.cells[row_idx][col_idx].value();
                        if val.is_none() {
                            try!(write!(f, "."));
                        } else {
                            try!(write!(f, "{}", val.unwrap()));
                        }

                        // If another number comes next, add some space between them.
                        if col_idx != (block_idx_c + 1) * SMALL_SIZE - 1 {
                            try!(write!(f, " "));
                        }
                    }
                    // Put in the next piece of border.
                    try!(write!(f, "|"));
                }
            }
            // Add the next row separator.
            try!(write!(f, "\n{}", row_sep));
        }

        Ok(())
    }
}

impl Grid {
    /// Create a new empty grid.
    pub fn empty() -> Grid {
        let mut cells: [[Cell; LARGE_SIZE]; LARGE_SIZE] = Default::default();
        for row_idx in 0..LARGE_SIZE {
            for col_idx in 0..LARGE_SIZE {
                let cell_idx = CellIdx::new(row_idx, col_idx);
                cells[row_idx][col_idx] = Cell::new(cell_idx);
            }
        }

        Grid { cells: cells }
    }

    /// Create a new grid from a string describing it.
    pub fn from_string(givens: &str) -> Grid {

        // Start with an empty grid and fill in all the givens.
        let mut grid = Grid::empty();
        for (idx, digit) in givens.as_bytes().iter().enumerate() {
            let val = digit - b'0';
            if val > 0 && val <= LARGE_SIZE as u8 {
                grid.place_value(CellIdx::new(idx / LARGE_SIZE, idx % LARGE_SIZE),
                                 val as usize);
            }
        }

        grid
    }

    /// Place a value in the cell at the given index, propagating to its neighbours to remove the
    /// value from their candidates.
    pub fn place_value(&mut self, cell_idx: CellIdx, val: usize) {

        // Place the value in the cell.
        self.cells[cell_idx.row][cell_idx.col].set_value(val);

        // Remove it from all neighbouring cells.
        let mut cells_to_eliminate = Vec::new();

        for cell in self.row_from_cell(cell_idx).cells() {
            cells_to_eliminate.push(cell.idx());
        }
        for cell in self.column_from_cell(cell_idx).cells() {
            cells_to_eliminate.push(cell.idx());
        }
        for cell in self.block_from_cell(cell_idx).cells() {
            cells_to_eliminate.push(cell.idx());
        }

        for idx in cells_to_eliminate {
            self.cells[idx.row][idx.col].remove_candidate(val);
        }
    }

    /// Remove a value from the cell at the given index.
    pub fn eliminate_value(&mut self, cell_idx: CellIdx, val: usize) {
        self.cells[cell_idx.row][cell_idx.col].remove_candidate(val);
    }

    /// Get the row having the given index.
    fn row(&self, row_idx: usize) -> Region {
        let cells = (0..LARGE_SIZE).map(|col_idx| &self.cells[row_idx][col_idx]).collect();
        Region::new(cells)
    }

    /// Get the column having the given index.
    fn column(&self, col_idx: usize) -> Region {
        let cells = (0..LARGE_SIZE).map(|row_idx| &self.cells[row_idx][col_idx]).collect();
        Region::new(cells)
    }

    /// Get the block having the given index.
    fn block(&self, row_idx: usize, col_idx: usize) -> Region {
        let base_row = SMALL_SIZE * row_idx;
        let base_col = SMALL_SIZE * col_idx;
        let cells = (0..LARGE_SIZE)
            .map(|idx| &self.cells[base_row + idx / SMALL_SIZE][base_col + idx % SMALL_SIZE])
            .collect();

        Region::new(cells)
    }

    /// Get the row that the given cell is a part of.
    pub fn row_from_cell(&self, cell_idx: CellIdx) -> Region {
        self.row(cell_idx.row)
    }

    /// Get the column that the given cell is a part of.
    pub fn column_from_cell(&self, cell_idx: CellIdx) -> Region {
        self.column(cell_idx.col)
    }

    /// Get the block that the given cell is a part of.
    pub fn block_from_cell(&self, cell_idx: CellIdx) -> Region {
        self.block(cell_idx.row / SMALL_SIZE, cell_idx.col / SMALL_SIZE)
    }

    /// All rows of the grid.
    pub fn rows(&self) -> Vec<Region> {
        (0..LARGE_SIZE).map(|x| self.row(x)).collect()
    }

    /// All columns of the grid.
    pub fn columns(&self) -> Vec<Region> {
        (0..LARGE_SIZE).map(|x| self.column(x)).collect()
    }

    /// All blocks of the grid.
    pub fn blocks(&self) -> Vec<Region> {
        (0..LARGE_SIZE).map(|x| self.block(x / SMALL_SIZE, x % SMALL_SIZE)).collect()
    }

    /// All regions of the grid.
    pub fn regions(&self) -> Vec<Region> {
        let mut acc = Vec::new();
        acc.extend(self.rows());
        acc.extend(self.columns());
        acc.extend(self.blocks());
        acc
    }

    /// All the cells of the grid.
    pub fn cells(&self) -> Vec<&Cell> {
        (0..LARGE_SIZE).flat_map(|x| &self.cells[x]).collect()
    }
}