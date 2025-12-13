// [Revision: v0.2] [Path: libs/lib_pseudo_rng/src/lib.rs] [Date: 2025-12-13]
// Description: Deterministic pseudo-random number generation wrapper

use dep::oorandom::Rand32;

/// Generate a random float in range [min, max] using seed and iteration.
/// Deterministic: same seed + iter always produces same result.
pub fn get(min: f32, max: f32, seed: u32, iter: usize) -> f32 {
    let mut random = Rand32::new_inc(seed as u64, iter as u64);
    random.rand_float() * (max - min) + min
}

/// A stateful RNG wrapper for sequential random number generation.
pub struct Rng {
    rng: Rand32,
}

impl Rng {
    /// Create a new RNG with a seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rand32::new(seed),
        }
    }

    /// Create a new RNG with seed and increment.
    pub fn new_inc(seed: u64, inc: u64) -> Self {
        Self {
            rng: Rand32::new_inc(seed, inc),
        }
    }

    /// Roll a d10 (1-10).
    pub fn d10(&mut self) -> u8 {
        self.rng.rand_range(1..11) as u8
    }

    /// Roll a d100 (1-100).
    pub fn d100(&mut self) -> u8 {
        self.rng.rand_range(1..101) as u8
    }

    /// Roll a d6 (1-6).
    pub fn d6(&mut self) -> u8 {
        self.rng.rand_range(1..7) as u8
    }

    /// Generate a random float in [0.0, 1.0).
    pub fn rand_float(&mut self) -> f32 {
        self.rng.rand_float()
    }

    /// Generate a random integer in range [min, max).
    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        self.rng.rand_range(min..max)
    }
}

