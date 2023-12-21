//! An adapter that allows a generator to be used as an iterator.

use std::marker::Unpin;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub struct GeneratorAdapter<G: Coroutine<Return = ()> + Unpin>(G);

impl <G: Coroutine<Return = ()> + Unpin> GeneratorAdapter<G> {
    pub fn of(generator: G) -> GeneratorAdapter<G> {
        GeneratorAdapter(generator)
    }
}

impl <G: Coroutine<Return = ()> + Unpin> Iterator for GeneratorAdapter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume(()) {
            CoroutineState::Yielded(x) => Some(x),
            CoroutineState::Complete(_) => None,
        }
    }
}
