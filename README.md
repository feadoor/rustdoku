# rustdoku
Logical Sudoku solver in Rust.

**NOTE**: I am not in the slightest bit interested in solving *every* Sudoku puzzle. What I am interested in doing here is in building up a repertoire of logical techniques and patterns that are capable of solving the vast majority of even very difficult puzzles.

Many of these techniques will be infeasible for human solvers, but human solving is not the goal here. My aim is to develop a small set of heuristics that can be used to find logical solutions to Sudoku puzzles which meet two goals:

* The solve should be linear - it can be followed step-by-step without having to remember previous steps.

* The solve must be verifiable by a human with sufficient understanding of the techniques involved.