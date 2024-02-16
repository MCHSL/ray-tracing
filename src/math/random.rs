use std::ops::Range;

use rand::distributions::{Distribution, Uniform};

use super::Interval;

pub fn random_range(range: Interval) -> f32 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(range);
    uniform.sample(&mut rng)
}

pub fn random() -> f32 {
    random_range(0.0..1.0)
}

pub fn random_int(range: Range<usize>) -> usize {
    random_range(range.start as f32..range.end as f32).floor() as usize
}
