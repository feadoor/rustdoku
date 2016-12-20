//! Some utility functions for working with regions of a Sudoku grid.

use grid::{CellIdx, Grid};
use grid::cellset::CellSet;

/// The cells in a given row.
static ROWS_INT: [[CellIdx; 9]; 9] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];

/// The cells in a given column.
static COLUMNS_INT: [[CellIdx; 9]; 9] = [
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
];

/// The cells in a given block.
static BLOCKS_INT: [[CellIdx; 9]; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

impl Grid {

    /// Get the indices of the cells which share a row with the given cell.
    pub fn row(cell_idx: CellIdx) -> &'static [CellIdx] {
        static ROW_INDICES: [usize; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
        ];

        &ROWS_INT[ROW_INDICES[cell_idx]]
    }

    /// Get the indices of the cells which share a column with the given cell.
    pub fn column(cell_idx: CellIdx) -> &'static [CellIdx] {
        static COLUMN_INDICES: [usize; 81] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        &COLUMNS_INT[COLUMN_INDICES[cell_idx]]
    }

    /// Get the indices of the cells which share a block with the given cell.
    pub fn block(cell_idx: CellIdx) -> &'static [CellIdx] {
        static BLOCK_INDICES: [usize; 81] = [
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];

        &BLOCKS_INT[BLOCK_INDICES[cell_idx]]
    }

    /// All the rows of the grid.
    pub fn rows() -> &'static [[CellIdx; 9]] {
        static ROWS: [[CellIdx; 9]; 9] = [
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80],
        ];

        &ROWS
    }

    /// All the columns of the grid.
    pub fn columns() -> &'static [[CellIdx; 9]] {
        static COLUMNS: [[CellIdx; 9]; 9] = [
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
        ];

        &COLUMNS
    }

    /// All the blocks of the grid.
    pub fn blocks() -> &'static [[CellIdx; 9]] {
        static BLOCKS: [[CellIdx; 9]; 9] = [
            [0, 1, 2, 9, 10, 11, 18, 19, 20],
            [3, 4, 5, 12, 13, 14, 21, 22, 23],
            [6, 7, 8, 15, 16, 17, 24, 25, 26],
            [27, 28, 29, 36, 37, 38, 45, 46, 47],
            [30, 31, 32, 39, 40, 41, 48, 49, 50],
            [33, 34, 35, 42, 43, 44, 51, 52, 53],
            [54, 55, 56, 63, 64, 65, 72, 73, 74],
            [57, 58, 59, 66, 67, 68, 75, 76, 77],
            [60, 61, 62, 69, 70, 71, 78, 79, 80],
        ];

        &BLOCKS
    }

    /// All the regions of the grid.
    pub fn regions() -> &'static [[CellIdx; 9]] {
        static REGIONS: [[CellIdx; 9]; 27] = [
            [0, 1, 2, 9, 10, 11, 18, 19, 20],
            [3, 4, 5, 12, 13, 14, 21, 22, 23],
            [6, 7, 8, 15, 16, 17, 24, 25, 26],
            [27, 28, 29, 36, 37, 38, 45, 46, 47],
            [30, 31, 32, 39, 40, 41, 48, 49, 50],
            [33, 34, 35, 42, 43, 44, 51, 52, 53],
            [54, 55, 56, 63, 64, 65, 72, 73, 74],
            [57, 58, 59, 66, 67, 68, 75, 76, 77],
            [60, 61, 62, 69, 70, 71, 78, 79, 80],
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80],
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
        ];

        &REGIONS
    }

    /// All the cells of the grid.
    pub fn cells() -> &'static [CellIdx] {
        static CELLS: [CellIdx; 81] = [
             0,  1,  2,  3,  4,  5,  6,  7,  8,
             9, 10, 11, 12, 13, 14, 15, 16, 17,
            18, 19, 20, 21, 22, 23, 24, 25, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35,
            36, 37, 38, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 52, 53,
            54, 55, 56, 57, 58, 59, 60, 61, 62,
            63, 64, 65, 66, 67, 68, 69, 70, 71,
            72, 73, 74, 75, 76, 77, 78, 79, 80,
        ];

        &CELLS
    }

    /// All the values that can be placed in a cell of the grid.
    pub fn values() -> &'static [usize] {
        static VALUES: [usize; 9] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];

        &VALUES
    }

    /// Get the indices of the neighbours of a particular cell.
    pub fn neighbours(cell_idx: CellIdx) -> &'static [CellIdx] {
        static NEIGHBOURS: [[CellIdx; 21]; 81] = [
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, 72],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 28, 37, 46, 55, 64, 73],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 29, 38, 47, 56, 65, 74],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 30, 39, 48, 57, 66, 75],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 31, 40, 49, 58, 67, 76],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 32, 41, 50, 59, 68, 77],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 24, 25, 26, 35, 44, 53, 62, 71, 80],
            [0, 1, 2, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 27, 36, 45, 54, 63, 72],
            [0, 1, 2, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 28, 37, 46, 55, 64, 73],
            [0, 1, 2, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 29, 38, 47, 56, 65, 74],
            [3, 4, 5, 9, 10, 11, 12, 13, 14, 15, 16, 17, 21, 22, 23, 30, 39, 48, 57, 66, 75],
            [3, 4, 5, 9, 10, 11, 12, 13, 14, 15, 16, 17, 21, 22, 23, 31, 40, 49, 58, 67, 76],
            [3, 4, 5, 9, 10, 11, 12, 13, 14, 15, 16, 17, 21, 22, 23, 32, 41, 50, 59, 68, 77],
            [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78],
            [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79],
            [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 24, 25, 26, 35, 44, 53, 62, 71, 80],
            [0, 1, 2, 9, 10, 11, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 36, 45, 54, 63, 72],
            [0, 1, 2, 9, 10, 11, 18, 19, 20, 21, 22, 23, 24, 25, 26, 28, 37, 46, 55, 64, 73],
            [0, 1, 2, 9, 10, 11, 18, 19, 20, 21, 22, 23, 24, 25, 26, 29, 38, 47, 56, 65, 74],
            [3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 23, 24, 25, 26, 30, 39, 48, 57, 66, 75],
            [3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 23, 24, 25, 26, 31, 40, 49, 58, 67, 76],
            [3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 23, 24, 25, 26, 32, 41, 50, 59, 68, 77],
            [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 33, 42, 51, 60, 69, 78],
            [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 34, 43, 52, 61, 70, 79],
            [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 35, 44, 53, 62, 71, 80],
            [0, 9, 18, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 54, 63, 72],
            [1, 10, 19, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 55, 64, 73],
            [2, 11, 20, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 56, 65, 74],
            [3, 12, 21, 27, 28, 29, 30, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 57, 66, 75],
            [4, 13, 22, 27, 28, 29, 30, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 58, 67, 76],
            [5, 14, 23, 27, 28, 29, 30, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 59, 68, 77],
            [6, 15, 24, 27, 28, 29, 30, 31, 32, 33, 34, 35, 42, 43, 44, 51, 52, 53, 60, 69, 78],
            [7, 16, 25, 27, 28, 29, 30, 31, 32, 33, 34, 35, 42, 43, 44, 51, 52, 53, 61, 70, 79],
            [8, 17, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 42, 43, 44, 51, 52, 53, 62, 71, 80],
            [0, 9, 18, 27, 28, 29, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 54, 63, 72],
            [1, 10, 19, 27, 28, 29, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 55, 64, 73],
            [2, 11, 20, 27, 28, 29, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 56, 65, 74],
            [3, 12, 21, 30, 31, 32, 36, 37, 38, 39, 40, 41, 42, 43, 44, 48, 49, 50, 57, 66, 75],
            [4, 13, 22, 30, 31, 32, 36, 37, 38, 39, 40, 41, 42, 43, 44, 48, 49, 50, 58, 67, 76],
            [5, 14, 23, 30, 31, 32, 36, 37, 38, 39, 40, 41, 42, 43, 44, 48, 49, 50, 59, 68, 77],
            [6, 15, 24, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 51, 52, 53, 60, 69, 78],
            [7, 16, 25, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 51, 52, 53, 61, 70, 79],
            [8, 17, 26, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 51, 52, 53, 62, 71, 80],
            [0, 9, 18, 27, 28, 29, 36, 37, 38, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 63, 72],
            [1, 10, 19, 27, 28, 29, 36, 37, 38, 45, 46, 47, 48, 49, 50, 51, 52, 53, 55, 64, 73],
            [2, 11, 20, 27, 28, 29, 36, 37, 38, 45, 46, 47, 48, 49, 50, 51, 52, 53, 56, 65, 74],
            [3, 12, 21, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 50, 51, 52, 53, 57, 66, 75],
            [4, 13, 22, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 50, 51, 52, 53, 58, 67, 76],
            [5, 14, 23, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 50, 51, 52, 53, 59, 68, 77],
            [6, 15, 24, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 60, 69, 78],
            [7, 16, 25, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 61, 70, 79],
            [8, 17, 26, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 62, 71, 80],
            [0, 9, 18, 27, 36, 45, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
            [1, 10, 19, 28, 37, 46, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
            [2, 11, 20, 29, 38, 47, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
            [3, 12, 21, 30, 39, 48, 54, 55, 56, 57, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77],
            [4, 13, 22, 31, 40, 49, 54, 55, 56, 57, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77],
            [5, 14, 23, 32, 41, 50, 54, 55, 56, 57, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77],
            [6, 15, 24, 33, 42, 51, 54, 55, 56, 57, 58, 59, 60, 61, 62, 69, 70, 71, 78, 79, 80],
            [7, 16, 25, 34, 43, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 69, 70, 71, 78, 79, 80],
            [8, 17, 26, 35, 44, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 69, 70, 71, 78, 79, 80],
            [0, 9, 18, 27, 36, 45, 54, 55, 56, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74],
            [1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74],
            [2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74],
            [3, 12, 21, 30, 39, 48, 57, 58, 59, 63, 64, 65, 66, 67, 68, 69, 70, 71, 75, 76, 77],
            [4, 13, 22, 31, 40, 49, 57, 58, 59, 63, 64, 65, 66, 67, 68, 69, 70, 71, 75, 76, 77],
            [5, 14, 23, 32, 41, 50, 57, 58, 59, 63, 64, 65, 66, 67, 68, 69, 70, 71, 75, 76, 77],
            [6, 15, 24, 33, 42, 51, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 78, 79, 80],
            [7, 16, 25, 34, 43, 52, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 78, 79, 80],
            [8, 17, 26, 35, 44, 53, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 78, 79, 80],
            [0, 9, 18, 27, 36, 45, 54, 55, 56, 63, 64, 65, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 64, 65, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 65, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [3, 12, 21, 30, 39, 48, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [4, 13, 22, 31, 40, 49, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [5, 14, 23, 32, 41, 50, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [6, 15, 24, 33, 42, 51, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [7, 16, 25, 34, 43, 52, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [8, 17, 26, 35, 44, 53, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
        ];

        &NEIGHBOURS[cell_idx]
    }

    /// Determine if a group of cells share a row.
    pub fn same_row(cells: &[&CellIdx]) -> bool {
        static ROWS: [usize; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
        ];

        cells.len() == 0 || cells.iter().all(|&&ix| ROWS[ix] == ROWS[*cells[0]])
    }

    /// Determine if a group of cells share a column.
    pub fn same_column(cells: &[&CellIdx]) -> bool {
        static COLUMNS: [usize; 81] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        cells.len() == 0 || cells.iter().all(|&&ix| COLUMNS[ix] == COLUMNS[*cells[0]])
    }

    /// Determine if a group of cells share a block.
    pub fn same_block(cells: &[&CellIdx]) -> bool {
        static BLOCKS: [usize; 81] = [
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];

        cells.len() == 0 || cells.iter().all(|&&ix| BLOCKS[ix] == BLOCKS[*cells[0]])
    }

    /// Get a `CellSet` representing the cells in the given row.
    fn rowset_int(row_idx: usize) -> &'static CellSet {
        static ROWSETS: [CellSet; 9] = [
            CellSet { hi: 0x0, lo: 0x1FF },
            CellSet { hi: 0x0, lo: 0x3FE00 },
            CellSet { hi: 0x0, lo: 0x7FC0000 },
            CellSet { hi: 0x0, lo: 0xFF8000000 },
            CellSet { hi: 0x0, lo: 0x1FF000000000 },
            CellSet { hi: 0x0, lo: 0x3FE00000000000 },
            CellSet { hi: 0x0, lo: 0x7FC0000000000000 },
            CellSet { hi: 0xFF, lo: 0x8000000000000000 },
            CellSet { hi: 0x1FF00, lo: 0x0 },
        ];

        &ROWSETS[row_idx]
    }

    /// Get a `CellSet` representing the cells in the given column.
    fn colset_int(column_idx: usize) -> &'static CellSet {
        static COLSETS: [CellSet; 9] = [
            CellSet { hi: 0x100, lo: 0x8040201008040201 },
            CellSet { hi: 0x201, lo: 0x0080402010080402 },
            CellSet { hi: 0x402, lo: 0x0100804020100804 },
            CellSet { hi: 0x804, lo: 0x0201008040201008 },
            CellSet { hi: 0x1008, lo: 0x0402010080402010 },
            CellSet { hi: 0x2010, lo: 0x0804020100804020 },
            CellSet { hi: 0x4020, lo: 0x1008040201008040 },
            CellSet { hi: 0x8040, lo: 0x2010080402010080 },
            CellSet { hi: 0x10080, lo: 0x4020100804020100 },
        ];

        &COLSETS[column_idx]
    }

    /// Get a `CellSet` representing the cells in the given block.
    fn blockset_int(block_idx: usize) -> &'static CellSet {
        static BLOCKSETS: [CellSet; 9] = [
            CellSet { hi: 0x0, lo: 0x1c0e07 },
            CellSet { hi: 0x0, lo: 0xe07038 },
            CellSet { hi: 0x0, lo: 0x70381c0 },
            CellSet { hi: 0x0, lo: 0xe07038000000 },
            CellSet { hi: 0x0, lo: 0x70381c0000000 },
            CellSet { hi: 0x0, lo: 0x381c0e00000000 },
            CellSet { hi: 0x703, lo: 0x81c0000000000000 },
            CellSet { hi: 0x381c, lo: 0x0e00000000000000 },
            CellSet { hi: 0x1c0e0, lo: 0x7000000000000000 },
        ];

        &BLOCKSETS[block_idx]
    }

    /// Get the indices of the cells which share a row with the given cell.
    pub fn rowset(cell_idx: CellIdx) -> &'static CellSet {
        static ROW_INDICES: [usize; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
        ];

        Grid::rowset_int(ROW_INDICES[cell_idx])
    }

    /// Get the indices of the cells which share a column with the given cell.
    pub fn colset(cell_idx: CellIdx) -> &'static CellSet {
        static COLUMN_INDICES: [usize; 81] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        Grid::colset_int(COLUMN_INDICES[cell_idx])
    }

    /// Get the indices of the cells which share a block with the given cell.
    pub fn blockset(cell_idx: CellIdx) -> &'static CellSet {
        static BLOCK_INDICES: [usize; 81] = [
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];

        Grid::blockset_int(BLOCK_INDICES[cell_idx])
    }

    /// All row sets for a grid.
    pub fn rowsets() -> &'static [CellSet] {
        static ROWSETS: [CellSet; 9] = [
            CellSet { hi: 0x0, lo: 0x1FF },
            CellSet { hi: 0x0, lo: 0x3FE00 },
            CellSet { hi: 0x0, lo: 0x7FC0000 },
            CellSet { hi: 0x0, lo: 0xFF8000000 },
            CellSet { hi: 0x0, lo: 0x1FF000000000 },
            CellSet { hi: 0x0, lo: 0x3FE00000000000 },
            CellSet { hi: 0x0, lo: 0x7FC0000000000000 },
            CellSet { hi: 0xFF, lo: 0x8000000000000000 },
            CellSet { hi: 0x1FF00, lo: 0x0 },
        ];

        &ROWSETS
    }

    /// All column sets for a grid.
    pub fn colsets() -> &'static [CellSet] {
        static COLSETS: [CellSet; 9] = [
            CellSet { hi: 0x100, lo: 0x8040201008040201 },
            CellSet { hi: 0x201, lo: 0x0080402010080402 },
            CellSet { hi: 0x402, lo: 0x0100804020100804 },
            CellSet { hi: 0x804, lo: 0x0201008040201008 },
            CellSet { hi: 0x1008, lo: 0x0402010080402010 },
            CellSet { hi: 0x2010, lo: 0x0804020100804020 },
            CellSet { hi: 0x4020, lo: 0x1008040201008040 },
            CellSet { hi: 0x8040, lo: 0x2010080402010080 },
            CellSet { hi: 0x10080, lo: 0x4020100804020100 },
        ];

        &COLSETS
    }

    /// All block sets for a grid.
    pub fn blocksets() -> &'static [CellSet] {
        static BLOCKSETS: [CellSet; 9] = [
            CellSet { hi: 0x0, lo: 0x1c0e07 },
            CellSet { hi: 0x0, lo: 0xe07038 },
            CellSet { hi: 0x0, lo: 0x70381c0 },
            CellSet { hi: 0x0, lo: 0xe07038000000 },
            CellSet { hi: 0x0, lo: 0x70381c0000000 },
            CellSet { hi: 0x0, lo: 0x381c0e00000000 },
            CellSet { hi: 0x703, lo: 0x81c0000000000000 },
            CellSet { hi: 0x381c, lo: 0x0e00000000000000 },
            CellSet { hi: 0x1c0e0, lo: 0x7000000000000000 },
        ];

        &BLOCKSETS
    }

    /// The set of neighbours for a particular cell.
    pub fn neighbours_set(cell_idx: CellIdx) -> &'static CellSet {
        static NEIGHBOURS_SETS: [CellSet; 81] = [
            CellSet { hi: 0x100, lo: 0x80402010081c0fff },
            CellSet { hi: 0x201, lo: 0x804020101c0fff },
            CellSet { hi: 0x402, lo: 0x1008040201c0fff },
            CellSet { hi: 0x804, lo: 0x201008040e071ff },
            CellSet { hi: 0x1008, lo: 0x402010080e071ff },
            CellSet { hi: 0x2010, lo: 0x804020100e071ff },
            CellSet { hi: 0x4020, lo: 0x10080402070381ff },
            CellSet { hi: 0x8040, lo: 0x20100804070381ff },
            CellSet { hi: 0x10080, lo: 0x40201008070381ff },
            CellSet { hi: 0x100, lo: 0x80402010081ffe07 },
            CellSet { hi: 0x201, lo: 0x804020101ffe07 },
            CellSet { hi: 0x402, lo: 0x1008040201ffe07 },
            CellSet { hi: 0x804, lo: 0x201008040e3fe38 },
            CellSet { hi: 0x1008, lo: 0x402010080e3fe38 },
            CellSet { hi: 0x2010, lo: 0x804020100e3fe38 },
            CellSet { hi: 0x4020, lo: 0x100804020703ffc0 },
            CellSet { hi: 0x8040, lo: 0x201008040703ffc0 },
            CellSet { hi: 0x10080, lo: 0x402010080703ffc0 },
            CellSet { hi: 0x100, lo: 0x804020100ffc0e07 },
            CellSet { hi: 0x201, lo: 0x80402017fc0e07 },
            CellSet { hi: 0x402, lo: 0x100804027fc0e07 },
            CellSet { hi: 0x804, lo: 0x201008047fc7038 },
            CellSet { hi: 0x1008, lo: 0x402010087fc7038 },
            CellSet { hi: 0x2010, lo: 0x804020107fc7038 },
            CellSet { hi: 0x4020, lo: 0x1008040207ff81c0 },
            CellSet { hi: 0x8040, lo: 0x2010080407ff81c0 },
            CellSet { hi: 0x10080, lo: 0x4020100807ff81c0 },
            CellSet { hi: 0x100, lo: 0x8040e07ff8040201 },
            CellSet { hi: 0x201, lo: 0x80e07ff8080402 },
            CellSet { hi: 0x402, lo: 0x100e07ff8100804 },
            CellSet { hi: 0x804, lo: 0x207038ff8201008 },
            CellSet { hi: 0x1008, lo: 0x407038ff8402010 },
            CellSet { hi: 0x2010, lo: 0x807038ff8804020 },
            CellSet { hi: 0x4020, lo: 0x10381c0ff9008040 },
            CellSet { hi: 0x8040, lo: 0x20381c0ffa010080 },
            CellSet { hi: 0x10080, lo: 0x40381c0ffc020100 },
            CellSet { hi: 0x100, lo: 0x8040fff038040201 },
            CellSet { hi: 0x201, lo: 0x80fff038080402 },
            CellSet { hi: 0x402, lo: 0x100fff038100804 },
            CellSet { hi: 0x804, lo: 0x2071ff1c0201008 },
            CellSet { hi: 0x1008, lo: 0x4071ff1c0402010 },
            CellSet { hi: 0x2010, lo: 0x8071ff1c0804020 },
            CellSet { hi: 0x4020, lo: 0x10381ffe01008040 },
            CellSet { hi: 0x8040, lo: 0x20381ffe02010080 },
            CellSet { hi: 0x10080, lo: 0x40381ffe04020100 },
            CellSet { hi: 0x100, lo: 0x807fe07038040201 },
            CellSet { hi: 0x201, lo: 0xbfe07038080402 },
            CellSet { hi: 0x402, lo: 0x13fe07038100804 },
            CellSet { hi: 0x804, lo: 0x23fe381c0201008 },
            CellSet { hi: 0x1008, lo: 0x43fe381c0402010 },
            CellSet { hi: 0x2010, lo: 0x83fe381c0804020 },
            CellSet { hi: 0x4020, lo: 0x103ffc0e01008040 },
            CellSet { hi: 0x8040, lo: 0x203ffc0e02010080 },
            CellSet { hi: 0x10080, lo: 0x403ffc0e04020100 },
            CellSet { hi: 0x703, lo: 0xffc0201008040201 },
            CellSet { hi: 0x703, lo: 0xffc0402010080402 },
            CellSet { hi: 0x703, lo: 0xffc0804020100804 },
            CellSet { hi: 0x381c, lo: 0x7fc1008040201008 },
            CellSet { hi: 0x381c, lo: 0x7fc2010080402010 },
            CellSet { hi: 0x381c, lo: 0x7fc4020100804020 },
            CellSet { hi: 0x1c0e0, lo: 0x7fc8040201008040 },
            CellSet { hi: 0x1c0e0, lo: 0x7fd0080402010080 },
            CellSet { hi: 0x1c0e0, lo: 0x7fe0100804020100 },
            CellSet { hi: 0x7ff, lo: 0x81c0201008040201 },
            CellSet { hi: 0x7ff, lo: 0x81c0402010080402 },
            CellSet { hi: 0x7ff, lo: 0x81c0804020100804 },
            CellSet { hi: 0x38ff, lo: 0x8e01008040201008 },
            CellSet { hi: 0x38ff, lo: 0x8e02010080402010 },
            CellSet { hi: 0x38ff, lo: 0x8e04020100804020 },
            CellSet { hi: 0x1c0ff, lo: 0xf008040201008040 },
            CellSet { hi: 0x1c0ff, lo: 0xf010080402010080 },
            CellSet { hi: 0x1c0ff, lo: 0xf020100804020100 },
            CellSet { hi: 0x1ff03, lo: 0x81c0201008040201 },
            CellSet { hi: 0x1ff03, lo: 0x81c0402010080402 },
            CellSet { hi: 0x1ff03, lo: 0x81c0804020100804 },
            CellSet { hi: 0x1ff1c, lo: 0xe01008040201008 },
            CellSet { hi: 0x1ff1c, lo: 0xe02010080402010 },
            CellSet { hi: 0x1ff1c, lo: 0xe04020100804020 },
            CellSet { hi: 0x1ffe0, lo: 0x7008040201008040 },
            CellSet { hi: 0x1ffe0, lo: 0x7010080402010080 },
            CellSet { hi: 0x1ffe0, lo: 0x7020100804020100 },
        ];

        &NEIGHBOURS_SETS[cell_idx]
    }
}