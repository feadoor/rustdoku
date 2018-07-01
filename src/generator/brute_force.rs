//! A (hopefully!) very quick brute-force solver, used to aid in grid generation.

type Cell = usize;
type House = usize;
type DigitMask = usize;

const DIGITS: usize = 9;
const NEIGHBOURS: usize = 20;
const HOUSES: usize = 27;
const CELLS: usize = 81;

const MASKS: usize = 512;
const ALL_DIGITS: DigitMask = 0x1FF;
const NO_DIGITS: DigitMask = 0;

const ROW_FOR_CELL: &'static [House; CELLS] = &[
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

const COL_FOR_CELL: &'static [House; CELLS] = &[
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
    9, 10, 11, 12, 13, 14, 15, 16, 17,
];

const BLOCK_FOR_CELL: &'static [House; CELLS] = &[
    18, 18, 18, 19, 19, 19, 20, 20, 20,
    18, 18, 18, 19, 19, 19, 20, 20, 20,
    18, 18, 18, 19, 19, 19, 20, 20, 20,
    21, 21, 21, 22, 22, 22, 23, 23, 23,
    21, 21, 21, 22, 22, 22, 23, 23, 23,
    21, 21, 21, 22, 22, 22, 23, 23, 23,
    24, 24, 24, 25, 25, 25, 26, 26, 26,
    24, 24, 24, 25, 25, 25, 26, 26, 26,
    24, 24, 24, 25, 25, 25, 26, 26, 26,
];

const CELLS_FOR_HOUSE: &'static[&'static[Cell; DIGITS]; HOUSES] = &[
    &[ 0,  1,  2,  3,  4,  5,  6,  7,  8],
    &[ 9, 10, 11, 12, 13, 14, 15, 16, 17],
    &[18, 19, 20, 21, 22, 23, 24, 25, 26],
    &[27, 28, 29, 30, 31, 32, 33, 34, 35],
    &[36, 37, 38, 39, 40, 41, 42, 43, 44],
    &[45, 46, 47, 48, 49, 50, 51, 52, 53],
    &[54, 55, 56, 57, 58, 59, 60, 61, 62],
    &[63, 64, 65, 66, 67, 68, 69, 70, 71],
    &[72, 73, 74, 75, 76, 77, 78, 79, 80],
    &[ 0,  9, 18, 27, 36, 45, 54, 63, 72],
    &[ 1, 10, 19, 28, 37, 46, 55, 64, 73],
    &[ 2, 11, 20, 29, 38, 47, 56, 65, 74],
    &[ 3, 12, 21, 30, 39, 48, 57, 66, 75],
    &[ 4, 13, 22, 31, 40, 49, 58, 67, 76],
    &[ 5, 14, 23, 32, 41, 50, 59, 68, 77],
    &[ 6, 15, 24, 33, 42, 51, 60, 69, 78],
    &[ 7, 16, 25, 34, 43, 52, 61, 70, 79],
    &[ 8, 17, 26, 35, 44, 53, 62, 71, 80],
    &[ 0,  1,  2,  9, 10, 11, 18, 19, 20],
    &[ 3,  4,  5, 12, 13, 14, 21, 22, 23],
    &[ 6,  7,  8, 15, 16, 17, 24, 25, 26],
    &[27, 28, 29, 36, 37, 38, 45, 46, 47],
    &[30, 31, 32, 39, 40, 41, 48, 49, 50],
    &[33, 34, 35, 42, 43, 44, 51, 52, 53],
    &[54, 55, 56, 63, 64, 65, 72, 73, 74],
    &[57, 58, 59, 66, 67, 68, 75, 76, 77],
    &[60, 61, 62, 69, 70, 71, 78, 79, 80],
];

const MASK_FOR_DIGIT: &'static[DigitMask; DIGITS + 1] = &[
    0, 1, 2, 4, 8, 16, 32, 64, 128, 256,
];

const DIGITS_IN_MASK: &'static[usize; MASKS] = &[
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
    4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, 5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9,
];

const UNIQ_DIGIT_IN_MASK: &'static[usize; MASKS] = &[
    0, 1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const NEIGHBOURS_FOR_CELL: &'static[&'static[Cell; NEIGHBOURS]; CELLS] = &[
    &[9, 18, 27, 36, 45, 54, 63, 72, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 19, 20],
    &[10, 19, 28, 37, 46, 55, 64, 73, 0, 2, 3, 4, 5, 6, 7, 8, 9, 11, 18, 20],
    &[11, 20, 29, 38, 47, 56, 65, 74, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 18, 19],
    &[12, 21, 30, 39, 48, 57, 66, 75, 0, 1, 2, 4, 5, 6, 7, 8, 13, 14, 22, 23],
    &[13, 22, 31, 40, 49, 58, 67, 76, 0, 1, 2, 3, 5, 6, 7, 8, 12, 14, 21, 23],
    &[14, 23, 32, 41, 50, 59, 68, 77, 0, 1, 2, 3, 4, 6, 7, 8, 12, 13, 21, 22],
    &[15, 24, 33, 42, 51, 60, 69, 78, 0, 1, 2, 3, 4, 5, 7, 8, 16, 17, 25, 26],
    &[16, 25, 34, 43, 52, 61, 70, 79, 0, 1, 2, 3, 4, 5, 6, 8, 15, 17, 24, 26],
    &[17, 26, 35, 44, 53, 62, 71, 80, 0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 24, 25],
    &[0, 18, 27, 36, 45, 54, 63, 72, 10, 11, 12, 13, 14, 15, 16, 17, 1, 2, 19, 20],
    &[1, 19, 28, 37, 46, 55, 64, 73, 9, 11, 12, 13, 14, 15, 16, 17, 0, 2, 18, 20],
    &[2, 20, 29, 38, 47, 56, 65, 74, 9, 10, 12, 13, 14, 15, 16, 17, 0, 1, 18, 19],
    &[3, 21, 30, 39, 48, 57, 66, 75, 9, 10, 11, 13, 14, 15, 16, 17, 4, 5, 22, 23],
    &[4, 22, 31, 40, 49, 58, 67, 76, 9, 10, 11, 12, 14, 15, 16, 17, 3, 5, 21, 23],
    &[5, 23, 32, 41, 50, 59, 68, 77, 9, 10, 11, 12, 13, 15, 16, 17, 3, 4, 21, 22],
    &[6, 24, 33, 42, 51, 60, 69, 78, 9, 10, 11, 12, 13, 14, 16, 17, 7, 8, 25, 26],
    &[7, 25, 34, 43, 52, 61, 70, 79, 9, 10, 11, 12, 13, 14, 15, 17, 6, 8, 24, 26],
    &[8, 26, 35, 44, 53, 62, 71, 80, 9, 10, 11, 12, 13, 14, 15, 16, 6, 7, 24, 25],
    &[0, 9, 27, 36, 45, 54, 63, 72, 19, 20, 21, 22, 23, 24, 25, 26, 1, 2, 10, 11],
    &[1, 10, 28, 37, 46, 55, 64, 73, 18, 20, 21, 22, 23, 24, 25, 26, 0, 2, 9, 11],
    &[2, 11, 29, 38, 47, 56, 65, 74, 18, 19, 21, 22, 23, 24, 25, 26, 0, 1, 9, 10],
    &[3, 12, 30, 39, 48, 57, 66, 75, 18, 19, 20, 22, 23, 24, 25, 26, 4, 5, 13, 14],
    &[4, 13, 31, 40, 49, 58, 67, 76, 18, 19, 20, 21, 23, 24, 25, 26, 3, 5, 12, 14],
    &[5, 14, 32, 41, 50, 59, 68, 77, 18, 19, 20, 21, 22, 24, 25, 26, 3, 4, 12, 13],
    &[6, 15, 33, 42, 51, 60, 69, 78, 18, 19, 20, 21, 22, 23, 25, 26, 7, 8, 16, 17],
    &[7, 16, 34, 43, 52, 61, 70, 79, 18, 19, 20, 21, 22, 23, 24, 26, 6, 8, 15, 17],
    &[8, 17, 35, 44, 53, 62, 71, 80, 18, 19, 20, 21, 22, 23, 24, 25, 6, 7, 15, 16],
    &[0, 9, 18, 36, 45, 54, 63, 72, 28, 29, 30, 31, 32, 33, 34, 35, 37, 38, 46, 47],
    &[1, 10, 19, 37, 46, 55, 64, 73, 27, 29, 30, 31, 32, 33, 34, 35, 36, 38, 45, 47],
    &[2, 11, 20, 38, 47, 56, 65, 74, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 45, 46],
    &[3, 12, 21, 39, 48, 57, 66, 75, 27, 28, 29, 31, 32, 33, 34, 35, 40, 41, 49, 50],
    &[4, 13, 22, 40, 49, 58, 67, 76, 27, 28, 29, 30, 32, 33, 34, 35, 39, 41, 48, 50],
    &[5, 14, 23, 41, 50, 59, 68, 77, 27, 28, 29, 30, 31, 33, 34, 35, 39, 40, 48, 49],
    &[6, 15, 24, 42, 51, 60, 69, 78, 27, 28, 29, 30, 31, 32, 34, 35, 43, 44, 52, 53],
    &[7, 16, 25, 43, 52, 61, 70, 79, 27, 28, 29, 30, 31, 32, 33, 35, 42, 44, 51, 53],
    &[8, 17, 26, 44, 53, 62, 71, 80, 27, 28, 29, 30, 31, 32, 33, 34, 42, 43, 51, 52],
    &[0, 9, 18, 27, 45, 54, 63, 72, 37, 38, 39, 40, 41, 42, 43, 44, 28, 29, 46, 47],
    &[1, 10, 19, 28, 46, 55, 64, 73, 36, 38, 39, 40, 41, 42, 43, 44, 27, 29, 45, 47],
    &[2, 11, 20, 29, 47, 56, 65, 74, 36, 37, 39, 40, 41, 42, 43, 44, 27, 28, 45, 46],
    &[3, 12, 21, 30, 48, 57, 66, 75, 36, 37, 38, 40, 41, 42, 43, 44, 31, 32, 49, 50],
    &[4, 13, 22, 31, 49, 58, 67, 76, 36, 37, 38, 39, 41, 42, 43, 44, 30, 32, 48, 50],
    &[5, 14, 23, 32, 50, 59, 68, 77, 36, 37, 38, 39, 40, 42, 43, 44, 30, 31, 48, 49],
    &[6, 15, 24, 33, 51, 60, 69, 78, 36, 37, 38, 39, 40, 41, 43, 44, 34, 35, 52, 53],
    &[7, 16, 25, 34, 52, 61, 70, 79, 36, 37, 38, 39, 40, 41, 42, 44, 33, 35, 51, 53],
    &[8, 17, 26, 35, 53, 62, 71, 80, 36, 37, 38, 39, 40, 41, 42, 43, 33, 34, 51, 52],
    &[0, 9, 18, 27, 36, 54, 63, 72, 46, 47, 48, 49, 50, 51, 52, 53, 28, 29, 37, 38],
    &[1, 10, 19, 28, 37, 55, 64, 73, 45, 47, 48, 49, 50, 51, 52, 53, 27, 29, 36, 38],
    &[2, 11, 20, 29, 38, 56, 65, 74, 45, 46, 48, 49, 50, 51, 52, 53, 27, 28, 36, 37],
    &[3, 12, 21, 30, 39, 57, 66, 75, 45, 46, 47, 49, 50, 51, 52, 53, 31, 32, 40, 41],
    &[4, 13, 22, 31, 40, 58, 67, 76, 45, 46, 47, 48, 50, 51, 52, 53, 30, 32, 39, 41],
    &[5, 14, 23, 32, 41, 59, 68, 77, 45, 46, 47, 48, 49, 51, 52, 53, 30, 31, 39, 40],
    &[6, 15, 24, 33, 42, 60, 69, 78, 45, 46, 47, 48, 49, 50, 52, 53, 34, 35, 43, 44],
    &[7, 16, 25, 34, 43, 61, 70, 79, 45, 46, 47, 48, 49, 50, 51, 53, 33, 35, 42, 44],
    &[8, 17, 26, 35, 44, 62, 71, 80, 45, 46, 47, 48, 49, 50, 51, 52, 33, 34, 42, 43],
    &[0, 9, 18, 27, 36, 45, 63, 72, 55, 56, 57, 58, 59, 60, 61, 62, 64, 65, 73, 74],
    &[1, 10, 19, 28, 37, 46, 64, 73, 54, 56, 57, 58, 59, 60, 61, 62, 63, 65, 72, 74],
    &[2, 11, 20, 29, 38, 47, 65, 74, 54, 55, 57, 58, 59, 60, 61, 62, 63, 64, 72, 73],
    &[3, 12, 21, 30, 39, 48, 66, 75, 54, 55, 56, 58, 59, 60, 61, 62, 67, 68, 76, 77],
    &[4, 13, 22, 31, 40, 49, 67, 76, 54, 55, 56, 57, 59, 60, 61, 62, 66, 68, 75, 77],
    &[5, 14, 23, 32, 41, 50, 68, 77, 54, 55, 56, 57, 58, 60, 61, 62, 66, 67, 75, 76],
    &[6, 15, 24, 33, 42, 51, 69, 78, 54, 55, 56, 57, 58, 59, 61, 62, 70, 71, 79, 80],
    &[7, 16, 25, 34, 43, 52, 70, 79, 54, 55, 56, 57, 58, 59, 60, 62, 69, 71, 78, 80],
    &[8, 17, 26, 35, 44, 53, 71, 80, 54, 55, 56, 57, 58, 59, 60, 61, 69, 70, 78, 79],
    &[0, 9, 18, 27, 36, 45, 54, 72, 64, 65, 66, 67, 68, 69, 70, 71, 55, 56, 73, 74],
    &[1, 10, 19, 28, 37, 46, 55, 73, 63, 65, 66, 67, 68, 69, 70, 71, 54, 56, 72, 74],
    &[2, 11, 20, 29, 38, 47, 56, 74, 63, 64, 66, 67, 68, 69, 70, 71, 54, 55, 72, 73],
    &[3, 12, 21, 30, 39, 48, 57, 75, 63, 64, 65, 67, 68, 69, 70, 71, 58, 59, 76, 77],
    &[4, 13, 22, 31, 40, 49, 58, 76, 63, 64, 65, 66, 68, 69, 70, 71, 57, 59, 75, 77],
    &[5, 14, 23, 32, 41, 50, 59, 77, 63, 64, 65, 66, 67, 69, 70, 71, 57, 58, 75, 76],
    &[6, 15, 24, 33, 42, 51, 60, 78, 63, 64, 65, 66, 67, 68, 70, 71, 61, 62, 79, 80],
    &[7, 16, 25, 34, 43, 52, 61, 79, 63, 64, 65, 66, 67, 68, 69, 71, 60, 62, 78, 80],
    &[8, 17, 26, 35, 44, 53, 62, 80, 63, 64, 65, 66, 67, 68, 69, 70, 60, 61, 78, 79],
    &[0, 9, 18, 27, 36, 45, 54, 63, 73, 74, 75, 76, 77, 78, 79, 80, 55, 56, 64, 65],
    &[1, 10, 19, 28, 37, 46, 55, 64, 72, 74, 75, 76, 77, 78, 79, 80, 54, 56, 63, 65],
    &[2, 11, 20, 29, 38, 47, 56, 65, 72, 73, 75, 76, 77, 78, 79, 80, 54, 55, 63, 64],
    &[3, 12, 21, 30, 39, 48, 57, 66, 72, 73, 74, 76, 77, 78, 79, 80, 58, 59, 67, 68],
    &[4, 13, 22, 31, 40, 49, 58, 67, 72, 73, 74, 75, 77, 78, 79, 80, 57, 59, 66, 68],
    &[5, 14, 23, 32, 41, 50, 59, 68, 72, 73, 74, 75, 76, 78, 79, 80, 57, 58, 66, 67],
    &[6, 15, 24, 33, 42, 51, 60, 69, 72, 73, 74, 75, 76, 77, 79, 80, 61, 62, 70, 71],
    &[7, 16, 25, 34, 43, 52, 61, 70, 72, 73, 74, 75, 76, 77, 78, 80, 60, 62, 69, 71],
    &[8, 17, 26, 35, 44, 53, 62, 71, 72, 73, 74, 75, 76, 77, 78, 79, 60, 61, 69, 70],
];

pub struct BruteForceResult {
    pub solution_count: usize,
    pub solution: Option<Vec<usize>>,
}

pub fn count_solutions(clues: &[usize], max_solutions: usize) -> BruteForceResult {
    let mut solver = BruteForceSolver::init_from_clues(clues);
    solver.count_solutions(max_solutions)
}

#[derive(Clone)]
struct BoardState {
    cells: [DigitMask; CELLS],
    cells_remaining: usize,
    solved_in_house: [DigitMask; HOUSES],
    solution: [usize; CELLS],
}

impl BoardState {
    pub fn empty() -> BoardState {
        BoardState {
            cells: [ALL_DIGITS; CELLS],
            cells_remaining: CELLS,
            solved_in_house: [NO_DIGITS; HOUSES],
            solution: [0; CELLS],
        }
    }
}

#[derive(Copy, Clone)]
struct Placement {
    cell: Cell,
    mask: DigitMask,
}

#[derive(Copy, Clone)]
struct Guess {
    cell: Cell,
    remaining: DigitMask,
}

struct BruteForceSolver {
    solution_count: usize,
    invalid: bool,
    finished: bool,

    board: BoardState,
    board_stack: Vec<BoardState>,
    saved_solution: Option<Vec<usize>>,

    placement_queue: Vec<Placement>,
    guess_stack: Vec<Guess>,
}

impl BruteForceSolver {

    fn init_from_clues(clues: &[usize]) -> BruteForceSolver {
        let mut solver = BruteForceSolver {
            solution_count: 0,
            invalid: false,
            finished: false,
            board: BoardState::empty(),
            board_stack: Vec::new(),
            saved_solution: None,
            placement_queue: Vec::new(),
            guess_stack: Vec::new(),
        };

        for (cell, &clue) in clues.iter().enumerate() {
            if clue != 0 {
                solver.enqueue_placement(cell, MASK_FOR_DIGIT[clue]);
            }
        }

        solver
    }

    fn count_solutions(&mut self, max_solutions: usize) -> BruteForceResult {
        while !self.finished {
            while !self.placement_queue.is_empty() { self.process_queue(); }
            if self.board.cells_remaining > 0 && !self.invalid {
                self.check_hidden_singles();
                if self.placement_queue.is_empty() { self.guess(); }
            }
            if self.invalid { self.backtrack(); }
            else if self.board.cells_remaining == 0 {
                self.solution_count += 1;
                if self.solution_count == 1 { self.saved_solution = Some(self.board.solution.to_vec()); }
                if self.solution_count >= max_solutions { break; }
                self.backtrack();
            }
        }

        BruteForceResult {
            solution_count: self.solution_count,
            solution: self.saved_solution.clone(),
        }
    }

    fn process_queue(&mut self) {
        if self.placement_queue.len() < 4 {
            while !self.placement_queue.is_empty() {
                let placement = self.placement_queue.pop().unwrap();
                self.place(placement);
                for &neighbour in NEIGHBOURS_FOR_CELL[placement.cell] {
                    if self.board.cells[neighbour] & placement.mask != NO_DIGITS {
                        self.board.cells[neighbour] ^= placement.mask;
                        let neighbour_mask = self.board.cells[neighbour];
                        let remaining = DIGITS_IN_MASK[neighbour_mask];
                        if remaining == 1 { self.enqueue_placement(neighbour, neighbour_mask); }
                        else if remaining == 0 { self.invalid = true; return; }
                    }
                }
            }
        } else {
            while !self.placement_queue.is_empty() {
                let placement = self.placement_queue.pop().unwrap();
                self.place(placement);
            }
            for cell in 0..CELLS {
                let mask = self.board.solved_in_house[ROW_FOR_CELL[cell]] |
                           self.board.solved_in_house[COL_FOR_CELL[cell]] |
                           self.board.solved_in_house[BLOCK_FOR_CELL[cell]];
                if self.board.cells[cell] & mask != NO_DIGITS {
                    self.board.cells[cell] &= !mask;
                    let neighbour_mask = self.board.cells[cell];
                    let remaining = DIGITS_IN_MASK[neighbour_mask];
                    if remaining == 1 { self.enqueue_placement(cell, neighbour_mask); }
                    else if remaining == 0 { self.invalid = true; return; }
                }
            }
        }
    }

    fn check_hidden_singles(&mut self) {
        for house in 0..HOUSES {
            let (mut at_least_once, mut more_than_once) = (NO_DIGITS, NO_DIGITS);


            for idx in 0..DIGITS {
                let mask = self.board.cells[CELLS_FOR_HOUSE[house][idx]];
                more_than_once |= at_least_once & mask;
                at_least_once |= mask;
            }

            if at_least_once | self.board.solved_in_house[house] != ALL_DIGITS {
                self.invalid = true;
                return;
            }

            let mut exactly_once = at_least_once & !more_than_once;
            if exactly_once != NO_DIGITS {
                for idx in 0..DIGITS {
                    let cell = CELLS_FOR_HOUSE[house][idx];
                    let mask = self.board.cells[cell] & exactly_once;
                    if mask != NO_DIGITS {
                        if DIGITS_IN_MASK[mask] > 1 {
                            self.invalid = true;
                            return;
                        }
                        self.enqueue_placement(cell, mask);
                        exactly_once ^= mask; if exactly_once == NO_DIGITS { break; }
                    }
                }
            }
        }
    }

    fn guess(&mut self) {
        let (mut best_cell, mut best_digits) = (0, DIGITS + 1);
        for cell in 0..CELLS {
            let digits = DIGITS_IN_MASK[self.board.cells[cell]];
            if digits > 1 && digits < best_digits {
                best_cell = cell;
                best_digits = digits;
                if digits == 2 { break; }
            }
        }
        if best_digits == DIGITS + 1 { self.invalid = true; return; }

        let best_mask = self.board.cells[best_cell];
        let leftovers = best_mask & best_mask.wrapping_sub(1);
        let guess_mask = best_mask ^ leftovers;
        let guess = Guess{ cell: best_cell, remaining: leftovers };

        self.board_stack.push(self.board.clone());
        self.guess_stack.push(guess);
        self.enqueue_placement(best_cell, guess_mask);
    }

    fn backtrack(&mut self) {
        if !self.board_stack.is_empty() {
            self.board = self.board_stack.pop().unwrap().clone();
            let guess = self.guess_stack.pop().unwrap();
            if DIGITS_IN_MASK[guess.remaining] > 1 {
                self.board.cells[guess.cell] = guess.remaining;
            } else {
                self.enqueue_placement(guess.cell, guess.remaining);
            }
            self.invalid = false;
        } else {
            self.finished = true;
        }
    }

    fn enqueue_placement(&mut self, cell: Cell, mask: DigitMask) {
        self.placement_queue.push(Placement{ cell, mask })
    }

    fn place(&mut self, placement: Placement) {
        if self.board.cells[placement.cell] != NO_DIGITS {
            let mask = placement.mask;
            if self.board.cells[placement.cell] & mask == NO_DIGITS ||
               self.board.solved_in_house[ROW_FOR_CELL[placement.cell]] & mask != NO_DIGITS ||
               self.board.solved_in_house[COL_FOR_CELL[placement.cell]] & mask != NO_DIGITS ||
               self.board.solved_in_house[BLOCK_FOR_CELL[placement.cell]] & mask != NO_DIGITS {
                self.invalid = true;
                return;
            }

            self.board.cells[placement.cell] = NO_DIGITS;
            self.board.solution[placement.cell] = UNIQ_DIGIT_IN_MASK[mask];
            self.board.solved_in_house[ROW_FOR_CELL[placement.cell]] |= mask;
            self.board.solved_in_house[COL_FOR_CELL[placement.cell]] |= mask;
            self.board.solved_in_house[BLOCK_FOR_CELL[placement.cell]] |= mask;
            self.board.cells_remaining -= 1;
        }
    }
}
