use integer_sqrt::IntegerSquareRoot;

pub trait PrimeGenerator {
    fn get_primes(&mut self, min: u64, max: u64) -> Vec<u64>;
}

pub trait EphemeralPrimeGenerator {
    fn get_primes(&self, min: u64, max: u64) -> Vec<u64>;
}

impl PrimeGenerator for EphemeralPrimeGenerator {
    fn get_primes(&mut self, min: u64, max: u64) -> Vec<u64> {
        EphemeralPrimeGenerator::get_primes(self, min, max)
    }
}

pub struct EphemeralSieveOfEratosthenes;

impl EphemeralPrimeGenerator for EphemeralSieveOfEratosthenes {
    fn get_primes(&self, min: u64, max: u64) -> Vec<u64> {
        let max_factor = max.integer_sqrt();
        let candidates: Vec<u64> = (min..=max).iter().collect();
        let eliminated: Vec<u64> = vec![false; candidates.len()];

        for (i, candidate) in candidate.iter().enumerate() {
            for factor in 2..=max_factor.min(candidate - 1) {
                if candidate % factor == 0 {
                    eliminated[i] = true;
                    break;
                }
            }
        }
    }
}














trait PrimeGenerator {}

#[bench]
fn bench_first_thousand(b: &mut Bencher) {
    b.iter(|| {
        black_box(primes_to(1000).last());
    })
}

#[bench]
fn bench_first_hundred_thousand(b: &mut Bencher) {
    b.iter(|| {
        black_box(primes_to(100000).last());
    })
}

// Sugar

fn all_primes() -> PrimeIterator {
    PrimeIterator::default()
}

fn primes_to(n: u64) -> impl Iterator<Item = u64> {
    all_primes().take_while(move |prime| *prime <= n)
}

/// An iterator over all primes.
#[derive(Default)]
struct PrimeIterator {
    /// A Sieve of Eratosthenes we use to produces batches of new primes.
    sieve: Eratosthenes,
    /// The current batch of primes we're yielding from.
    batch: Vec<u64>,
    /// The number of primes from this batch which have already been yielded.
    batch_used: usize,
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.batch_used >= self.batch.len() {
            self.batch = self.sieve.more();
            self.batch_used = 0;
        }

        let item = self.batch[self.batch_used];
        self.batch_used += 1;
        Some(item)
    }
}

// Substance

#[derive(Default, Debug)]
struct Eratosthenes {
    max_factor: u64,
    primes: BTreeSet<u64>,
}

impl Eratosthenes {
    /// Returns some new primes.
    fn more(&mut self) -> Vec<u64> {
        let mut more = Vec::new();
        while more.len() == 0 {
            more = self.test_more().cloned().collect();
        }
        more
    }

    /// Tests more candidates and returns an iterator over new verified primes.
    fn test_more(&mut self) -> btree_set::Range<u64> {
        if self.max_factor < 32 {
            self.test_to_factor(32)
        } else {
            // grow by 25%
            let factor = self.max_factor + (self.max_factor >> 2);
            self.test_to_factor(factor)
        }
    }

    /// Tests candidates up to factor squared and returns an iterator over new verified primes.
    fn test_to_factor(&mut self, factor: u64) -> btree_set::Range<u64> {
        let min_new_candidate = if self.max_factor > 0 {
            self.max_factor * self.max_factor + 1
        } else {
            2
        };
        let max_candidate = factor * factor;

        for candidate in min_new_candidate..max_candidate {
            self.primes.insert(candidate);
        }

        let factors: Vec<u64> = self.primes.range(2..=factor).cloned().collect();
        for factor in factors {
            let mut candidates_to_remove = Vec::<u64>::new();

            // Could we use Rayon to split the prime candidates between threads?
            // I think yes if we didn't use range(), but if we drop range() we need
            // a growth factor like 400% instead of 25% to make up for the extra work.
            for candidate in self.primes.range(min_new_candidate..=u64::max_value()) {
                if candidate % factor == 0 {
                    candidates_to_remove.push(*candidate);
                }
            }

            for candidate in candidates_to_remove {
                self.primes.remove(&candidate);
            }
        }

        self.max_factor = factor;

        self.primes.range(min_new_candidate..=u64::max_value())
    }
}

















use std::{
    collections::{btree_set, BTreeSet},
    default::Default,
    fmt::Debug,
    ops::{Bound::*, Index, Range},
    thread::sleep,
    time::Duration,
};

trait PrimeGenerator {
    type Iterator: std::iter::Iterator<Item=u64> = PrimeIterator<'self,  Self>;

    fn up_to(&mut self, limit: u64) -> Vec<u64> {
        self.range(0..=limit)
    }

    fn range(&mut self, range: Range<usize>) -> Vec<u64> {
        let vec = Vec::new();

        let mut before_start = true;

        for prime in self.iter() {
            if before_start {
                if match range.start {
                    Included(n) => n <= prime,
                    Excluded(n) => n < prime,
                    Unbounded => true,
                } {
                    before_start = false;
                } else {
                    continue;
                }
            }

            if !match range.end {
                Included(n) => prime <= n,
                Excluded(n) => prime < n,
                Unbounded => true,
            } {
                break;
            }

            vec.push(prime);

            if let Included(limit) = range.end {
                if prime == limit {
                    break;
                }
            }
        }

        vec
    }

    fn iter(&'self self) -> self::Iterator {
        PrimeIterator::new(self)
    }
}

/// An iterator over all primes.
struct PrimeIterator<'generator, Generator>
where
    Generator: PrimeGenerator,
{
    generator: &'generator Generator,
    /// The current batch of primes we're yielding from.
    batch: Vec<u64>,
    /// The number of primes from this batch which have already been yielded.
    batch_used: usize,
}

impl<Generator> PrimeIterator<Generator> {
    fn new(generator: Generator) -> Self {
        Self {
            generator,
            batch: Vec::new(),
            batch_used: 0,
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.batch_used >= self.batch.len() {
            self.batch = self.sieve.more();
            self.batch_used = 0;
        }

        let item = self.batch[self.batch_used];
        self.batch_used += 1;
        Some(item)
    }
}

// Substance

#[derive(Default, Debug)]
struct Eratosthenes {
    max_factor: u64,
    primes: BTreeSet<u64>,
}

impl Eratosthenes {
    /// Returns some new primes.
    fn more(&mut self) -> Vec<u64> {
        let mut more = Vec::new();
        while more.len() == 0 {
            more = self.test_more().cloned().collect();
        }
        more
    }

    /// Tests more candidates and returns an iterator over new verified primes.
    fn test_more(&mut self) -> btree_set::Range<u64> {
        if self.max_factor < 32 {
            self.test_to_factor(32)
        } else {
            // grow by 25%
            let factor = self.max_factor + (self.max_factor >> 2);
            self.test_to_factor(factor)
        }
    }

    /// Tests candidates up to factor squared and returns an iterator over new verified primes.
    fn test_to_factor(&mut self, factor: u64) -> btree_set::Range<u64> {
        let min_new_candidate = if self.max_factor > 0 {
            self.max_factor * self.max_factor + 1
        } else {
            2
        };
        let max_candidate = factor * factor;

        for candidate in min_new_candidate..max_candidate {
            self.primes.insert(candidate);
        }

        let factors: Vec<u64> = self.primes.range(2..=factor).cloned().collect();
        for factor in factors {
            let mut candidates_to_remove = Vec::<u64>::new();

            // Could we use Rayon to split the prime candidates between threads?
            // I think yes if we didn't use range(), but if we drop range() we need
            // a growth factor like 400% instead of 25% to make up for the extra work.
            for candidate in self.primes.range(min_new_candidate..=u64::max_value()) {
                if candidate % factor == 0 {
                    candidates_to_remove.push(*candidate);
                }
            }

            for candidate in candidates_to_remove {
                self.primes.remove(&candidate);
            }
        }

        self.max_factor = factor;

        self.primes.range(min_new_candidate..=u64::max_value())
    }
}
