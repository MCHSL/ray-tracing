use std::ops::Range;

use rand::distributions::{Distribution, Uniform};

pub fn random_range(range: Range<f32>) -> f32 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(range);
    uniform.sample(&mut rng)
}

pub fn random() -> f32 {
    random_range(0.0..1.0)
}
