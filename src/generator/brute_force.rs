//! A quick brute-force solver, used to aid in grid generation.

use grid::{CellIdx, Candidate, Grid, GridSize};
use grid::candidateset::CandidateSet;
use grid::cell::Cell;

pub fn has_unique_solution<T: GridSize>(empty_grid: &Grid<T>, clues: &[usize]) -> bool {
    let mut solver = BruteForceSolver::init_from_empty_grid_and_clues(empty_grid, clues);
    solver.run(2);
    solver.solution_count == 1
}

pub fn count_solutions<T: GridSize>(empty_grid: &Grid<T>, clues: &[usize]) -> usize {
    let mut solver = BruteForceSolver::init_from_empty_grid_and_clues(empty_grid, clues);
    solver.run(usize::max_value());
    solver.solution_count
}

#[derive(Clone)]
struct BoardState<T: GridSize> {
    cells: Vec<Cell<T>>,
    cells_remaining: usize,
}

impl <T: GridSize> BoardState<T> {
    pub fn empty() -> BoardState<T> {
        let num_cells = T::size() * T::size();
        BoardState {
            cells: vec![Cell::empty(); num_cells],
            cells_remaining: num_cells,
        }
    }
}

#[derive(Copy, Clone)]
struct Placement {
    cell: CellIdx,
    value: Candidate,
}

#[derive(Copy, Clone)]
struct Guess {
    cell: CellIdx,
    value: Candidate,
}

struct BruteForceSolver<T: GridSize> {
    invalid: bool,
    finished: bool,

    regions: Vec<Vec<usize>>,
    neighbours: Vec<Vec<usize>>,

    board: BoardState<T>,
    board_stack: Vec<BoardState<T>>,
    solution_count: usize,

    placement_queue: Vec<Placement>,
    guess_stack: Vec<Guess>,
}

impl <T: GridSize> BruteForceSolver<T> {

    fn init_from_empty_grid_and_clues(grid: &Grid<T>, clues: &[usize]) -> BruteForceSolver<T> {
        let mut solver = BruteForceSolver {
            invalid: false,
            finished: false,
            regions: grid.all_regions().iter().map(|r| r.iter().collect()).collect(),
            neighbours: grid.cells().iter().map(|c| grid.neighbours(c).iter().collect()).collect(),
            board: BoardState::empty(),
            board_stack: Vec::new(),
            solution_count: 0,
            placement_queue: Vec::new(),
            guess_stack: Vec::new(),
        };

        for (cell, &clue) in clues.iter().enumerate() {
            if clue != 0 {
                solver.enqueue_placement(cell, clue);
            }
        }

        solver
    }

    fn run(&mut self, max_solutions: usize) {
        while !self.finished {
            while !self.placement_queue.is_empty() { self.process_queue(); }
            if self.board.cells_remaining > 0 && !self.invalid {
                self.check_hidden_singles();
                if self.placement_queue.is_empty() { self.guess(); }
            }
            if self.invalid { self.backtrack(); }
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
            for neighbour_idx_idx in 0..self.neighbours[placement.cell].len() {
                let neighbour_idx = self.neighbours[placement.cell][neighbour_idx_idx];
                if self.board.cells[neighbour_idx].has_candidate(placement.value) {
                    self.board.cells[neighbour_idx].remove_candidate(placement.value);
                    let remaining = self.board.cells[neighbour_idx].num_candidates();
                    if remaining == 1 { self.enqueue_placement(neighbour_idx, self.board.cells[neighbour_idx].first_candidate().unwrap()); }
                    else if remaining == 0 { self.invalid = true; return; }
                }
            }
        }
    }

    fn check_hidden_singles(&mut self) {

        for region_idx in 0..self.regions.len() {

            let (mut solved, mut at_least_once, mut more_than_once) = (CandidateSet::empty(), CandidateSet::empty(), CandidateSet::empty());

            for idx in 0..T::size() {
                let candidates = self.board.cells[self.regions[region_idx][idx]].candidates();
                more_than_once |= at_least_once & candidates;
                at_least_once |= candidates;
                if candidates.is_empty() { solved.add_candidate(self.board.cells[self.regions[region_idx][idx]].first_candidate().unwrap()); }
            }

            if at_least_once | solved != CandidateSet::full() {
                self.invalid = true;
                return;
            }

            let mut exactly_once = at_least_once & !more_than_once;
            if !exactly_once.is_empty() {
                for idx in 0..T::size() {
                    let cell = self.regions[region_idx][idx];
                    let candidates = self.board.cells[cell].candidates() & exactly_once;
                    if !candidates.is_empty() {
                        if candidates.len() > 1 {
                            self.invalid = true;
                            return;
                        }
                        self.enqueue_placement(cell, candidates.first().unwrap());
                        exactly_once ^= candidates; if exactly_once.is_empty() { break; }
                    }
                }
            }
        }
    }

    fn get_best_cell_to_guess(&mut self) -> Option<CellIdx> {
        let (mut best_cell_idx, mut best_digits) = (0, T::size() + 1);
        for (idx, cell) in self.board.cells.iter().enumerate() {
            let digits = cell.num_candidates();
            if digits > 1 && digits < best_digits {
                best_cell_idx = idx;
                best_digits = digits;
                if digits == 2 { break; }
            }
        }
        if best_digits == T::size() + 1 { None } else { Some(best_cell_idx) }
    }

    fn get_guess_for_cell(&mut self, cell_idx: CellIdx) -> Guess {
        let cell = self.board.cells[cell_idx];
        let guess_value = cell.first_candidate().unwrap();
        Guess { cell: cell_idx, value: guess_value }
    }

    fn guess(&mut self) {
        if let Some(best_cell) = self.get_best_cell_to_guess() {
            let guess = self.get_guess_for_cell(best_cell);
            self.board_stack.push(self.board.clone());
            self.guess_stack.push(guess);
            self.enqueue_placement(best_cell, guess.value);
        } else {
            self.invalid = true;
        }
    }

    fn backtrack(&mut self) {
        if !self.board_stack.is_empty() {

            self.board = self.board_stack.pop().unwrap().clone();
            self.placement_queue.clear();

            let guess = self.guess_stack.pop().unwrap();
            self.board.cells[guess.cell].remove_candidate(guess.value);
            if self.board.cells[guess.cell].num_candidates() == 1 {
                self.enqueue_placement(guess.cell, guess.value);
            }

            self.invalid = false;

        } else {
            self.finished = true;
        }
    }

    fn enqueue_placement(&mut self, cell: CellIdx, value: Candidate) {
        self.placement_queue.push(Placement{ cell, value })
    }

    fn place(&mut self, placement: Placement) {
        if self.board.cells[placement.cell].num_candidates() > 0 {

            let value = placement.value;
            if !self.board.cells[placement.cell].has_candidate(value) {
                self.invalid = true;
                return;
            }

            self.board.cells[placement.cell].set_value(value);
            self.board.cells_remaining -= 1;
        }
    }
}
