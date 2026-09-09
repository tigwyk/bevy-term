//! The single game RNG, deterministic given a seed. Game code takes `&mut Rng`;
//! never `rand::rng()`.

use bevy::prelude::Resource;
use rand::rngs::ChaCha8Rng;
use rand::{RngExt as _, SeedableRng};

#[derive(Resource)]
pub struct Rng(ChaCha8Rng);

impl Rng {
    pub fn seeded(seed: u64) -> Self {
        Self(ChaCha8Rng::seed_from_u64(seed))
    }

    /// Uniform integer in `[0, n)`. `n` must be nonzero.
    pub fn below(&mut self, n: usize) -> usize {
        self.0.random_range(0..n)
    }

    /// Uniform integer in `[lo, hi]` inclusive.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        self.0.random_range(lo..=hi)
    }

    /// `percent` (0..=100) chance of `true`.
    pub fn chance(&mut self, percent: u8) -> bool {
        self.0.random_range(0..100) < percent
    }

    /// An index weighted by `weights`; `None` if empty or all zero.
    pub fn weighted(&mut self, weights: &[u32]) -> Option<usize> {
        let total: u32 = weights.iter().sum();
        if total == 0 {
            return None;
        }
        let mut roll = self.0.random_range(0..total);
        for (i, w) in weights.iter().enumerate() {
            if roll < *w {
                return Some(i);
            }
            roll -= *w;
        }
        None
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            Some(&items[self.below(items.len())])
        }
    }
}
