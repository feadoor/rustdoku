//! A structure representing the state of a Sudoku grid.

mod cell;
mod region;

use self::cell::Cell;
use self::region::{Row, Column, Block, Cells};
use strategies::Deduction;

/// The size of the internal regions in a Sudoku grid.
pub const SMALL_SIZE: usize = 3;
/// The overall size of a Sudoku grid.
pub const LARGE_SIZE: usize = SMALL_SIZE * SMALL_SIZE;

/// A type that lets us refer to coordinates in the grid.
#[derive(Clone, Copy, Default)]
pub struct CellIdx {
    row: usize,
    col: usize
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

impl Grid {

    /// Create a new grid with the given cells filled in.
    pub fn new(givens: [[usize; LARGE_SIZE]; LARGE_SIZE]) -> Grid {
        let mut cells: [[Cell; LARGE_SIZE]; LARGE_SIZE] = Default::default();
        for row_idx in 0..LARGE_SIZE {
            for col_idx in 0..LARGE_SIZE {
                let cell_idx = CellIdx::new(row_idx, col_idx);
                cells[row_idx][col_idx] = Cell::new(cell_idx);
            }
        }

        let mut grid = Grid { cells: cells };
        for row_idx in 0..LARGE_SIZE {
            for col_idx in 0..LARGE_SIZE {
                if givens[row_idx][col_idx] != 0 {
                    grid.place_value(CellIdx::new(row_idx, col_idx), givens[row_idx][col_idx]);
                }
            }
        }

        grid
    }

    /// Get a reference to the cell at the given index.
    pub fn get_cell(&self, cell_idx: CellIdx) -> &Cell {
        &self.cells[cell_idx.row][cell_idx.col]
    }

    /// Place a value in the cell at the given index, propagating to its neighbours to remove the
    /// value from their candidates.
    pub fn place_value(&mut self, cell_idx: CellIdx, val: usize) {

        // Place the value in the cell.
        self.cells[cell_idx.row][cell_idx.col].set_value(val);

        // Remove it from all neighbouring cells.
        let mut cells_to_eliminate = Vec::new();

        for cell in self.get_row(cell_idx) {
            cells_to_eliminate.push(cell.get_idx());
        }
        for cell in self.get_column(cell_idx) {
            cells_to_eliminate.push(cell.get_idx());
        }
        for cell in self.get_block(cell_idx) {
            cells_to_eliminate.push(cell.get_idx());
        }

        for idx in cells_to_eliminate {
            self.cells[idx.row][idx.col].remove_candidate(val);
        }
    }

    /// Remove a value from the cell at the given index.
    pub fn eliminate_value(&mut self, cell_idx: CellIdx, val: usize) {
        self.cells[cell_idx.row][cell_idx.col].remove_candidate(val);
    }

    /// Get the row that the given cell is a part of.
    pub fn get_row(&self, cell_idx: CellIdx) -> Row {
        Row::new(&self, cell_idx.row)
    }

    /// Get the column that the given cell is a part of.
    pub fn get_column(&self, cell_idx: CellIdx) -> Column {
        Column::new(&self, cell_idx.col)
    }

    /// Get the block that the given cell is a part of.
    pub fn get_block(&self, cell_idx: CellIdx) -> Block {
        Block::new(&self, cell_idx.row / 3, cell_idx.col / 3)
    }

    /// All rows of the grid.
    pub fn rows(&self) -> Vec<Row> {
        (0..LARGE_SIZE).map(|x| Row::new(&self, x)).collect()
    }

    /// All columns of the grid.
    pub fn columns(&self) -> Vec<Column> {
        (0..LARGE_SIZE).map(|x| Column::new(&self, x)).collect()
    }

    /// All blocks of the grid.
    pub fn blocks(&self) -> Vec<Block> {
        (0..LARGE_SIZE).map(|x| Block::new(&self, x / 3, x % 3)).collect()
    }

    /// An iterator over all the cells of the grid.
    pub fn cells(&self) -> Cells {
        Cells::new(&self)
    }

    /// Decide if the grid is solved or not.
    pub fn is_solved(&self) -> bool {
        self.cells().all(|x| !x.is_empty())
    }

    /// Apply the results of a deduction to the grid.
    pub fn apply_deduction(&mut self, deduction: Deduction) {
        match deduction {
            Deduction::Placement(cell_idx, val) => self.place_value(cell_idx, val),
            Deduction::Elimination(cell_idx, val) => self.eliminate_value(cell_idx, val),
        }
    }
}