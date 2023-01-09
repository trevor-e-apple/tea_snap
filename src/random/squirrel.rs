use std::num::Wrapping;

use crate::random::rng::Rng;

const NOISE1: u32 = 0xB5297A4D;
const NOISE2: u32 = 0x68E31dA4;
const NOISE3: u32 = 0x1B56C4E9;

/// Based on Squirrel Eiserloh's noise generator
pub struct SquirrelRandom {
    n: Wrapping<u32>,
    seed: u32,
}

impl Rng for SquirrelRandom {
    fn seed(seed: u32) -> Self {
        Self { n: Wrapping(0), seed: seed }
    }

    fn rand(&mut self) -> u32 {
        let mut n = self.n;
        n *= NOISE1;
        n += self.seed;
        n ^= n >> 8;
        n += NOISE2;
        n ^= n << 8;
        n *= NOISE3;
        n ^= n >> 8;

        self.n += 1;
        // return as a plain u32
        n.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instance(seed: u32) -> SquirrelRandom {
        SquirrelRandom::seed(seed)
    }

    include!("rng_tests.incl.rs");
}
