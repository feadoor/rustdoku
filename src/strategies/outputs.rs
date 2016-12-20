//! Obtain short, text descriptions of the various logical manoeuvres.

use std::fmt;
use std::fmt::Write;

use grid::{CellIdx, Grid};
use grid::cellset::CellSet;
use strategies::Reason;

/// A structure containing all the information needed to define a Full House.
pub struct FullHouse {
    pub cell: CellIdx,
    pub region: CellSet,
}

/// A structure containing all the information needed to define a Hidden Single.
pub struct HiddenSingle {
    pub cell: CellIdx,
    pub region: CellSet,
    pub value: usize,
}

/// A structure containing all the information needed to define a Naked Single.
pub struct NakedSingle {
    pub cell: CellIdx,
}

/// A structure containing all the information needed to define a Pointing.
pub struct Pointing {
    pub block: CellSet,
    pub value: usize,
    pub region: CellSet,
}

/// A structure containing all the information needed to define a Claiming.
pub struct Claiming {
    pub region: CellSet,
    pub value: usize,
    pub block: CellSet,
}

/// A structure containing all the information needed to define a Naked Subset.
pub struct NakedSubset {
    pub cells: Vec<CellIdx>,
    pub candidates: usize,
}

/// A structure containing all the information needed to define a Hidden Subset.
pub struct HiddenSubset {
    pub cells: CellSet,
    pub candidates: Vec<usize>,
}

/// A structure containing all the information needed to define a Basic Fish.
pub struct BasicFish {
    pub base: Vec<CellSet>,
    pub value: usize,
    pub eliminations: CellSet,
    pub finned: bool,
    pub rows: bool,
}

/// A structure containing all the information needed to define an XY-Wing.
pub struct XYWing {
    pub pivot: CellIdx,
    pub pincer1: CellIdx,
    pub pincer2: CellIdx,
    pub value: usize
}

/// A structure containing all the information needed to define an XYZ-Wing.
pub struct XYZWing {
    pub pivot: CellIdx,
    pub pincer1: CellIdx,
    pub pincer2: CellIdx,
    pub value: usize
}

impl Reason for FullHouse {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Work out what sort of region was in use for the deduction.
        let region = if Grid::same_row(&self.region) {
            "row"
        } else if Grid::same_column(&self.region) {
            "column"
        } else if Grid::same_block(&self.region) {
            "block"
        } else {
            "ERROR"
        };

        // Work out the row and column that the cell belongs to.
        let row_idx = Grid::row_idx(self.cell) + 1;
        let col_idx = Grid::column_idx(self.cell) + 1;

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Full house r{}c{} - last cell in {}.",
                                      row_idx, col_idx, region));

        Ok(description)
    }
}

impl Reason for HiddenSingle {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Work out what sort of region was in use for the deduction.
        let region = if Grid::same_row(&self.region) {
            "row"
        } else if Grid::same_column(&self.region) {
            "column"
        } else if Grid::same_block(&self.region) {
            "block"
        } else {
            "ERROR"
        };

        // Work out the row and column that the cell belongs to.
        let row_idx = Grid::row_idx(self.cell) + 1;
        let col_idx = Grid::column_idx(self.cell) + 1;

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Hidden single r{}c{} - only place for {} in {}.",
                                      row_idx, col_idx, self.value, region));

        Ok(description)
    }
}

impl Reason for NakedSingle {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Work out the row and column that the cell belongs to.
        let row_idx = Grid::row_idx(self.cell) + 1;
        let col_idx = Grid::column_idx(self.cell) + 1;

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Naked single r{}c{}.", row_idx, col_idx));

        Ok(description)
    }
}

impl Reason for Pointing {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Work out the secondary region type.
        let region_type = if Grid::same_row(&self.region) {
            "row"
        } else if Grid::same_column(&self.region) {
            "column"
        } else {
            "ERROR"
        };

        // Work out the index of the row or column under consideration.
        let region_idx = if region_type == "row" {
            Grid::row_idx(self.region.first().unwrap()) + 1
        } else if region_type == "column" {
            Grid::column_idx(self.region.first().unwrap()) + 1
        } else {
            0
        };

        // Write the description.
        let mut description = String::new();
        let block_idx = Grid::block_idx(self.block.first().unwrap()) + 1;
        try!(write!(&mut description, "Pointing in block {} - value {} removed from rest of {} {}.",
                                      block_idx, self.value, region_type, region_idx));

        Ok(description)
    }
}

impl Reason for Claiming {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Work out the region type.
        let region_type = if Grid::same_row(&self.region) {
            "row"
        } else if Grid::same_column(&self.region) {
            "column"
        } else {
            "ERROR"
        };

        // Work out the index of the row or column under consideration.
        let region_idx = if region_type == "row" {
            Grid::row_idx(self.region.first().unwrap()) + 1
        } else if region_type == "column" {
            Grid::column_idx(self.region.first().unwrap()) + 1
        } else {
            0
        };

        // Work out the index of the block under consideration.
        let block_idx = Grid::block_idx(self.block.first().unwrap()) + 1;

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Claiming in {} {} - value {} removed from rest of block {}.",
                                      region_type, region_idx, self.value, block_idx));

        Ok(description)
    }
}

impl Reason for NakedSubset {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Get the row/column description of each cell.
        let coords: Vec<String> = self.cells.iter()
            .map(|&ix| format!("r{}c{}", Grid::row_idx(ix) + 1, Grid::column_idx(ix) + 1)).collect();

        // Get the name of the subset (pair, triple or quad).
        let name = match self.cells.len() {
            2 => "pair",
            3 => "triple",
            4 => "quad",
            _ => "subset",
        };

        // Get the values that we're eliminating.
        let mut vals = Vec::new();
        let mut cands = self.candidates;
        while cands != 0 {
            vals.push(cands.trailing_zeros() as usize + 1);
            cands &= cands - 1;
        }

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Naked {} in cells {:?} - eliminate values {:?} from common neighbours.",
                                      name, coords, vals));

        Ok(description)
    }
}

impl Reason for HiddenSubset {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Get the row/column description of each cell.
        let coords: Vec<String> = self.cells.iter()
            .map(|ix| format!("r{}c{}", Grid::row_idx(ix) + 1, Grid::column_idx(ix) + 1)).collect();

        // Get the name of the subset (pair, triple or quad).
        let name = match self.cells.len() {
            2 => "pair",
            3 => "triple",
            4 => "quad",
            _ => "subset",
        };

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "Hidden {} in cells {:?} with values {:?} - eliminate other values from these cells.",
                                      name, coords, self.candidates));

        Ok(description)
    }
}

impl Reason for BasicFish {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Get the name of the fish variant.
        let name = match self.base.len() {
            2 => "X-Wing",
            3 => "Swordfish",
            4 => "Jellyfish",
            _ => "Fish",
        };

        // Get the name of the base region type.
        let region_type = if self.rows { "rows" } else { "columns" };

        // Get the indices of the base regions.
        let indices: Vec<_> = if self.rows {
            self.base.iter().map(|x| Grid::row_idx(x.first().unwrap()) + 1).collect()
        } else {
            self.base.iter().map(|x| Grid::column_idx(x.first().unwrap()) + 1).collect()
        };

        // Get the coordinates of the cells that eliminations occur in.
        let coords: Vec<_> = self.eliminations.iter()
            .map(|ix| format!("r{}c{}", Grid::row_idx(ix) + 1, Grid::column_idx(ix) + 1))
            .collect();

        // Write the description.
        let mut description = String::new();
        if !self.finned {
            try!(write!(&mut description, "{} on value {} in {} {:?} - eliminations from {:?}.",
                                          name, self.value, region_type, indices, coords));
        } else {
            try!(write!(&mut description, "Finned {} on value {} in {} {:?} - eliminations from {:?}.",
                                          name, self.value, region_type, indices, coords));
        }

        Ok(description)
    }
}

impl Reason for XYWing {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Get coordinates for each of the cells involved.
        let coords = |idx| {
            format!("r{}c{}", Grid::row_idx(idx) + 1, Grid::column_idx(idx) + 1)
        };

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "XY-Wing with pivot {} and pincers {}, {} - eliminate {} from cells that can see both pincers.",
                                      coords(self.pivot), coords(self.pincer1), coords(self.pincer2), self.value));

        Ok(description)
    }
}

impl Reason for XYZWing {
    /// Get a description of the logic behind the deductions.
    fn description(&self) -> Result<String, fmt::Error> {

        // Get coordinates for each of the cells involved.
        let coords = |idx| {
            format!("r{}c{}", Grid::row_idx(idx) + 1, Grid::column_idx(idx) + 1)
        };

        // Write the description.
        let mut description = String::new();
        try!(write!(&mut description, "XYZ-Wing with pivot {} and pincers {}, {} - eliminate {} from cells that can see all three.",
                                      coords(self.pivot), coords(self.pincer1), coords(self.pincer2), self.value));

        Ok(description)
    }
}
