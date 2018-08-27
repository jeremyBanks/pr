use std::{
    iter::FusedIterator,
    ops::{Bound::*, RangeBounds},
};

trait PrimeGenerator {
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
}

impl IntoIterator for PrimeGenerator {
    type Item = u64;
    type IntoIter = PrimeIterator;

    fn into_iter(self) -> PrimeIterator {
        PrimeIterator::new(self)
    }
}

struct PrimeIterator<Generator: PrimeGenerator> {
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
trait InfiniteIterator: Iterator {
    fn pop(&mut self) -> Self::Item {
        Iterator::next(self).expect("this InfiniteIterator was finite")
    }
}

impl<Item> FusedIterator for InfiniteIterator<Item = Item> {}

impl<Generator: PrimeGenerator> Iterator for PrimeIterator<Generator> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.index_in_buffer >= self.buffer.len() {
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
            self.index_in_buffer = 0;
        }

        let value = self.buffer[self.index_in_buffer];
        self.index_in_buffer += 1;
        value
    }
}

impl<Generator: PrimeGenerator> InfiniteIterator for PrimeIterator<Generator> {}
