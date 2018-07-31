#![feature(generators, generator_trait)]

extern crate ansi_term;
extern crate itertools;
extern crate rand;

pub mod analyser;
pub mod generator;
pub mod grid;
pub mod solver;
pub mod strategies;
mod utils;