//! An adapter that allows a generator to be used as an iterator.

use std::ops::{Generator, GeneratorState};

pub struct GeneratorAdapter<G: Generator<Return = ()>>(G);

impl <G: Generator<Return = ()>> GeneratorAdapter<G> {
    pub fn of(generator: G) -> GeneratorAdapter<G> {
        GeneratorAdapter(generator)
    }
}

impl <G: Generator<Return = ()>> Iterator for GeneratorAdapter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.0.resume() } {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}
