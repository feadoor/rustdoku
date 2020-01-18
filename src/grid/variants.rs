//! Specific grids to deal with particular Sudoku variants

use crate::define_grid_size;
use grid::{CellIdx, Grid, GridSize};
use grid::cellset::CellSet;
use grid::placementset::{Placement, PlacementSet};

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

    Grid::empty(&grid_regions, &vec![vec![PlacementSet::empty(); 9]; 81])
}

pub fn classic_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_classic(), input)
}

pub fn classic_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_classic(), clues)
}

// Nonconsecutive Sudoku

pub fn empty_nonconsecutive() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let mut additional_neighbours = vec![vec![PlacementSet::empty(); 9]; 81];

    for cell in 0..81 {

        if cell % 9 != 8 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell + 1, candidate: other });
                    }
                }
            }
        }

        if cell % 9 != 0 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell - 1, candidate: other });
                    }
                }
            }
        }

        if cell / 9 != 0 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell - 9, candidate: other });
                    }
                }
            }
        }

        if cell / 9 != 8 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell + 9, candidate: other });
                    }
                }
            }
        }
    }

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn nonconsecutive_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_nonconsecutive(), input)
}

pub fn nonconsecutive_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_nonconsecutive(), clues)
}

pub fn empty_diagonal_nonconsecutive() -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let mut additional_neighbours = vec![vec![PlacementSet::empty(); 9]; 81];

    for cell in 0..81 {

        if cell % 9 != 8 && cell / 9 != 0 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell - 8, candidate: other });
                    }
                }
            }
        }

        if cell % 9 != 8 && cell / 9 != 8 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell + 10, candidate: other });
                    }
                }
            }
        }

        if cell % 9 != 0 && cell / 9 != 0 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell - 10, candidate: other });
                    }
                }
            }
        }

        if cell % 9 != 0 && cell / 9 != 8 {
            for value in 1..10 {
                for other in 1..10 {
                    if value == other + 1 || value == other - 1 {
                        additional_neighbours[cell][value - 1].add_placement(Placement { cell: cell + 8, candidate: other });
                    }
                }
            }
        }
    }

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn diagonal_nonconsecutive_from_string(input: String) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_diagonal_nonconsecutive(), input)
}

pub fn diagonal_nonconsecutive_from_clues(clues: &[usize]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_diagonal_nonconsecutive(), clues)
}

// Less-than Sudoku

pub fn empty_less_than(inequalities: &[(usize, usize)]) -> Grid<Grid9> {

    let grid_regions: Vec<CellSet<Grid9>> = (0..9)
            .map(|idx| 27 * (idx / 3) + 3 * (idx % 3))
            .map(|idx| vec![idx, idx + 1, idx + 2, idx + 9, idx + 10, idx + 11, idx + 18, idx + 19, idx + 20])
            .map(|cells| CellSet::from_cells(cells))
            .collect();

    let mut additional_neighbours = vec![vec![PlacementSet::empty(); 9]; 81];

    for &(small_cell, big_cell) in inequalities {
        for small_val in 0..9 {
            for big_val in small_val..9 {
                additional_neighbours[small_cell][big_val].add_placement(Placement { cell: big_cell, candidate: small_val + 1 });
                additional_neighbours[big_cell][small_val].add_placement(Placement { cell: small_cell, candidate: big_val + 1 });
            }
        }
    }

    Grid::empty(&grid_regions, &additional_neighbours)
}

pub fn less_than_from_string(input: String, inequalities: &[(usize, usize)]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_string(&empty_less_than(inequalities), input)
}

pub fn less_than_from_clues(clues: &[usize], inequalities: &[(usize, usize)]) -> Result<Grid<Grid9>, GridParseError> {
    grid_from_empty_grid_and_clues(&empty_less_than(inequalities), clues)
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