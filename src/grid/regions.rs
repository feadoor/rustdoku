//! Some utility functions for working with regions of a Sudoku grid.

use grid::{CellIdx, Grid, RowOrColumn};
use grid::RowOrColumn::*;
use grid::cellset::CellSet;
use grid::fixed_size::GridSize;

impl<'a, T: GridSize> Grid<T> {

    /// The human name for the given cell
    pub fn cell_name(&self, cell: CellIdx) -> String {
        format!("r{}c{}", (cell / T::size()) + 1, (cell % T::size()) + 1)
    }

    /// The human name for the given region
    pub fn region_name(&self, region: &CellSet<T>) -> String {

        for (idx, row) in self.rows().iter().enumerate() {
            if region == row {
                return format!("Row {}", idx + 1);
            }
        }

        for (idx, column) in self.columns().iter().enumerate() {
            if region == column {
                return format!("Column {}", idx + 1);
            }
        }

        for (idx, extra_region) in self.extra_regions().iter().enumerate() {
            if region == extra_region {
                return format!("Region {}", idx + 1);
            }
        }

        format!("({})", region.iter().map(|c| self.cell_name(c)).collect::<Vec<String>>().join(", "))
    }

    /// All the values that can be placed in a cell of the grid
    pub fn values(&self) -> Vec<usize> {
        (1..T::size() + 1).collect()
    }

    /// All the cells of the grid
    pub fn cells(&self) -> CellSet<T> {
        CellSet::full()
    }

    /// All rows for a grid
    pub fn rows(&self) -> &[CellSet<T>] {
        &self.rows
    }

    /// All columns for a grid
    pub fn columns(&self) -> &[CellSet<T>] {
        &self.columns
    }

    /// All extra regions for a grid
    pub fn extra_regions(&self) -> &[CellSet<T>] {
        &self.extra_regions
    }

    /// All regions for a grid
    pub fn all_regions(&self) -> &[CellSet<T>] {
        &self.all_regions
    }

    /// The neighbours for a particular cell
    pub fn neighbours(&'a self, cell: CellIdx) -> &'a CellSet<T> {
        &self.neighbours[cell]
    }

    /// Return the row which contains all of the given cells
    pub fn row_containing(&self, cells: &CellSet<T>) -> Option<CellSet<T>> {
        for row in self.rows() {
            if row & cells == *cells { return Some(row.clone()); }
        }
        None
    }

    /// Return the column which contains all of the given cells
    pub fn column_containing(&self, cells: &CellSet<T>) -> Option<CellSet<T>> {
        for column in self.columns() {
            if column & cells == *cells { return Some(column.clone()); }
        }
        None
    }

    /// Get the rows which intersect the given `CellSet`
    pub fn intersecting_rows(&self, cells: &CellSet<T>) -> Vec<CellSet<T>> {
        self.rows().iter().filter(|&row| !((row & cells).is_empty())).map(|row| row.clone()).collect()
    }

    /// Get the columns which intersect the given `CellSet`
    pub fn intersecting_columns(&self, cells: &CellSet<T>) -> Vec<CellSet<T>> {
        self.columns().iter().filter(|&row| !((row & cells).is_empty())).map(|column| column.clone()).collect()
    }

    /// Return all regions which contain all of the given cells
    pub fn all_regions_containing(&self, cells: &CellSet<T>) -> Vec<CellSet<T>> {
        self.all_regions().iter().filter(|&region| region.contains_all(cells)).map(|region| region.clone()).collect()
    }

    /// Group the cells in the given `CellSet` by either rows or columns
    pub fn group_cells_by(&self, cells: &CellSet<T>, variety: RowOrColumn) -> Vec<CellSet<T>> {

        let regions = match variety {
            Row => self.rows(),
            Column => self.columns(),
        };

        regions.iter()
            .map(|region| region & cells)
            .filter(|intersection| !intersection.is_empty())
            .collect()
    }

    /// Get the common neighbours of all of the given cells
    pub fn common_neighbours(&self, cells: &CellSet<T>) -> CellSet<T> {
        CellSet::intersection(&cells.map(|cell| self.neighbours(cell)))
    }
}
