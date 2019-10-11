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

pub fn parse_classic(input: String) -> Result<Grid<Grid9>, GridParseError> {

    if input.len() != 81 {
        return Err(GridParseError::BadLength);
    }

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let clues: Vec<usize> = input.bytes().map(|byte| match byte {
        b'1'..=b'9' => (byte - b'0') as usize,
        _ => 0,
    }).collect();

    let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 81]);
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

pub fn parse_six_by_six(input: String) -> Result<Grid<Grid6>, GridParseError> {

    if input.len() != 36 {
        return Err(GridParseError::BadLength);
    }

    let grid_regions: Vec<CellSet<Grid6>> = (0..6)
            .map(|idx| 12 * (idx / 2) + 3 * (idx % 2))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 6, idx + 7, idx + 8])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let clues: Vec<usize> = input.bytes().map(|byte| match byte {
        b'1'..=b'6' => (byte - b'0') as usize,
        _ => 0,
    }).collect();

    let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 36]);
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

pub fn parse_diagonal(input: String) -> Result<Grid<Grid9>, GridParseError> {

    if input.len() != 81 {
        return Err(GridParseError::BadLength);
    }

    let mut grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    grid_regions.push(CellSet::from_cells(vec![0, 10, 20, 30, 40, 50, 60, 70, 80]));
    grid_regions.push(CellSet::from_cells(vec![8, 16, 24, 32, 40, 48, 56, 64, 72]));

    let clues: Vec<usize> = input.bytes().map(|byte| match byte {
        b'1'..=b'9' => (byte - b'0') as usize,
        _ => 0,
    }).collect();

    let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 81]);
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

pub fn parse_antiknight(input: String) -> Result<Grid<Grid9>, GridParseError> {

    if input.len() != 81 {
        return Err(GridParseError::BadLength);
    }

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

    let clues: Vec<usize> = input.bytes().map(|byte| match byte {
        b'1'..=b'9' => (byte - b'0') as usize,
        _ => 0,
    }).collect();

    let mut grid = Grid::empty(&grid_regions, &additional_neighbours);
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

pub fn parse_disjoint_groups(input: String) -> Result<Grid<Grid9>, GridParseError> {

    if input.len() != 81 {
        return Err(GridParseError::BadLength);
    }

    let mut grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    grid_regions.extend(vec![0, 1, 2, 9, 10, 11, 18, 19, 20].into_iter()
        .map(|idx| vec![idx, idx + 3, idx + 6, idx + 27, idx + 30, idx + 33, idx + 54, idx + 57, idx + 60])
        .map(|cells| CellSet::from_cells(cells))
    );

    let clues: Vec<usize> = input.bytes().map(|byte| match byte {
        b'1'..=b'9' => (byte - b'0') as usize,
        _ => 0,
    }).collect();

    let mut grid = Grid::empty(&grid_regions, &vec![CellSet::empty(); 81]);
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
