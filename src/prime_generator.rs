use std::{
    iter::FusedIterator,
    ops::{Bound::*, RangeBounds},
};

pub mod prelude {
    pub use super::InfiniteIterator;
    pub use super::PrimeGenerator;
}

pub trait PrimeGenerator: Sized {
    fn generate(&mut self, min: u64, max: u64) -> Vec<u64>;

    fn range(&mut self, range: impl RangeBounds<u64>) -> Vec<u64> {
        let min = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };

        let max = match range.end_bound() {
            Included(&n) => n,
            Excluded(&n) => n - 1,
            Unbounded => panic!("PrimeGenerator can't give you infinite primes because that would take too much memory. You may want to look at PrimeIterator."),
        };

        self.generate(min, max)
    }

    // TODO: move into IntoIterator trait
    fn into_iter(self) -> PrimeIterator<Self> {
        PrimeIterator::new(self)
    }
}

#[derive(Debug)]
pub struct PrimeIterator<Generator: PrimeGenerator> {
    generator: Generator,
    buffer: Vec<u64>,
    buffer_min: u64,
    buffer_max: u64,
    buffer_index: usize,
}

impl<Generator: PrimeGenerator> PrimeIterator<Generator> {
    pub fn new(generator: Generator) -> PrimeIterator<Generator> {
        PrimeIterator {
            generator,
            buffer: Vec::new(),
            buffer_min: 0,
            buffer_max: 0,
            buffer_index: 0,
        }
    }
}

/// Marker trait for infinite iterators
pub trait InfiniteIterator: Iterator {
    fn pop(&mut self) -> Self::Item {
        Iterator::next(self).expect("this InfiniteIterator was finite")
    }
}

impl<Item> FusedIterator for InfiniteIterator<Item = Item> {}

impl<Generator: PrimeGenerator> Iterator for PrimeIterator<Generator> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.buffer_index >= self.buffer.len() {
            let old_range_size = self.buffer_max - self.buffer_min;
            let new_range_size = if old_range_size == 0 {
                1024
            } else if old_range_size >= 1048576 {
                1048576
            } else {
                old_range_size * 2
            };

            self.buffer_min = self.buffer_max + 1;
            self.buffer_max = self.buffer_min + new_range_size - 1;
            self.buffer = self.generator.range(self.buffer_min..=self.buffer_max);
            self.buffer_index = 0;
        }

        let value = self.buffer[self.buffer_index];
        self.buffer_index += 1;
        Some(value)
    }
}

impl<Generator: PrimeGenerator> InfiniteIterator for PrimeIterator<Generator> {}
