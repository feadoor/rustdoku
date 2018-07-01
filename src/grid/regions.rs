//! Some utility functions for working with regions of a Sudoku grid.

use grid::{CellIdx, Grid};
use grid::cellset::CellSet;

impl Grid {

    /// The human name for the given cell.
    pub fn cell_name(cell_idx: CellIdx) -> String {
        format!("r{}c{}", (cell_idx / 9) + 1, (cell_idx % 9) + 1)
    }

    /// The human name for the given region.
    pub fn region_name(region: &CellSet) -> String {
        match (region.hi, region.lo) {
            (0x0, 0x1c0e07) => "Block 1".to_string(),
            (0x0, 0xe07038) => "Block 2".to_string(),
            (0x0, 0x70381c0) => "Block 3".to_string(),
            (0x0, 0xe07038000000) => "Block 4".to_string(),
            (0x0, 0x70381c0000000) => "Block 5".to_string(),
            (0x0, 0x381c0e00000000) => "Block 6".to_string(),
            (0x703, 0x81c0000000000000) => "Block 7".to_string(),
            (0x381c, 0x0e00000000000000) => "Block 8".to_string(),
            (0x1c0e0, 0x7000000000000000) => "Block 9".to_string(),
            (0x0, 0x1FF) => "Row 1".to_string(),
            (0x0, 0x3FE00) => "Row 2".to_string(),
            (0x0, 0x7FC0000) => "Row 3".to_string(),
            (0x0, 0xFF8000000) => "Row 4".to_string(),
            (0x0, 0x1FF000000000) => "Row 5".to_string(),
            (0x0, 0x3FE00000000000) => "Row 6".to_string(),
            (0x0, 0x7FC0000000000000) => "Row 7".to_string(),
            (0xFF, 0x8000000000000000) => "Row 8".to_string(),
            (0x1FF00, 0x0) => "Row 9".to_string(),
            (0x100, 0x8040201008040201) => "Column 1".to_string(),
            (0x201, 0x0080402010080402) => "Column 2".to_string(),
            (0x402, 0x0100804020100804) => "Column 3".to_string(),
            (0x804, 0x0201008040201008) => "Column 4".to_string(),
            (0x1008, 0x0402010080402010) => "Column 5".to_string(),
            (0x2010, 0x0804020100804020) => "Column 6".to_string(),
            (0x4020, 0x1008040201008040) => "Column 7".to_string(),
            (0x8040, 0x2010080402010080) => "Column 8".to_string(),
            (0x10080, 0x4020100804020100) => "Column 9".to_string(),
            _ => format!("({})", region.iter().map(Grid::cell_name).collect::<Vec<String>>().join(", ")),
        }
    }

    /// All the values that can be placed in a cell of the grid.
    pub fn values() -> &'static [usize] {
        static VALUES: [usize; 9] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];

        &VALUES
    }

    /// All the cells of the grid.
    pub fn cells() -> &'static CellSet {
        static CELLS: CellSet = CellSet{ hi: 0x1ffff, lo: 0xffffffffffffffff };
        &CELLS
    }

    /// All rows for a grid.
    pub fn rows() -> &'static [CellSet] {
        static ROWSETS: [CellSet; 9] = [
            CellSet{ hi: 0x0, lo: 0x1FF},
            CellSet{ hi: 0x0, lo: 0x3FE00},
            CellSet{ hi: 0x0, lo: 0x7FC0000},
            CellSet{ hi: 0x0, lo: 0xFF8000000},
            CellSet{ hi: 0x0, lo: 0x1FF000000000},
            CellSet{ hi: 0x0, lo: 0x3FE00000000000},
            CellSet{ hi: 0x0, lo: 0x7FC0000000000000},
            CellSet{ hi: 0xFF, lo: 0x8000000000000000},
            CellSet{ hi: 0x1FF00, lo: 0x0},
        ];

        &ROWSETS
    }

    /// All columns for a grid.
    pub fn columns() -> &'static [CellSet] {
        static COLSETS: [CellSet; 9] = [
            CellSet{ hi: 0x100, lo: 0x8040201008040201},
            CellSet{ hi: 0x201, lo: 0x0080402010080402},
            CellSet{ hi: 0x402, lo: 0x0100804020100804},
            CellSet{ hi: 0x804, lo: 0x0201008040201008},
            CellSet{ hi: 0x1008, lo: 0x0402010080402010},
            CellSet{ hi: 0x2010, lo: 0x0804020100804020},
            CellSet{ hi: 0x4020, lo: 0x1008040201008040},
            CellSet{ hi: 0x8040, lo: 0x2010080402010080},
            CellSet{ hi: 0x10080, lo: 0x4020100804020100},
        ];

        &COLSETS
    }

    /// All blocks for a grid.
    pub fn blocks() -> &'static [CellSet] {
        static BLOCKSETS: [CellSet; 9] = [
            CellSet{ hi: 0x0, lo: 0x1c0e07},
            CellSet{ hi: 0x0, lo: 0xe07038},
            CellSet{ hi: 0x0, lo: 0x70381c0},
            CellSet{ hi: 0x0, lo: 0xe07038000000},
            CellSet{ hi: 0x0, lo: 0x70381c0000000},
            CellSet{ hi: 0x0, lo: 0x381c0e00000000},
            CellSet{ hi: 0x703, lo: 0x81c0000000000000},
            CellSet{ hi: 0x381c, lo: 0x0e00000000000000},
            CellSet{ hi: 0x1c0e0, lo: 0x7000000000000000},
        ];

        &BLOCKSETS
    }

    /// All regions for a grid.
    pub fn regions() -> &'static [CellSet] {
        static REGIONS: [CellSet; 27] = [
            CellSet{ hi: 0x0, lo: 0x1c0e07},
            CellSet{ hi: 0x0, lo: 0xe07038},
            CellSet{ hi: 0x0, lo: 0x70381c0},
            CellSet{ hi: 0x0, lo: 0xe07038000000},
            CellSet{ hi: 0x0, lo: 0x70381c0000000},
            CellSet{ hi: 0x0, lo: 0x381c0e00000000},
            CellSet{ hi: 0x703, lo: 0x81c0000000000000},
            CellSet{ hi: 0x381c, lo: 0x0e00000000000000},
            CellSet{ hi: 0x1c0e0, lo: 0x7000000000000000},
            CellSet{ hi: 0x0, lo: 0x1FF},
            CellSet{ hi: 0x0, lo: 0x3FE00},
            CellSet{ hi: 0x0, lo: 0x7FC0000},
            CellSet{ hi: 0x0, lo: 0xFF8000000},
            CellSet{ hi: 0x0, lo: 0x1FF000000000},
            CellSet{ hi: 0x0, lo: 0x3FE00000000000},
            CellSet{ hi: 0x0, lo: 0x7FC0000000000000},
            CellSet{ hi: 0xFF, lo: 0x8000000000000000},
            CellSet{ hi: 0x1FF00, lo: 0x0},
            CellSet{ hi: 0x100, lo: 0x8040201008040201},
            CellSet{ hi: 0x201, lo: 0x0080402010080402},
            CellSet{ hi: 0x402, lo: 0x0100804020100804},
            CellSet{ hi: 0x804, lo: 0x0201008040201008},
            CellSet{ hi: 0x1008, lo: 0x0402010080402010},
            CellSet{ hi: 0x2010, lo: 0x0804020100804020},
            CellSet{ hi: 0x4020, lo: 0x1008040201008040},
            CellSet{ hi: 0x8040, lo: 0x2010080402010080},
            CellSet{ hi: 0x10080, lo: 0x4020100804020100},
        ];

        &REGIONS
    }

    /// The set of neighbours for a particular cell.
    pub fn neighbours(cell_idx: CellIdx) -> &'static CellSet {
        static NEIGHBOURS_SETS: [CellSet; 81] = [
            CellSet{ hi: 0x100, lo: 0x80402010081c0ffe},
            CellSet{ hi: 0x201, lo: 0x804020101c0ffd},
            CellSet{ hi: 0x402, lo: 0x1008040201c0ffb},
            CellSet{ hi: 0x804, lo: 0x201008040e071f7},
            CellSet{ hi: 0x1008, lo: 0x402010080e071ef},
            CellSet{ hi: 0x2010, lo: 0x804020100e071df},
            CellSet{ hi: 0x4020, lo: 0x10080402070381bf},
            CellSet{ hi: 0x8040, lo: 0x201008040703817f},
            CellSet{ hi: 0x10080, lo: 0x40201008070380ff},
            CellSet{ hi: 0x100, lo: 0x80402010081ffc07},
            CellSet{ hi: 0x201, lo: 0x804020101ffa07},
            CellSet{ hi: 0x402, lo: 0x1008040201ff607},
            CellSet{ hi: 0x804, lo: 0x201008040e3ee38},
            CellSet{ hi: 0x1008, lo: 0x402010080e3de38},
            CellSet{ hi: 0x2010, lo: 0x804020100e3be38},
            CellSet{ hi: 0x4020, lo: 0x1008040207037fc0},
            CellSet{ hi: 0x8040, lo: 0x201008040702ffc0},
            CellSet{ hi: 0x10080, lo: 0x402010080701ffc0},
            CellSet{ hi: 0x100, lo: 0x804020100ff80e07},
            CellSet{ hi: 0x201, lo: 0x80402017f40e07},
            CellSet{ hi: 0x402, lo: 0x100804027ec0e07},
            CellSet{ hi: 0x804, lo: 0x201008047dc7038},
            CellSet{ hi: 0x1008, lo: 0x402010087bc7038},
            CellSet{ hi: 0x2010, lo: 0x8040201077c7038},
            CellSet{ hi: 0x4020, lo: 0x1008040206ff81c0},
            CellSet{ hi: 0x8040, lo: 0x2010080405ff81c0},
            CellSet{ hi: 0x10080, lo: 0x4020100803ff81c0},
            CellSet{ hi: 0x100, lo: 0x8040e07ff0040201},
            CellSet{ hi: 0x201, lo: 0x80e07fe8080402},
            CellSet{ hi: 0x402, lo: 0x100e07fd8100804},
            CellSet{ hi: 0x804, lo: 0x207038fb8201008},
            CellSet{ hi: 0x1008, lo: 0x407038f78402010},
            CellSet{ hi: 0x2010, lo: 0x807038ef8804020},
            CellSet{ hi: 0x4020, lo: 0x10381c0df9008040},
            CellSet{ hi: 0x8040, lo: 0x20381c0bfa010080},
            CellSet{ hi: 0x10080, lo: 0x40381c07fc020100},
            CellSet{ hi: 0x100, lo: 0x8040ffe038040201},
            CellSet{ hi: 0x201, lo: 0x80ffd038080402},
            CellSet{ hi: 0x402, lo: 0x100ffb038100804},
            CellSet{ hi: 0x804, lo: 0x2071f71c0201008},
            CellSet{ hi: 0x1008, lo: 0x4071ef1c0402010},
            CellSet{ hi: 0x2010, lo: 0x8071df1c0804020},
            CellSet{ hi: 0x4020, lo: 0x10381bfe01008040},
            CellSet{ hi: 0x8040, lo: 0x203817fe02010080},
            CellSet{ hi: 0x10080, lo: 0x40380ffe04020100},
            CellSet{ hi: 0x100, lo: 0x807fc07038040201},
            CellSet{ hi: 0x201, lo: 0xbfa07038080402},
            CellSet{ hi: 0x402, lo: 0x13f607038100804},
            CellSet{ hi: 0x804, lo: 0x23ee381c0201008},
            CellSet{ hi: 0x1008, lo: 0x43de381c0402010},
            CellSet{ hi: 0x2010, lo: 0x83be381c0804020},
            CellSet{ hi: 0x4020, lo: 0x1037fc0e01008040},
            CellSet{ hi: 0x8040, lo: 0x202ffc0e02010080},
            CellSet{ hi: 0x10080, lo: 0x401ffc0e04020100},
            CellSet{ hi: 0x703, lo: 0xff80201008040201},
            CellSet{ hi: 0x703, lo: 0xff40402010080402},
            CellSet{ hi: 0x703, lo: 0xfec0804020100804},
            CellSet{ hi: 0x381c, lo: 0x7dc1008040201008},
            CellSet{ hi: 0x381c, lo: 0x7bc2010080402010},
            CellSet{ hi: 0x381c, lo: 0x77c4020100804020},
            CellSet{ hi: 0x1c0e0, lo: 0x6fc8040201008040},
            CellSet{ hi: 0x1c0e0, lo: 0x5fd0080402010080},
            CellSet{ hi: 0x1c0e0, lo: 0x3fe0100804020100},
            CellSet{ hi: 0x7ff, lo: 0x1c0201008040201},
            CellSet{ hi: 0x7fe, lo: 0x81c0402010080402},
            CellSet{ hi: 0x7fd, lo: 0x81c0804020100804},
            CellSet{ hi: 0x38fb, lo: 0x8e01008040201008},
            CellSet{ hi: 0x38f7, lo: 0x8e02010080402010},
            CellSet{ hi: 0x38ef, lo: 0x8e04020100804020},
            CellSet{ hi: 0x1c0df, lo: 0xf008040201008040},
            CellSet{ hi: 0x1c0bf, lo: 0xf010080402010080},
            CellSet{ hi: 0x1c07f, lo: 0xf020100804020100},
            CellSet{ hi: 0x1fe03, lo: 0x81c0201008040201},
            CellSet{ hi: 0x1fd03, lo: 0x81c0402010080402},
            CellSet{ hi: 0x1fb03, lo: 0x81c0804020100804},
            CellSet{ hi: 0x1f71c, lo: 0xe01008040201008},
            CellSet{ hi: 0x1ef1c, lo: 0xe02010080402010},
            CellSet{ hi: 0x1df1c, lo: 0xe04020100804020},
            CellSet{ hi: 0x1bfe0, lo: 0x7008040201008040},
            CellSet{ hi: 0x17fe0, lo: 0x7010080402010080},
            CellSet{ hi: 0xffe0, lo: 0x7020100804020100},
        ];

        &NEIGHBOURS_SETS[cell_idx]
    }

    /// Return the row which contains all of the given cells.
    pub fn row_containing(cells: &CellSet) -> Option<&'static CellSet> {
        for row in Grid::rows() {
            if row & cells == * cells { return Some(row) }
        }
        None
    }

    /// Return the column which contains all of the given cells.
    pub fn column_containing(cells: &CellSet) -> Option<&'static CellSet> {
        for column in Grid::columns() {
            if column & cells == * cells { return Some(column) }
        }
        None
    }

    /// Return the block which contains all of the given cells.
    pub fn block_containing(cells: &CellSet) -> Option<&'static CellSet> {
        for block in Grid::blocks() {
            if block & cells == * cells { return Some(block) }
        }
        None
    }

    /// Get the rows which intersect the given `CellSet`.
    pub fn intersecting_rows(cells: &CellSet) -> Vec<&'static CellSet> {
        Grid::rows().iter().filter(|&row| !((row & cells).is_empty())).collect()
    }

    /// Get the columns which intersect the given `CellSet`.
    pub fn intersecting_columns(cells: &CellSet) -> Vec<&'static CellSet> {
        Grid::columns().iter().filter(|&column| !((column & cells).is_empty())).collect()
    }
}
