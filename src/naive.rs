use super::prelude::*;

#[derive(Debug, Default)]
pub struct Naive;

impl PrimeGenerator for Naive {
    fn generate(&mut self, min: u64, max: u64) -> Vec<u64> {
        let mut vec = Vec::new();
        let candidates = (min.max(2))..=max;

        let mut max_factor = 0;
        let mut max_factor_through = 2;
        'candidates: for candidate in candidates {
            while candidate > max_factor_through {
                max_factor += 1;
                max_factor_through = max_factor * max_factor;
            }
            let possible_factors = (2..=max_factor).take_while(|&n| n <= max_factor);
            for factor in possible_factors {
                if candidate % factor == 0 {
                    continue 'candidates;
                }
            }
            vec.push(candidate);
        }

        vec
    }
}
