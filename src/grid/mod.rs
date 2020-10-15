//! A structure representing a Sudoku grid.

pub mod candidateset;
pub mod cell;
pub mod cellset;
mod fixed_size;
mod regions;
pub mod variants;

use self::candidateset::CandidateSet;
use self::cell::Cell;
use self::cellset::CellSet;
pub use self::fixed_size::GridSize;

use strategies::Deduction;
use strategies::Deduction::*;

use std::fmt;

/// A named type for indexing cells of the grid.
pub type CellIdx = usize;

/// A named type for candidates of a cell.
pub type Candidate = usize;

// A simple enum for choosing between rows and columns
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RowOrColumn {
    Row,
    Column,
}

/// A Sudoku grid
#[derive(Clone)]
pub struct Grid<T: GridSize> {

    /// The cells of the grid, in row-major order
    cells: Vec<Cell<T>>,

    /// The rows of the grid
    rows: Vec<CellSet<T>>,

    /// The columns of the grid
    columns: Vec<CellSet<T>>,

    /// The non-row and non-column regions of the grid
    extra_regions: Vec<CellSet<T>>,

    /// All regions (including rows and columns) of the grid
    all_regions: Vec<CellSet<T>>,

    /// The neighbours for each cell of the grid
    neighbours: Vec<CellSet<T>>,
}

impl <T: GridSize> fmt::Display for Grid<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let dashes = "+".to_string() + &String::from_utf8(vec![b'-'; 3 * T::size()]).unwrap() + "+";

        write!(f, "{}\n", dashes)?;

        for row in self.rows() {
            write!(f, "|")?;
            for value in row.map(|ix| self.value(ix).unwrap_or(0)) {
                write!(f, "{:^3}", if value == 0 { ".".to_string() } else { value.to_string() })?;
            }
            write!(f, "|\n")?;
        }

        write!(f, "{}", dashes)
    }
}

impl<T: GridSize> Grid<T> {

    /// Create a new, empty `Grid` with the given regions and additional (non-regional) neighbours
    pub fn empty(regions: &[CellSet<T>], additional_neighbours: &[CellSet<T>]) -> Grid<T> {

        let rows = Grid::create_rows();
        let columns = Grid::create_columns();
        let extra_regions = regions.to_vec();

        let all_regions: Vec<_> = extra_regions.iter()
            .chain(rows.iter())
            .chain(columns.iter())
            .map(|x| x.clone())
            .collect();

        let neighbours = Grid::create_neighbours(&all_regions, additional_neighbours);

        Grid {
            cells: vec![Cell::empty(); T::size() * T::size()],
            rows: rows,
            columns: columns,
            extra_regions: extra_regions,
            all_regions: all_regions,
            neighbours: neighbours,
        }
    }

    /// Place a value in the given cell, propagating eliminations though the grid
    pub fn place_value(&mut self, cell: CellIdx, val: Candidate) {
        self.cells[cell].set_value(val);
        for neighbour in self.neighbours(cell).iter() {
            self.eliminate_value(neighbour, val);
        }
    }

    /// Remove a value from the cell at the given index
    pub fn eliminate_value(&mut self, cell: CellIdx, val: Candidate) {
        self.cells[cell].remove_candidate(val);
    }

    /// Apply the given deduction to the grid
    pub fn apply_deduction(&mut self, deduction: Deduction) {
        match deduction {
            Placement(cell, val) => self.place_value(cell, val),
            Elimination(cell, val) => self.eliminate_value(cell, val),
            Contradiction => {},
        }
    }

    /// Check if the given cell has a particular candidate
    pub fn has_candidate(&self, cell: CellIdx, val: Candidate) -> bool {
        self.cells[cell].has_candidate(val)
    }

    /// Check if the given cell is empty
    pub fn is_empty(&self, cell: CellIdx) -> bool {
        self.cells[cell].is_empty()
    }

    /// Check if the grid is fully solved
    pub fn is_solved(&self) -> bool {
        self.cells().iter().all(|ix| !self.is_empty(ix))
    }

    /// Get the first candidate that can go in the given cell
    pub fn first_candidate(&self, cell: CellIdx) -> Option<Candidate> {
        self.cells[cell].first_candidate()
    }

    /// Get the number of candidates for the given cell
    pub fn num_candidates(&self, cell: CellIdx) -> usize {
        self.cells[cell].num_candidates()
    }

    /// Get the value in the given cell
    pub fn value(&self, cell: CellIdx) -> Option<Candidate> {
        self.cells[cell].value()
    }

    /// Get the candidates for the given cell
    pub fn candidates(&self, cell: CellIdx) -> CandidateSet<T> {
        self.cells[cell].candidates()
    }

    /// Get all of the empty cells
    pub fn empty_cells(&self) -> CellSet<T> {
        CellSet::full().filter(|&cell| self.is_empty(cell))
    }

    /// Get the cells which are able to hold a particular value
    pub fn cells_with_candidate(&self, value: Candidate) -> CellSet<T> {
        let cells = self.cells().iter()
            .filter_map(|cell| if self.has_candidate(cell, value) { Some(cell) } else { None });
        CellSet::from_cells(cells)
    }

    /// Get the cells which have a particular number of candidates
    pub fn cells_with_n_candidates(&self, n: usize) -> CellSet<T> {
        CellSet::full().filter(|&cell| self.num_candidates(cell) == n)
    }

    /// Determine if a particular value has been placed in the given region
    pub fn value_placed_in_region(&self, value: Candidate, region: &CellSet<T>) -> bool {
        region.iter().any(|cell| self.value(cell) == Some(value))
    }

    /// Determine if a particular candidate appears in the given region
    pub fn candidate_in_region(&self, candidate: Candidate, region: &CellSet<T>) -> bool {
        region.iter().any(|cell| self.candidates(cell).has_candidate(candidate))
    }

    /// Get the empty cells from the given region
    pub fn empty_cells_in_region(&self, region: &CellSet<T>) -> CellSet<T> {
        region.filter(|&cell| self.is_empty(cell))
    }

    /// Get the cells in the given region which contain a particular value
    pub fn cells_with_candidate_in_region(&self, value: Candidate, region: &CellSet<T>) -> CellSet<T> {
        region.filter(|&cell| self.has_candidate(cell, value))
    }

    /// Get the values which appear in a given region
    pub fn values_in_region(&self, region: &CellSet<T>) -> CandidateSet<T> {
        CandidateSet::from_candidates(region.iter().filter_map(|cell| self.value(cell)))
    }

    /// Get the values which are missing from a given region
    pub fn values_missing_from_region(&self, region: &CellSet<T>) -> CandidateSet<T> {
        !self.values_in_region(region)
    }

    /// Get all candidates which appear in at least one of the given cells
    pub fn all_candidates_from_region(&self, region: &CellSet<T>) -> CandidateSet<T> {
        region.iter().fold(CandidateSet::empty(), |acc, cell| acc | self.candidates(cell))
    }

    /// Get all cells in the given region which contain any of the given candidates
    pub fn cells_with_candidates_in_region(&self, candidates: &CandidateSet<T>, region: &CellSet<T>) -> CellSet<T> {
        region.filter(|&cell| candidates.iter().any(|val| self.has_candidate(cell, val)))
    }

    /// Get all cells in the given region with a particular number of candidates
    pub fn cells_with_n_candidates_in_region(&self, n: usize, region: &CellSet<T>) -> CellSet<T> {
        region.filter(|&cell| self.num_candidates(cell) == n)
    }

    /// Get the cells in the given region which have exactly the given candidates.
    pub fn cells_with_exact_candidates_in_region(&self, candidates: &CandidateSet<T>, region: &CellSet<T>) -> CellSet<T> {
        region.filter(|&cell| self.candidates(cell) == *candidates)
    }

    /// Determine the rows for this `Grid`
    fn create_rows() -> Vec<CellSet<T>> {
        let size = T::size();
        (0..size).map(|idx| CellSet::from_cells((0..size).map(|jdx| idx * size + jdx))).collect()
    }

    /// Determine the columns for this `Grid`
    fn create_columns() -> Vec<CellSet<T>> {
        let size = T::size();
        (0..size).map(|idx| CellSet::from_cells((0..size).map(|jdx| jdx * size + idx))).collect()
    }

    /// Determine the neighbours for each cell of the `Grid` using the given regions
    fn create_neighbours(all_regions: &[CellSet<T>], additional_neighbours: &[CellSet<T>]) -> Vec<CellSet<T>> {

        let mut neighbours = additional_neighbours.to_vec();

        for cell in 0..T::size() * T::size() {
            for neighbour in neighbours[cell].iter() {
                neighbours[neighbour].add_cell(cell);
            }
        }

        for region in all_regions {
            for cell in region.iter() {
                neighbours[cell] |= region;
            }
        }

        for cell in 0..T::size() * T::size() {
            neighbours[cell].remove_cell(cell);
        }

        neighbours
    }
}
