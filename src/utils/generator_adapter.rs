//! An adapter that allows a generator to be used as an iterator.

use std::marker::Unpin;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub struct GeneratorAdapter<G: Generator<Return = ()> + Unpin>(G);

impl <G: Generator<Return = ()> + Unpin> GeneratorAdapter<G> {
    pub fn of(generator: G) -> GeneratorAdapter<G> {
        GeneratorAdapter(generator)
    }
}

impl <G: Generator<Return = ()> + Unpin> Iterator for GeneratorAdapter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}
