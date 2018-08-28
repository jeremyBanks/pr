use super::prelude::*;

#[derive(Debug)]
pub struct EratoOne {
    max_tested: u64,
    primes: Vec<u64>,
}

impl Default for EratoOne {
    fn default() -> EratoOne {
        EratoOne {
            max_tested: 1,
            primes: Vec::default(),
        }
    }
}

impl PrimeGenerator for EratoOne {
    fn generate(&mut self, min: u64, max: u64) -> Vec<u64> {
        if max > self.max_tested {
            let candidates = (self.max_tested + 1)..=max;

            let mut max_factor = 1;
            let mut max_factor_until = 0;
            'candidates: for candidate in candidates {
                while candidate > max_factor_until {
                    max_factor += 1;
                    max_factor_until = max_factor * max_factor;
                }
                let possible_factors = self.primes.iter().take_while(|&&n| n <= max_factor);
                for factor in possible_factors {
                    if candidate % factor == 0 {
                        continue 'candidates;
                    }
                }
                self.primes.push(candidate);
            }

            self.max_tested = max;
        }

        self.primes
            .iter()
            .skip_while(|&&p| p < min)
            .take_while(|&&p| p < max)
            .cloned()
            .collect()
    }
}
