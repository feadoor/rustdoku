//! A (hopefully!) very quick brute-force solver, used to aid in grid generation.

use rand::prelude::*;

use grid::{Grid, GridSize};

type Cell = usize;
type House = usize;
type DigitMask = usize;

struct ConstantData {
    num_digits: usize,
    num_houses: usize,
    num_cells: usize,
    all_digits_mask: DigitMask,
    cells_for_house: Vec<Vec<Cell>>,
    houses_for_cell: Vec<Vec<House>>,
    mask_for_digit: Vec<DigitMask>,
    digits_in_mask: Vec<usize>,
    possible_guesses_for_mask: Vec<Vec<DigitMask>>,
    neighbours_for_cell: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct BoardState {
    cells: Vec<DigitMask>,
    cells_remaining: usize,
    solved_in_house: Vec<DigitMask>,
    solution: Vec<usize>,
}

impl BoardState {
    pub fn empty_for_grid<T: GridSize>(grid: &Grid<T>) -> BoardState {
        let (num_digits, num_cells) = (T::size(), T::size() * T::size());
        BoardState {
            cells: vec![(1 << num_digits) - 1; num_cells],
            cells_remaining: num_cells,
            solved_in_house: vec![0; grid.all_regions().len()],
            solution: vec![0; num_cells],
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
    mask: DigitMask,
    remaining: DigitMask,
}

pub struct BruteForceSolver {

    constants: ConstantData,

    invalid: bool,
    finished: bool,

    board: BoardState,
    board_stack: Vec<BoardState>,
    solution_count: usize,

    placement_queue: Vec<Placement>,
    guess_stack: Vec<Guess>,
}

impl BruteForceSolver {

    pub fn for_empty_grid<T: GridSize>(grid: &Grid<T>) -> BruteForceSolver {

        let constants = ConstantData {
            num_digits: Self::get_num_digits_from_grid(grid),
            num_houses: Self::get_num_houses_from_grid(grid),
            num_cells: Self::get_num_cells_from_grid(grid),
            all_digits_mask: Self::get_all_digits_mask_from_grid(grid),
            cells_for_house: Self::get_cells_for_house_from_grid(grid),
            houses_for_cell: Self::get_houses_for_cell_from_grid(grid),
            mask_for_digit: Self::get_mask_for_digit_from_grid(grid),
            digits_in_mask: Self::get_digits_in_mask_from_grid(grid),
            possible_guesses_for_mask: Self::get_possible_guesses_for_mask_from_grid(grid),
            neighbours_for_cell: Self::get_neighbours_for_cell_from_grid(grid),
        };

        BruteForceSolver {
            constants: constants,
            invalid: false,
            finished: false,
            board: BoardState::empty_for_grid(grid),
            board_stack: Vec::new(),
            solution_count: 0,
            placement_queue: Vec::new(),
            guess_stack: Vec::new(),
        }
    }

    pub fn has_unique_solution(&mut self, clues: &[usize]) -> bool {
        self.run(clues, 2);
        self.solution_count == 1
    }

    pub fn has_any_solution(&mut self, clues: &[usize]) -> bool {
        self.run(clues, 1);
        self.solution_count > 0
    }

    pub fn count_solutions(&mut self, clues: &[usize]) -> usize {
        self.run(clues, usize::max_value());
        self.solution_count
    }

    fn reset(&mut self) {
        self.invalid = false;
        self.finished = false;
        self.board = BoardState {
            cells: vec![self.constants.all_digits_mask; self.constants.num_cells],
            cells_remaining: self.constants.num_cells,
            solved_in_house: vec![0; self.constants.num_houses],
            solution: vec![0; self.constants.num_cells],
        };
        self.board_stack.clear();
        self.solution_count = 0;
        self.placement_queue.clear();
        self.guess_stack.clear();
    }

    fn prepare_with_clues(&mut self, clues: &[usize]) {
        self.reset();
        for (cell, &clue) in clues.iter().enumerate() {
            if clue != 0 {
                self.enqueue_placement(cell, self.constants.mask_for_digit[clue]);
            }
        }
    }

    fn run(&mut self, clues: &[usize], max_solutions: usize) {
        self.prepare_with_clues(clues);
        while !self.finished {
            while !self.placement_queue.is_empty() { self.process_queue(); }
            if self.board.cells_remaining > 0 && !self.invalid {
                self.check_hidden_singles();
                if self.placement_queue.is_empty() { self.guess(); }
            }
            else if self.invalid { self.backtrack(); }
            else if self.board.cells_remaining == 0 {
                self.solution_count += 1;
                if self.solution_count >= max_solutions { break; }
                self.backtrack();
            }
        }
    }

    fn process_queue(&mut self) {
        while !self.placement_queue.is_empty() {
            let placement = self.placement_queue.pop().unwrap();
            self.place(placement);
            for neighbour_idx in 0..self.constants.neighbours_for_cell[placement.cell].len() {
                let neighbour = self.constants.neighbours_for_cell[placement.cell][neighbour_idx];
                if self.board.cells[neighbour] & placement.mask != 0 {
                    self.board.cells[neighbour] ^= placement.mask;
                    let neighbour_mask = self.board.cells[neighbour];
                    let remaining = self.constants.digits_in_mask[neighbour_mask];
                    if remaining == 1 { self.enqueue_placement(neighbour, neighbour_mask); }
                    else if remaining == 0 { self.invalid = true; return; }
                }
            }
        }
    }

    fn check_hidden_singles(&mut self) {
        for house in 0..self.constants.num_houses {
            let (mut at_least_once, mut more_than_once) = (0, 0);

            for idx in 0..self.constants.num_digits {
                let mask = self.board.cells[self.constants.cells_for_house[house][idx]];
                more_than_once |= at_least_once & mask;
                at_least_once |= mask;
            }

            if at_least_once | self.board.solved_in_house[house] != self.constants.all_digits_mask {
                self.invalid = true;
                return;
            }

            let mut exactly_once = at_least_once & !more_than_once;
            if exactly_once != 0 {
                for idx in 0..self.constants.num_digits {
                    let cell = self.constants.cells_for_house[house][idx];
                    let mask = self.board.cells[cell] & exactly_once;
                    if mask != 0 {
                        if self.constants.digits_in_mask[mask] > 1 {
                            self.invalid = true;
                            return;
                        }
                        self.enqueue_placement(cell, mask);
                        exactly_once ^= mask; if exactly_once == 0 { break; }
                    }
                }
            }
        }
    }

    fn get_best_cell_to_guess(&mut self) -> Option<Cell> {
        let (mut best_cell, mut best_digits) = (0, self.constants.num_digits + 1);
        for cell in 0..self.constants.num_cells {
            let digits = self.constants.digits_in_mask[self.board.cells[cell]];
            if digits > 1 && digits < best_digits {
                best_cell = cell; best_digits = digits;
                if digits == 2 { break; }
            }
        }
        if best_digits == self.constants.num_digits + 1 { None } else { Some(best_cell) }
    }

    fn get_guess_for_cell(&mut self, cell: Cell) -> Guess {
        let cell_mask = self.board.cells[cell];
        let guess_mask = *thread_rng().choose(&self.constants.possible_guesses_for_mask[cell_mask]).unwrap();
        let leftovers = cell_mask ^ guess_mask;
        Guess { cell: cell, mask: guess_mask, remaining: leftovers }
    }

    fn guess(&mut self) {
        if let Some(best_cell) = self.get_best_cell_to_guess() {
            let guess = self.get_guess_for_cell(best_cell);
            self.board_stack.push(self.board.clone());
            self.guess_stack.push(guess);
            self.enqueue_placement(best_cell, guess.mask);
        } else {
            self.invalid = true;
        }
    }

    fn backtrack(&mut self) {
        if !self.board_stack.is_empty() {
            self.board = self.board_stack.pop().unwrap().clone();
            self.placement_queue.clear();
            let guess = self.guess_stack.pop().unwrap();
            if self.constants.digits_in_mask[guess.remaining] > 1 {
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
        self.placement_queue.push(Placement { cell, mask });
    }

    fn place(&mut self, placement: Placement) {
        if self.board.cells[placement.cell] != 0 {

            let mask = placement.mask;
            if self.board.cells[placement.cell] & mask == 0 {
                self.invalid = true;
                return;
            }

            self.board.cells[placement.cell] = 0;
            for &house in &self.constants.houses_for_cell[placement.cell] {
                self.board.solved_in_house[house] |= mask;
            }
            self.board.solution[placement.cell] = placement.mask;
            self.board.cells_remaining -= 1;
        } else if self.board.solution[placement.cell] != placement.mask {
            self.invalid = true;
        }
    }

    fn get_num_digits_from_grid<T: GridSize>(_grid: &Grid<T>) -> usize {
        T::size()
    }

    fn get_num_houses_from_grid<T: GridSize>(grid: &Grid<T>) -> usize {
        grid.all_regions().len()
    }

    fn get_num_cells_from_grid<T: GridSize>(_grid: &Grid<T>) -> usize {
        T::size() * T::size()
    }

    fn get_all_digits_mask_from_grid<T: GridSize>(_grid: &Grid<T>) -> DigitMask {
        (1 << T::size()) - 1
    }

    fn get_cells_for_house_from_grid<T: GridSize>(grid: &Grid<T>) -> Vec<Vec<Cell>> {
        grid.all_regions().iter()
            .map(|region| region.iter().collect())
            .collect()
    }

    fn get_houses_for_cell_from_grid<T: GridSize>(grid: &Grid<T>) -> Vec<Vec<House>> {
        let mut houses_for_cell = vec![vec![]; T::size() * T::size()];
        for (idx, house) in grid.all_regions().iter().enumerate() {
            for cell in house.iter() {
                houses_for_cell[cell].push(idx);
            }
        }
        houses_for_cell
    }

    fn get_mask_for_digit_from_grid<T: GridSize>(_grid: &Grid<T>) -> Vec<DigitMask> {
        (0..T::size() + 1).map(|digit| if digit == 0 { 0 } else { 1 << (digit - 1) }).collect()
    }

    fn get_digits_in_mask_from_grid<T: GridSize>(_grid: &Grid<T>) -> Vec<usize> {
        (0..(1 << T::size())).map(|mask: usize| mask.count_ones() as usize).collect()
    }

    fn get_possible_guesses_for_mask_from_grid<T: GridSize>(grid: &Grid<T>) -> Vec<Vec<DigitMask>> {
        let mask_for_digit = Self::get_mask_for_digit_from_grid(grid);
        (0..(1 << T::size())).map(|mask| (0..T::size()).filter(|v| mask & (1 << v) != 0).map(|v| mask_for_digit[v + 1]).collect()).collect()
    }

    fn get_neighbours_for_cell_from_grid<T: GridSize>(grid: &Grid<T>) -> Vec<Vec<Cell>> {
        (0..T::size() * T::size()).map(|cell| grid.neighbours(cell).iter().collect()).collect()
    }

}