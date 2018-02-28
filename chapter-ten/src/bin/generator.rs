#![feature(generators, generator_trait, conservative_impl_trait)]
use std::ops::{Generator, GeneratorState};

pub fn gen_to_iter<A, G: Generator<Return = (), Yield = A>>(gen: G) -> impl Iterator<Item = A> {
    GeneratorIter {
        state: GeneratorIterState::Pending,
        gen,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GeneratorIter<G> {
    state: GeneratorIterState,
    gen: G,
}

#[derive(Debug, PartialEq, Eq)]
enum GeneratorIterState {
    Pending,
    Empty,
}

impl<G: Generator<Return = ()>> Iterator for GeneratorIter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            GeneratorIterState::Empty => None,
            GeneratorIterState::Pending => match self.gen.resume() {
                GeneratorState::Yielded(value) => Some(value),
                GeneratorState::Complete(_) => {
                    self.state = GeneratorIterState::Empty;
                    None
                }
            },
        }
    }
}

fn main() {
    let mut generator = || {
        yield 1;
        return "foo";
    };

    match generator.resume() {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }
    match generator.resume() {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected value from resume"),
    }

    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);
}

fn fibonacci() -> impl Iterator<Item = u32> {
    gen_to_iter(move || {
        let mut curr = 0;
        let mut next = 1;
        loop {
            yield curr;
            let old = curr;
            curr = next;
            next += old;
        }
    })
}
