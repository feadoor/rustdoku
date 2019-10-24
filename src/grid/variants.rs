//! Specific grids to deal with particular Sudoku variants

use crate::define_grid_size;
use grid::{CellIdx, Grid, GridSize};
use grid::cellset::CellSet;

use std::fmt;

define_grid_size!(Grid6, 6);
define_grid_size!(Grid9, 9);

/// Errors that can arise when reading in a grid from a string representation.
pub enum GridParseError {
    BadLength,
    Contradiction(CellIdx),
}

impl fmt::Display for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GridParseError::*;
        match *self {
            BadLength => write!(f, "The grid does not have the expected length"),
            Contradiction(pos) => write!(f, "The clue at position {} contradicts the others", pos),
        }
    }
}

impl fmt::Debug for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

// Classic Sudoku

pub fn empty_classic() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    Grid::empty(&grid_regions, &vec![CellSet::empty(); 81])
}

pub fn classic_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_classic(), input)
}

pub fn classic_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_classic(), clues)
}

// 6x6 Sudoku

pub fn empty_six_by_six() -> Grid<Grid6> {

    let grid_regions: Vec<CellSet<Grid6>> = (0..6)
            .map(|idx| 12 * (idx / 2) + 3 * (idx % 2))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 6, idx + 7, idx + 8])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    Grid::empty(&grid_regions, &vec![CellSet::empty(); 36])
}

pub fn six_by_six_from_string(input: String) -> Result<Grid<Grid6>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_six_by_six(), input)
}

pub fn six_by_six_from_clues(clues: &[usize]) -> Result<Grid<Grid6>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_six_by_six(), clues)
}

// Diagonal Sudoku

pub fn empty_diagonal() -> Grid<Grid9> {

    let mut grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    grid_regions.push(CellSet::from_cells(vec![0, 10, 20, 30, 40, 50, 60, 70, 80]));
    grid_regions.push(CellSet::from_cells(vec![8, 16, 24, 32, 40, 48, 56, 64, 72]));

    Grid::empty(&grid_regions, &vec![CellSet::empty(); 81])
}

pub fn diagonal_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_diagonal(), input)
}

pub fn diagonal_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_diagonal(), clues)
}

// Antidiagonal Sudoku

pub fn empty_antidiagonal() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let mut additional_neighbours = vec![CellSet::empty(); 81];

    let on_diagonal_1 = vec![0, 10, 20, 30, 40, 50, 60, 70, 80];
    let off_diagonal_1 = vec![1, 2, 9, 11, 18, 19, 31, 32, 39, 41, 48, 49, 61, 62, 69, 71, 78, 79];
    let on_diagonal_2 = vec![8, 16, 24, 32, 40, 48, 56, 64, 72];
    let off_diagonal_2 = vec![6, 7, 15, 17, 25, 26, 30, 31, 39, 41, 49, 50, 54, 55, 63, 65, 73, 74];

    for &on1 in &on_diagonal_1 {
        for &off1 in &off_diagonal_1 {
            additional_neighbours[on1].add_cell(off1);
            additional_neighbours[off1].add_cell(on1);
        }
    }

    for &on2 in &on_diagonal_2 {
        for &off2 in &off_diagonal_2 {
            additional_neighbours[on2].add_cell(off2);
            additional_neighbours[off2].add_cell(on2);
        }
    }

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn antidiagonal_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_antidiagonal(), input)
}

pub fn antidiagonal_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_antidiagonal(), clues)
}

// Windoku

pub fn empty_windoku() -> Grid<Grid9> {

    let mut grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    grid_regions.push(CellSet::from_cells(vec![10, 11, 12, 19, 20, 21, 28, 29, 30]));
    grid_regions.push(CellSet::from_cells(vec![14, 15, 16, 23, 24, 25, 32, 33, 34]));
    grid_regions.push(CellSet::from_cells(vec![46, 47, 48, 55, 56, 57, 64, 65, 66]));
    grid_regions.push(CellSet::from_cells(vec![50, 51, 52, 59, 60, 61, 68, 69, 70]));
    grid_regions.push(CellSet::from_cells(vec![1, 2, 3, 37, 38, 39, 73, 74, 75]));
    grid_regions.push(CellSet::from_cells(vec![5, 6, 7, 41, 42, 43, 77, 78, 79]));
    grid_regions.push(CellSet::from_cells(vec![9, 18, 27, 13, 22, 31, 17, 26, 35]));
    grid_regions.push(CellSet::from_cells(vec![45, 54, 63, 49, 58, 67, 53, 62, 71]));
    grid_regions.push(CellSet::from_cells(vec![0, 4, 8, 36, 40, 44, 72, 76, 80]));

    Grid::empty(&grid_regions, &vec![CellSet::empty(); 81])
}

pub fn windoku_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_windoku(), input)
}

pub fn windoku_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_windoku(), clues)
}

// Antiwindoku

pub fn empty_antiwindoku() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let mut additional_neighbours = vec![CellSet::empty(); 81];

    let on_window_1 = vec![10, 11, 12, 19, 20, 21, 28, 29, 30];
    let on_window_2 = vec![14, 15, 16, 23, 24, 25, 32, 33, 34];
    let on_window_3 = vec![46, 47, 48, 55, 56, 57, 64, 65, 66];
    let on_window_4 = vec![50, 51, 52, 59, 60, 61, 68, 69, 70];

    let off_window_1 = vec![0, 1, 2, 9, 18];
    let off_window_2 = vec![6, 7, 8, 17, 26];
    let off_window_3 = vec![54, 63, 72, 73, 74];
    let off_window_4 = vec![62, 71, 80, 79, 78];

    for &on1 in &on_window_1 {
        for &off1 in &off_window_1 {
            additional_neighbours[on1].add_cell(off1);
            additional_neighbours[off1].add_cell(on1);
        }
    }

    for &on2 in &on_window_2 {
        for &off2 in &off_window_2 {
            additional_neighbours[on2].add_cell(off2);
            additional_neighbours[off2].add_cell(on2);
        }
    }

    for &on3 in &on_window_3 {
        for &off3 in &off_window_3 {
            additional_neighbours[on3].add_cell(off3);
            additional_neighbours[off3].add_cell(on3);
        }
    }

    for &on4 in &on_window_4 {
        for &off4 in &off_window_4 {
            additional_neighbours[on4].add_cell(off4);
            additional_neighbours[off4].add_cell(on4);
        }
    }

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn antiwindoku_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_antiwindoku(), input)
}

pub fn antiwindoku_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_antiwindoku(), clues)
}

// Antiknight Sudoku

pub fn empty_antiknight() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let knight_steps = vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
    let additional_neighbours: Vec<CellSet<Grid9>> = (0..81)
        .map(|cell| (cell / 9, cell % 9))
        .map(|(row, col)| CellSet::from_cells(
            knight_steps.iter()
                .map(|step| (row as i32 + step.0, col as i32 + step.1))
                .filter(|&(r, c)| 0 <= r && r < 9 && 0 <= c && c < 9)
                .map(|(r, c)| (9 * r + c) as usize)
            )
        )
        .collect();

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn antiknight_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_antiknight(), input)
}

pub fn antiknight_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_antiknight(), clues)
}

// Disjoint Groups Sudoku

pub fn empty_disjoint_groups() -> Grid<Grid9> {

    let mut grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    grid_regions.extend(vec![0, 1, 2, 9, 10, 11, 18, 19, 20].into_iter()
        .map(|idx| vec![idx, idx + 3, idx + 6, idx + 27, idx + 30, idx + 33, idx + 54, idx + 57, idx + 60])
        .map(|cells| CellSet::from_cells(cells))
    );

    Grid::empty(&grid_regions, &vec![CellSet::empty(); 81])
}

pub fn disjoint_groups_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_disjoint_groups(), input)
}

pub fn disjoint_groups_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_disjoint_groups(), clues)
}

// Helpers

fn grid_from_empty_grid_and_string<T: GridSize>(empty_grid: &Grid<T>, input: String) -> Result<Grid<T>, GridParseError> {
    if T::size() <= 9 {
        let clues: Vec<usize> = input.bytes().map(|byte| match byte {
            b'1'..=b'9' => (byte - b'0') as usize,
            _ => 0,
        }).collect();
        grid_from_empty_grid_and_clues(empty_grid, &clues)
    } else {
        unimplemented!();
    }
}

fn grid_from_empty_grid_and_clues<T: GridSize>(grid: &Grid<T>, clues: &[usize]) -> Result<Grid<T>, GridParseError> {

    let mut grid = grid.clone();

    if clues.len() != T::size() * T::size() {
        return Err(GridParseError::BadLength);
    }

    for (idx, clue) in clues.iter().enumerate() {
        if *clue > 0 {
            if grid.has_candidate(idx, *clue) {
                grid.place_value(idx, *clue);
            } else {
                return Err(GridParseError::Contradiction(idx));
            }
        }
    }

    Ok(grid)
}