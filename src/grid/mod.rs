//! A structure representing a Sudoku grid.

mod cell;
pub mod cellset;
mod utils;

use std::fmt;

use ansi_term::Style;

use self::cell::Cell;
use self::cellset::CellSet;

/// A named type for indexing cells of the grid.
pub type CellIdx = usize;

/// Errors that can arise when reading in a grid from a string representation.
pub enum GridParseError {
    BadLength,
    Contradiction(usize),
}

impl fmt::Display for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GridParseError::*;
        match *self {
            BadLength => write!(f, "The grid does not have length 81"),
            Contradiction(pos) => write!(f, "The clue at position {} contradicts the others", pos),
        }
    }
}

impl fmt::Debug for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

/// A Sudoku grid.
pub struct Grid {
    /// The cells of the grid, in row-major order. Although this is a `Vec<Cell>`, it always has
    /// exactly 81 entries.
    cells: Vec<Cell>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Work out the most candidates that will need to fit in any cell.
        let mut max_c = Grid::cells().iter().map(|ix| self.num_candidates(ix)).max().unwrap();
        if max_c == 0 {
            max_c = 1;
        }

        // Create a String which will separate groups of rows in the grid.
        let dashes = String::from_utf8(vec![b'-'; 3 * max_c + 2]).unwrap();
        let row_sep = (0..3).map(|_| "+".to_string() + &dashes).collect::<String>() + "+";

        try!(write!(f, "{}", row_sep));

        // Iterate over every group of three cells in the grid.
        for cell_idx in 0..81 {

            // Start each row off with some border.
            if cell_idx % 9 == 0 {
                try!(write!(f, "\n|"));
            }

            // Write either the number in the cell, or all its candidates if there isn't one.
            match self.value(cell_idx) {
                Some(val) => {
                    try!(write!(f, "{}", Style::new().bold().paint(format!("{}", val))));
                    try!(write!(f, "{}", String::from_utf8(vec![b' '; max_c - 1]).unwrap()));
                }
                None => {
                    let mut written = 0;
                    let mut candidates = self.candidates(cell_idx);
                    while candidates != 0 {
                        try!(write!(f, "{}", candidates.trailing_zeros() as usize + 1));
                        candidates &= candidates - 1;
                        written += 1;
                    }
                    try!(write!(f, "{}", String::from_utf8(vec![b' '; max_c - written]).unwrap()));
                }
            }

            // If another number comes next, add some space between them. Otherwise, write the
            // next piece of border.
            match (cell_idx + 1) % 3 {
                0 => try!(write!(f, "|")),
                _ => try!(write!(f, " ")),
            }

            // Add the next row separator if needed.
            if (cell_idx + 1) % 27 == 0 {
                try!(write!(f, "\n{}", row_sep));
            }
        }

        Ok(())
    }
}

impl Grid {
    /// Create a new, empty `Grid`.
    pub fn empty() -> Grid {
        Grid { cells: vec![Cell::new(); 81] }
    }

    /// Create a new grid from a string describing it.
    pub fn from_str(givens: &str) -> Result<Grid, GridParseError> {

        // Check that the given string has the right length.
        if givens.len() != 81 {
            return Err(GridParseError::BadLength);
        }

        // Start with an empty grid and fill in all the givens.
        let mut grid = Grid::empty();
        for (idx, digit) in givens.as_bytes().iter().enumerate() {
            let val = digit - b'0';
            if val > 0 && val <= 9 as u8 {
                if !grid.has_candidate(idx, val as usize) {
                    return Err(GridParseError::Contradiction(idx));
                } else {
                    grid.place_value(idx, val as usize);
                }
            }
        }

        Ok(grid)
    }

    /// Place a value in the cell at the given index, propagating to its neighbours to remove the
    /// value from their candidates.
    pub fn place_value(&mut self, cell_idx: CellIdx, val: usize) {

        // Place the value in the cell.
        self.cells[cell_idx].set_value(val);

        // Remove it from all neighbouring cells.
        for neighbour_idx in Grid::neighbours(cell_idx).iter() {
            self.eliminate_value(neighbour_idx, val);
        }
    }

    /// Remove a value from the cell at the given index.
    pub fn eliminate_value(&mut self, cell_idx: CellIdx, val: usize) {
        self.cells[cell_idx].remove_candidate(val);
    }

    /// Check if the given cell has a particular candidate.
    pub fn has_candidate(&self, cell_idx: CellIdx, val: usize) -> bool {
        self.cells[cell_idx].has_candidate(val)
    }

    /// Check if the given cell is empty.
    pub fn is_empty(&self, cell_idx: CellIdx) -> bool {
        self.cells[cell_idx].is_empty()
    }

    /// Get the first candidate that can go in the given cell.
    pub fn first_candidate(&self, cell_idx: CellIdx) -> usize {
        self.cells[cell_idx].first_candidate()
    }

    /// Get the number of candidates for the given cell.
    pub fn num_candidates(&self, cell_idx: CellIdx) -> usize {
        self.cells[cell_idx].num_candidates()
    }

    /// Get the value in the given cell.
    pub fn value(&self, cell_idx: CellIdx) -> Option<usize> {
        self.cells[cell_idx].value()
    }

    /// Get the candidates for the given cell.
    pub fn candidates(&self, cell_idx: CellIdx) -> usize {
        self.cells[cell_idx].candidates()
    }

    /// Get the cells which are able to hold a particular value.
    pub fn cells_with_candidate(&self, value: usize) -> CellSet {
        let cells = Grid::cells().iter()
            .filter_map(|ix| if self.has_candidate(ix, value) { Some(ix) } else { None });

        CellSet::from_cells(cells)
    }
}